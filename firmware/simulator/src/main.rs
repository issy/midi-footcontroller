mod midi;
mod sleep;
mod storage;

use crate::midi::{FakeMidiReader, FakeMidiWriter};
use crate::storage::{LocalStorageManager, Preset};
use embedded_graphics::geometry::Dimensions;
use embedded_graphics::mono_font::ascii::FONT_10X20;
use embedded_graphics::prelude::Primitive;
use embedded_graphics::prelude::RgbColor;
use embedded_graphics::primitives::{PrimitiveStyleBuilder, StyledDrawable};
use embedded_graphics::{
    Drawable,
    mono_font::MonoTextStyle,
    pixelcolor::Rgb565,
    prelude::{Point, WebColors},
    text::Text,
};
use embedded_graphics_web_simulator::{
    display::WebSimulatorDisplay, output_settings::OutputSettingsBuilder,
};
use foundation::application::state::{Application, ApplicationBuilder, Displays};
use log::{Level, info};
use static_cell::StaticCell;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlButtonElement, Storage, window};

const STORAGE_KEY_PRESETS: &str = "presets";
const STORAGE_KEY_PRESET_ID: &str = "preset_id";

static LOCAL_STORAGE: StaticCell<Storage> = StaticCell::new();
static MIDI_READER: StaticCell<FakeMidiReader> = StaticCell::new();
static MIDI_WRITER: StaticCell<FakeMidiWriter> = StaticCell::new();
static STORAGE_MANAGER: StaticCell<LocalStorageManager> = StaticCell::new();
static APP: StaticCell<Application<FakeMidiReader, FakeMidiWriter, LocalStorageManager>> =
    StaticCell::new();
static DISPLAY_1: StaticCell<WebSimulatorDisplay<Rgb565>> = StaticCell::new();
static DISPLAY_2: StaticCell<WebSimulatorDisplay<Rgb565>> = StaticCell::new();
static DISPLAY_3: StaticCell<WebSimulatorDisplay<Rgb565>> = StaticCell::new();
static DISPLAY_4: StaticCell<WebSimulatorDisplay<Rgb565>> = StaticCell::new();
static DISPLAYS: StaticCell<Displays<WebSimulatorDisplay<Rgb565>>> = StaticCell::new();

pub fn init_logging() {
    console_log::init_with_level(Level::Debug).expect("logger init failed");
}

