#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

extern crate alloc;
mod midi;
mod storage;

include!(concat!(env!("OUT_DIR"), "/version.rs"));

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

use crate::storage::FakeStorageManager;

use core::cell::RefCell;
use display_interface_spi::SPIInterface;
use embassy_embedded_hal::shared_bus::blocking::spi::SpiDevice;
use embassy_executor::Spawner;
use embassy_sync::blocking_mutex::{Mutex, raw::NoopRawMutex};
use embassy_time::Delay;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use esp_hal::clock::CpuClock;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::spi::master::Spi;
use esp_hal::time::Rate;
use esp_hal::timer::timg::TimerGroup;
use esp_hal::uart::{Config as UartConfig, DataBits, Parity, StopBits, Uart, UartRx, UartTx};
use esp_hal::{Async, Blocking};
use foundation::application::state::{Application, ApplicationBuilder};
use log::info;
use midi::{UartMidiReader, UartMidiWriter};
use mipidsi::models::ST7789;
use mipidsi::options::Rotation::Deg270;
use mipidsi::options::{ColorInversion, Orientation};
use mipidsi::{Display, NoResetPin};
use static_cell::StaticCell;

type FirmwareApplication = Application<
    'static,
    Display<
        SPIInterface<
            SpiDevice<'static, NoopRawMutex, Spi<'static, Blocking>, Output<'static>>,
            Output<'static>,
        >,
        ST7789,
        NoResetPin,
    >,
    UartMidiReader<'static, 'static>,
    UartMidiWriter<'static, 'static>,
    FakeStorageManager,
>;

static APP: StaticCell<FirmwareApplication> = StaticCell::new();
static RX: StaticCell<UartRx<Async>> = StaticCell::new();
static TX: StaticCell<UartTx<Async>> = StaticCell::new();
static DISPLAY_1: StaticCell<
    Display<
        SPIInterface<SpiDevice<NoopRawMutex, Spi<Blocking>, Output>, Output>,
        ST7789,
        NoResetPin,
    >,
> = StaticCell::new();
static DISPLAY_2: StaticCell<
    Display<
        SPIInterface<SpiDevice<NoopRawMutex, Spi<Blocking>, Output>, Output>,
        ST7789,
        NoResetPin,
    >,
> = StaticCell::new();
static DISPLAY_3: StaticCell<
    Display<
        SPIInterface<SpiDevice<NoopRawMutex, Spi<Blocking>, Output>, Output>,
        ST7789,
        NoResetPin,
    >,
> = StaticCell::new();
static DISPLAY_4: StaticCell<
    Display<
        SPIInterface<SpiDevice<NoopRawMutex, Spi<Blocking>, Output>, Output>,
        ST7789,
        NoResetPin,
    >,
> = StaticCell::new();
static UART_MIDI_READER: StaticCell<UartMidiReader> = StaticCell::new();
static UART_MIDI_WRITER: StaticCell<UartMidiWriter> = StaticCell::new();
static STORAGE_MANAGER: StaticCell<FakeStorageManager> = StaticCell::new();
static SPI_BUS: StaticCell<Mutex<NoopRawMutex, RefCell<Spi<Blocking>>>> = StaticCell::new();

#[embassy_executor::task]
async fn midi_thru_task(app: &'static mut FirmwareApplication) -> ! {
    loop {
        app.midi_thru_task().await;
    }
}

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

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
    let spi_bus = SPI_BUS.init(Mutex::new(RefCell::new(spi)));
    let spi_device = SpiDevice::new(spi_bus, cs);

    let dc = Output::new(peripherals.GPIO21, Level::Low, OutputConfig::default());
    let di = SPIInterface::new(spi_device, dc);
    let display = DISPLAY_1.init(
        mipidsi::Builder::new(ST7789, di)
            .display_size(240, 280)
            .orientation(Orientation::default().rotate(Deg270))
            .display_offset(0, 20)
            .invert_colors(ColorInversion::Inverted)
            // TODO: Add reset pin
            .init(&mut Delay)
            .expect("Failed to initialise ST7789 display"),
    );

    display.clear(Rgb565::BLACK).unwrap();

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
    let (local_rx, local_tx) = uart.split();
    let rx = RX.init(local_rx);
    let tx = TX.init(local_tx);

    info!("Startup complete.");

    let midi_reader = UART_MIDI_READER.init(UartMidiReader::new(rx));
    let midi_writer = UART_MIDI_WRITER.init(UartMidiWriter::new(tx));
    let storage_manager = STORAGE_MANAGER.init(FakeStorageManager::default());

    let app = APP.init(
        ApplicationBuilder::new()
            .with_display(display)
            .with_midi_reader(midi_reader)
            .with_midi_writer(midi_writer)
            .with_storage_manager(storage_manager)
            .build(),
    );

    // Start app tasks here
    spawner.spawn(midi_thru_task(app)).unwrap();

    core::future::pending().await
}
