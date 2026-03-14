#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

#[allow(
    unused_imports,
    reason = "esp-alloc is required for heap allocation to work."
)]
use esp_alloc as _;
#[allow(
    unused_imports,
    reason = "esp-backtrace is required for backtraces to work."
)]
use esp_backtrace as _;

use core::cell::RefCell;
use core::ops::Add;
use core::str::FromStr;
use embassy_embedded_hal::shared_bus::blocking::spi::SpiDevice;
use embassy_executor::{Spawner, task};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::{
    blocking_mutex::{Mutex, raw::NoopRawMutex},
    channel::Channel,
};
use embassy_time::Delay;
use embassy_time::Timer;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::mono_font::{MonoTextStyleBuilder, ascii::FONT_10X20};
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::PrimitiveStyleBuilder;
use embedded_graphics::text::{Alignment, Baseline, TextStyleBuilder};
use embedded_graphics::{pixelcolor::Rgb565, text::Text};
use esp_hal::Async;
use esp_hal::clock::CpuClock;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::spi::master::Spi;
use esp_hal::time::Rate;
use esp_hal::timer::timg::TimerGroup;
use esp_hal::uart::{
    Config as UartConfig, DataBits, Parity, RxError, StopBits, TxError, Uart, UartRx, UartTx,
};
use esp_println::println;
use foundation::layout::DisplayLayout;
use foundation::storage::StorageManager;
use foundation::{
    application::state::ApplicationBuilder,
    midi::{MidiPacket, MidiParser, MidiReader, MidiWriter},
};
use heapless::String;
use log::info;
use mipidsi::models::ST7789;
use mipidsi::options::Rotation::Deg270;
use mipidsi::options::{ColorInversion, Orientation};

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

static MIDI_OUT_CHANNEL: Channel<CriticalSectionRawMutex, MidiPacket, 128> = Channel::new();

struct UartMidiReader<'a, 'b> {
    uart: &'a mut UartRx<'b, Async>,
    parser: MidiParser,
}

impl<'a, 'b> UartMidiReader<'a, 'b> {
    fn new(uart: &'a mut UartRx<'b, Async>) -> Self {
        Self {
            uart,
            parser: MidiParser::default(),
        }
    }
}

impl<'a, 'b> MidiReader for UartMidiReader<'a, 'b> {
    type Error = RxError;

    async fn read_midi_packet(&mut self) -> Result<Option<MidiPacket>, Self::Error> {
        let mut buf = [0u8; 1];
        self.uart.read_async(&mut buf).await?;

        Ok(self.parser.feed(buf[0]))
    }
}

struct UartMidiWriter<'a, 'b> {
    uart: &'a mut UartTx<'b, Async>,
}

impl<'a, 'b> UartMidiWriter<'a, 'b> {
    fn new(uart: &'a mut UartTx<'b, Async>) -> Self {
        Self { uart }
    }
}

impl<'a, 'b> MidiWriter for UartMidiWriter<'a, 'b> {
    type Error = TxError;

    async fn write_midi_packet(&mut self, packet: &MidiPacket) -> Result<(), Self::Error> {
        self.uart
            .write_async(&packet.data[..packet.len as usize])
            .await?;
        Ok(())
    }
}

#[derive(Default)]
struct FakeStorageManager<'a> {}

impl<'a> StorageManager for FakeStorageManager<'a> {
    fn load_presets(
        &self,
    ) -> heapless::Vec<
        foundation::storage::state::StoredPreset,
        { foundation::storage::state::MAX_PRESETS },
    > {
        heapless::Vec::new()
    }

    fn save_presets(
        &mut self,
        _presets: &heapless::Vec<
            foundation::storage::state::StoredPreset,
            foundation::storage::state::MAX_PRESETS,
        >,
    ) {
        // Do nothing
    }
}

/// Forward MIDI messages IN to the MIDI_OUT_CHANNEL
#[task]
async fn midi_thru_task(mut reader: UartMidiReader<'static, 'static>) {
    loop {
        if let Some(packet) = reader.read_midi_packet().await.unwrap() {
            MIDI_OUT_CHANNEL.send(packet).await;
        }
    }
}

