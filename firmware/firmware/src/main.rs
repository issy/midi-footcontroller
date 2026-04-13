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
use foundation::application::state::{Application, ApplicationBuilder, Displays};
use log::info;
use midi::{UartMidiReader, UartMidiWriter};
use mipidsi::models::ST7789;
use mipidsi::options::Rotation::Deg270;
use mipidsi::options::{ColorInversion, Orientation};
use mipidsi::{Display, NoResetPin};
use static_cell::StaticCell;

type FirmwareDisplay = Display<
    SPIInterface<
        SpiDevice<'static, NoopRawMutex, Spi<'static, Blocking>, Output<'static>>,
        Output<'static>,
    >,
    ST7789,
    NoResetPin,
>;
type FirmwareApplication = Application<
    'static,
    UartMidiReader<'static, 'static>,
    UartMidiWriter<'static, 'static>,
    FakeStorageManager,
>;

static RX: StaticCell<UartRx<Async>> = StaticCell::new();
static TX: StaticCell<UartTx<Async>> = StaticCell::new();
static DISPLAY_1: StaticCell<FirmwareDisplay> = StaticCell::new();
static DISPLAY_2: StaticCell<FirmwareDisplay> = StaticCell::new();
static DISPLAY_3: StaticCell<FirmwareDisplay> = StaticCell::new();
static DISPLAY_4: StaticCell<FirmwareDisplay> = StaticCell::new();
static DISPLAYS: StaticCell<Displays<FirmwareDisplay>> = StaticCell::new();
static UART_MIDI_READER: StaticCell<UartMidiReader> = StaticCell::new();
static UART_MIDI_WRITER: StaticCell<UartMidiWriter> = StaticCell::new();
static STORAGE_MANAGER: StaticCell<FakeStorageManager> = StaticCell::new();
static SPI_BUS: StaticCell<Mutex<NoopRawMutex, RefCell<Spi<Blocking>>>> = StaticCell::new();
static APP: StaticCell<FirmwareApplication> = StaticCell::new();

#[embassy_executor::task]
async fn midi_thru_task(app: &'static FirmwareApplication) -> ! {
    app.midi_thru_task().await;
}

#[embassy_executor::task]
async fn midi_out_task(app: &'static FirmwareApplication) -> ! {
    app.midi_out_task().await;
}

#[embassy_executor::task]
async fn display_task(
    app: &'static FirmwareApplication,
    displays: &'static mut Displays<'static, FirmwareDisplay>,
) -> ! {
    app.display_task(displays).await;
}

#[embassy_executor::task]
async fn storage_read_task(app: &'static FirmwareApplication) -> ! {
    app.storage_read_task().await;
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

    info!("✅ Embassy initialised");

    let spi = Spi::new(
        peripherals.SPI2,
        esp_hal::spi::master::Config::default().with_frequency(Rate::from_mhz(40)),
    )
    .expect("Failed to initialise SPI2 peripheral")
    .with_sck(peripherals.GPIO18)
    .with_mosi(peripherals.GPIO19)
    .with_cs(peripherals.GPIO20);

    let spi_bus = SPI_BUS.init(Mutex::new(RefCell::new(spi)));

    let cs_1 = Output::new(peripherals.GPIO5, Level::High, OutputConfig::default());
    let spi_device_1 = SpiDevice::new(spi_bus, cs_1);
    let dc_1 = Output::new(peripherals.GPIO21, Level::Low, OutputConfig::default());
    let display_1 = DISPLAY_1.init(
        mipidsi::Builder::new(ST7789, SPIInterface::new(spi_device_1, dc_1))
            .display_size(240, 280)
            .orientation(Orientation::default().rotate(Deg270))
            .display_offset(0, 20)
            .invert_colors(ColorInversion::Inverted)
            // TODO: Add reset pin
            .init(&mut Delay)
            .expect("Failed to initialise display 1"),
    );

    // FIXME: Dummy pins used for displays 2-4. Figure out which pins are appropriate

    let cs_2 = Output::new(peripherals.GPIO6, Level::High, OutputConfig::default());
    let spi_device_2 = SpiDevice::new(spi_bus, cs_2);
    let dc_2 = Output::new(peripherals.GPIO22, Level::Low, OutputConfig::default());
    let display_2 = DISPLAY_2.init(
        mipidsi::Builder::new(ST7789, SPIInterface::new(spi_device_2, dc_2))
            .display_size(240, 280)
            .orientation(Orientation::default().rotate(Deg270))
            .display_offset(0, 20)
            .invert_colors(ColorInversion::Inverted)
            .init(&mut Delay)
            .expect("Failed to initialise display 2"),
    );

    let cs_3 = Output::new(peripherals.GPIO10, Level::High, OutputConfig::default());
    let spi_device_3 = SpiDevice::new(spi_bus, cs_3);
    let dc_3 = Output::new(peripherals.GPIO23, Level::Low, OutputConfig::default());
    let display_3 = DISPLAY_3.init(
        mipidsi::Builder::new(ST7789, SPIInterface::new(spi_device_3, dc_3))
            .display_size(240, 280)
            .orientation(Orientation::default().rotate(Deg270))
            .display_offset(0, 20)
            .invert_colors(ColorInversion::Inverted)
            .init(&mut Delay)
            .expect("Failed to initialise display 3"),
    );

    let cs_4 = Output::new(peripherals.GPIO9, Level::High, OutputConfig::default());
    let spi_device_4 = SpiDevice::new(spi_bus, cs_4);
    let dc_4 = Output::new(peripherals.GPIO17, Level::Low, OutputConfig::default());
    let display_4 = DISPLAY_4.init(
        mipidsi::Builder::new(ST7789, SPIInterface::new(spi_device_4, dc_4))
            .display_size(240, 280)
            .orientation(Orientation::default().rotate(Deg270))
            .display_offset(0, 20)
            .invert_colors(ColorInversion::Inverted)
            .init(&mut Delay)
            .expect("Failed to initialise display 4"),
    );

    display_1.clear(Rgb565::BLACK).unwrap();
    display_2.clear(Rgb565::BLACK).unwrap();
    display_3.clear(Rgb565::BLACK).unwrap();
    display_4.clear(Rgb565::BLACK).unwrap();

    info!("✅ Displays initialised");

    let displays = DISPLAYS.init(Displays::new(display_1, display_2, display_3, display_4));

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

    let midi_reader = UART_MIDI_READER.init(UartMidiReader::new(rx));
    let midi_writer = UART_MIDI_WRITER.init(UartMidiWriter::new(tx));
    let storage_manager = STORAGE_MANAGER.init(FakeStorageManager::default());

    let app = APP.init(
        ApplicationBuilder::new()
            .with_midi_reader(midi_reader)
            .with_midi_writer(midi_writer)
            .with_storage_manager(storage_manager)
            .build(),
    );

    info!("✅ Application initialised");

    // Start app tasks here
    info!("Starting tasks...");
    spawner.spawn(midi_thru_task(app)).unwrap();
    info!("✅ midi_thru_task");
    spawner.spawn(midi_out_task(app)).unwrap();
    info!("✅ midi_out_task");
    spawner.spawn(display_task(app, displays)).unwrap();
    info!("✅ display_task");
    spawner.spawn(storage_read_task(app)).unwrap();
    info!("✅ storage_read_task");

    core::future::pending().await
}
