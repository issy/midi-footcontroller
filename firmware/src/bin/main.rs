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
use embassy_embedded_hal::shared_bus::blocking::spi::SpiDevice;
use embassy_executor::{Spawner, task};
use embassy_sync::{
    blocking_mutex::{Mutex, raw::NoopRawMutex},
    channel::Channel,
};
use embassy_time::Delay;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::mono_font::ascii::FONT_10X20;
use embedded_graphics::prelude::*;
use embedded_graphics::{pixelcolor::Rgb565, text::Text};
use esp_hal::Async;
use esp_hal::clock::CpuClock;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::spi::master::Spi;
use esp_hal::time::Rate;
use esp_hal::timer::timg::TimerGroup;
use esp_hal::uart::{Config as UartConfig, DataBits, Parity, StopBits, Uart, UartRx, UartTx};
use firmware::midi::{MidiPacket, MidiParser};
use log::info;
use mipidsi::models::ST7789;
use mipidsi::options::Orientation;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

const MIDI_OUT_CHANNEL: Channel<NoopRawMutex, MidiPacket, 128> = Channel::new();

// Forward MIDI messages IN to the MIDI_OUT_CHANNEL
#[task]
async fn midi_thru_task(mut uart: UartRx<'static, Async>) {
    let mut parser = MidiParser::new();
    let mut buf = [0u8; 1];

    loop {
        uart.read_async(&mut buf).await.unwrap();

        if let Some(packet) = parser.feed(buf[0]) {
            MIDI_OUT_CHANNEL.send(packet).await;
        }
    }
}

// Read MIDI messages from the MIDI_OUT_CHANNEL and send them out over UART
#[task]
async fn midi_out_task(mut uart: UartTx<'static, Async>) {
    loop {
        let packet = MIDI_OUT_CHANNEL.receive().await;
        uart.write_async(&packet.data[..packet.len as usize])
            .await
            .unwrap();
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
        esp_hal::spi::master::Config::default().with_frequency(Rate::from_khz(40)),
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
        .display_size(240, 240)
        .orientation(Orientation::default())
        .init(&mut Delay)
        .expect("Failed to initialise ST7789 display");

    display.clear(Rgb565::BLACK).unwrap();
    let style = MonoTextStyle::new(&FONT_10X20, Rgb565::GREEN);
    Text::new("Hello, world!", Point::new(10, 30), style)
        .draw(&mut display)
        .unwrap();

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
    let (rx, tx) = uart.split();

    spawner
        .spawn(midi_thru_task(rx))
        .expect("Unable to spawn MIDI thru task");
    info!("MIDI thru task spawned");
    spawner
        .spawn(midi_out_task(tx))
        .expect("Unable to spawn MIDI out task");
    info!("MIDI out task spawned");

    info!("Startup complete.");

    loop {}
}