fn main() {
    console_error_panic_hook::set_once();
    init_logging();

    let local_storage = LOCAL_STORAGE.init(
        window()
            .unwrap()
            .local_storage()
            .expect("Failed to access localStorage")
            .expect("No localStorage"),
    );

    let initial_preset_id: u8 = local_storage
        .get_item(STORAGE_KEY_PRESET_ID)
        .expect("Failed to get item from localStorage")
        .map(|v| {
            v.parse::<u8>()
                .expect("Failed to parse item from localStorage")
        })
        .unwrap_or(0);

    let document = window()
        .and_then(|win| win.document())
        .expect("Could not access the document");
    let body = document.body().expect("Could not access document.body");
    let root_element = document
        .get_element_by_id("app")
        .expect("Could not find root element with id 'app'");

    root_element
        .set_attribute("style", "display: grid; grid-template-columns: repeat(4, 1fr); grid-template-rows: 2em auto 2em; gap: 1rem;")
        .unwrap();
    let button_1_element = root_element
        .append_child(&document.create_element("button").unwrap())
        .and_then(|el| Ok(el.unchecked_into::<HtmlButtonElement>()))
        .expect("Failed to create button 1 element");
    let button_3_element = root_element
        .append_child(&document.create_element("button").unwrap())
        .and_then(|el| Ok(el.unchecked_into::<HtmlButtonElement>()))
        .expect("Failed to create button 3 element");
    let button_5_element = root_element
        .append_child(&document.create_element("button").unwrap())
        .and_then(|el| Ok(el.unchecked_into::<HtmlButtonElement>()))
        .expect("Failed to create button 5 element");
    let button_6_element = root_element
        .append_child(&document.create_element("button").unwrap())
        .and_then(|el| Ok(el.unchecked_into::<HtmlButtonElement>()))
        .expect("Failed to create button 6 element");

    let display_1_element = root_element
        .append_child(&document.create_element("div").unwrap())
        .and_then(|el| Ok(el.dyn_into::<Element>()?))
        .and_then(|el| {
            el.set_attribute("style", "display: flex;")?;
            Ok(el)
        })
        .expect("Failed to create display-1 element");
    let display_2_element = root_element
        .append_child(&document.create_element("div").unwrap())
        .and_then(|el| Ok(el.dyn_into::<Element>()?))
        .and_then(|el| {
            el.set_attribute("style", "display: flex;")?;
            Ok(el)
        })
        .expect("Failed to create display-2 element");
    let display_3_element = root_element
        .append_child(&document.create_element("div").unwrap())
        .and_then(|el| Ok(el.dyn_into::<Element>()?))
        .and_then(|el| {
            el.set_attribute("style", "display: flex;")?;
            Ok(el)
        })
        .expect("Failed to create display-3 element");
    let display_4_element = root_element
        .append_child(&document.create_element("div").unwrap())
        .and_then(|el| Ok(el.dyn_into::<Element>()?))
        .and_then(|el| {
            el.set_attribute("style", "display: flex;")?;
            Ok(el)
        })
        .expect("Failed to create display-4 element");

    let button_2_element = root_element
        .append_child(&document.create_element("button").unwrap())
        .and_then(|el| Ok(el.unchecked_into::<HtmlButtonElement>()))
        .expect("Failed to create button 2 element");
    let button_4_element = root_element
        .append_child(&document.create_element("button").unwrap())
        .and_then(|el| Ok(el.unchecked_into::<HtmlButtonElement>()))
        .expect("Failed to create button 4 element");
    let button_6_element = root_element
        .append_child(&document.create_element("button").unwrap())
        .and_then(|el| Ok(el.unchecked_into::<HtmlButtonElement>()))
        .expect("Failed to create button 6 element");
    let button_8_element = root_element
        .append_child(&document.create_element("button").unwrap())
        .and_then(|el| Ok(el.unchecked_into::<HtmlButtonElement>()))
        .expect("Failed to create button 8 element");

    let text_style = MonoTextStyle::new(&FONT_10X20, Rgb565::CSS_ORANGE);
    let display_output_settings = OutputSettingsBuilder::new()
        .scale(1)
        .pixel_spacing(0)
        .build();
    let display_1 = DISPLAY_1.init(WebSimulatorDisplay::new(
        (240, 280),
        &display_output_settings,
        Some(display_1_element.as_ref()),
    ));
    let display_2 = DISPLAY_2.init(WebSimulatorDisplay::new(
        (240, 280),
        &display_output_settings,
        Some(display_2_element.as_ref()),
    ));
    let display_3 = DISPLAY_3.init(WebSimulatorDisplay::new(
        (240, 280),
        &display_output_settings,
        Some(display_3_element.as_ref()),
    ));
    let display_4 = DISPLAY_4.init(WebSimulatorDisplay::new(
        (240, 280),
        &display_output_settings,
        Some(display_4_element.as_ref()),
    ));

    display_1
        .bounding_box()
        .draw_styled(
            &PrimitiveStyleBuilder::new()
                .fill_color(Rgb565::BLACK)
                .build(),
            display_1,
        )
        .unwrap();
    display_2
        .bounding_box()
        .draw_styled(
            &PrimitiveStyleBuilder::new()
                .fill_color(Rgb565::BLACK)
                .build(),
            display_2,
        )
        .unwrap();
    display_3
        .bounding_box()
        .draw_styled(
            &PrimitiveStyleBuilder::new()
                .fill_color(Rgb565::BLACK)
                .build(),
            display_3,
        )
        .unwrap();
    display_4
        .bounding_box()
        .draw_styled(
            &PrimitiveStyleBuilder::new()
                .fill_color(Rgb565::BLACK)
                .build(),
            display_4,
        )
        .unwrap();

    Text::new("Hello, world!", Point::new(10, 30), text_style)
        .draw(display_1)
        .unwrap();
    Text::new("Hello, world!", Point::new(10, 30), text_style)
        .draw(display_3)
        .unwrap();
    Text::new("Bellooooo", Point::new(10, 30), text_style)
        .draw(display_4)
        .unwrap();
    display_1.flush().unwrap();
    display_2.flush().unwrap();
    display_3.flush().unwrap();
    display_4.flush().unwrap();

    let foo = embedded_graphics::primitives::Circle::new(Point::new(29, 29), 70).into_styled(
        embedded_graphics::primitives::PrimitiveStyle::with_stroke(Rgb565::CSS_WHITE, 1),
    );
    foo.draw(display_1).unwrap();
    foo.draw(display_2).unwrap();

    display_1.flush().unwrap();
    display_2.flush().unwrap();

    let midi_reader = MIDI_READER.init(FakeMidiReader::default());
    let midi_writer = MIDI_WRITER.init(FakeMidiWriter::default());
    let storage_manager = STORAGE_MANAGER.init(LocalStorageManager::new(local_storage));

    let app = APP.init(
        ApplicationBuilder::new()
            .with_midi_reader(midi_reader)
            .with_midi_writer(midi_writer)
            .with_storage_manager(storage_manager)
            .build(),
    );
    let displays = DISPLAYS.init(Displays::new(display_1, display_2, display_3, display_4));

    info!("Hello world from main");
    async_wasm_task::spawn(async {
        app.storage_read_task().await;
    });
    info!("Started storage task");
    async_wasm_task::spawn(async {
        app.midi_thru_task().await;
    });
    info!("Started midi thru task");
    async_wasm_task::spawn(async {
        app.midi_out_task().await;
    });
    info!("Started midi out task");
    async_wasm_task::spawn(async {
        app.button_task().await;
    });
    info!("Started button task");
    async_wasm_task::spawn(async {
        app.display_task(displays).await;
    });
    info!("Started display task");
}