/// Read MIDI messages from the MIDI_OUT_CHANNEL and send them out over UART
#[task]
async fn midi_out_task(mut writer: UartMidiWriter<'static, 'static>) {
    loop {
        let packet = MIDI_OUT_CHANNEL.receive().await;
        let res: Result<(), TxError> = writer.write_midi_packet(&packet).await;
        res.unwrap();
    }
}

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    esp_println::logger::init_logger_from_env();

    let peripherals = esp_hal::init(esp_hal::Config::default().with_cpu_clock(CpuClock::max()));

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let sw_interrupt =
        esp_hal::interrupt::software::SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    esp_rtos::start(timg0.timer0, sw_interrupt.software_interrupt0);

    info!("Embassy initialized!");

    let spi = Spi::new(
        peripherals.SPI2,
        esp_hal::spi::master::Config::default().with_frequency(Rate::from_mhz(40)),
    )
    .expect("Failed to initialise SPI2 peripheral")
    .with_sck(peripherals.GPIO18)
    .with_mosi(peripherals.GPIO19)
    .with_cs(peripherals.GPIO20);

    let cs = Output::new(peripherals.GPIO5, Level::High, OutputConfig::default());
    let spi_bus: Mutex<NoopRawMutex, _> = Mutex::new(RefCell::new(spi));
    let spi_device = SpiDevice::new(&spi_bus, cs);

    let dc = Output::new(peripherals.GPIO21, Level::Low, OutputConfig::default());
    let di = display_interface_spi::SPIInterface::new(spi_device, dc);
    let mut display = mipidsi::Builder::new(ST7789, di)
        .display_size(240, 280)
        .orientation(Orientation::default().rotate(Deg270))
        .display_offset(0, 20)
        .invert_colors(ColorInversion::Inverted)
        // TODO: Add reset pin
        .init(&mut Delay)
        .expect("Failed to initialise ST7789 display");

    display.clear(Rgb565::BLACK).unwrap();

    embedded_graphics::primitives::Rectangle::new(
        Point::zero(),
        Size::new(display.size().width, display.size().height / 3),
    )
    .into_styled(
        PrimitiveStyleBuilder::new()
            .fill_color(Rgb565::CSS_ORANGE)
            .build(),
    )
    .draw(&mut display)
    .unwrap();
    embedded_graphics::primitives::Rectangle::new(
        Point::new(
            0,
            display.size().height as i32 - display.size().height as i32 / 3,
        ),
        Size::new(display.size().width, display.size().height / 3),
    )
    .into_styled(
        PrimitiveStyleBuilder::new()
            .fill_color(Rgb565::BLUE)
            .build(),
    )
    .draw(&mut display)
    .unwrap();

    let text_style = TextStyleBuilder::new()
        .alignment(Alignment::Center)
        .baseline(Baseline::Middle)
        .build();
    let character_style = MonoTextStyleBuilder::new()
        .font(&FONT_10X20)
        .text_color(Rgb565::GREEN)
        .build();
    Text::with_text_style(
        "Hello, world!",
        display
            .bounding_box()
            .top_left
            .add(Point::new(display.size().width as i32 / 2, 60)),
        character_style,
        text_style,
    )
    .draw(&mut display)
    .unwrap();

    let mut layout = DisplayLayout::new(&mut display);
    layout.set_top_text(String::from_str("Foo").unwrap());
    layout.set_bottom_text(String::from_str("Bar").unwrap());
    layout.draw().unwrap();

    let uart = Uart::new(
        peripherals.UART1,
        UartConfig::default()
            .with_baudrate(31_250)
            .with_data_bits(DataBits::_8)
            .with_parity(Parity::None)
            .with_stop_bits(StopBits::_1),
    )
    .expect("Failed to initialise UART1")
    .with_rx(peripherals.GPIO7)
    .with_tx(peripherals.GPIO8)
    .into_async();
    let (mut rx, mut tx) = uart.split();

    // spawner
    //     .spawn(midi_thru_task(rx))
    //     .expect("Unable to spawn MIDI thru task");
    // info!("MIDI thru task spawned");
    // spawner
    //     .spawn(midi_out_task(tx))
    //     .expect("Unable to spawn MIDI out task");
    // info!("MIDI out task spawned");

    info!("Startup complete.");

    let mut midi_reader = UartMidiReader::new(&mut rx);
    let mut midi_writer = UartMidiWriter::new(&mut tx);
    let mut storage_manager = FakeStorageManager::default();

    let app = ApplicationBuilder::new()
        .with_display(&mut display)
        .with_midi_reader(&mut midi_reader)
        .with_midi_writer(&mut midi_writer)
        .with_storage_manager(&mut storage_manager)
        .build();

    loop {
        Timer::after_secs(5).await;
        println!("Heartbeat");
    }
}
