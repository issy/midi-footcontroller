mod midi;
mod sleep;
mod storage;

use crate::midi::{FakeMidiReader, FakeMidiWriter};
use crate::sleep::sleep;
use crate::storage::LocalStorageManager;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::mono_font::ascii::FONT_10X20;
use embedded_graphics::prelude::Primitive;
use embedded_graphics::prelude::RgbColor;
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
use wasm_bindgen::prelude::*;
use web_sys::console::info;
use web_sys::js_sys::futures::spawn_local;
use web_sys::{Document, Element, EventListener, HtmlButtonElement, Storage, window};

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

fn init_logging() {
    console_log::init_with_level(Level::Debug).expect("logger init failed");
}

fn create_button_element(
    document: &Document,
    parent: &Element,
) -> Result<HtmlButtonElement, JsValue> {
    parent
        .append_child(&document.create_element("button")?.into())
        .and_then(|el| Ok(el.unchecked_into::<HtmlButtonElement>()))
}

fn create_display_element(document: &Document, parent: &Element) -> Result<Element, JsValue> {
    parent
        .append_child(&document.create_element("div")?.into())
        .and_then(|el| Ok(el.dyn_into::<Element>()?))
        .and_then(|el| {
            el.set_attribute("style", "display: flex; justify-content: center;")?;
            Ok(el)
        })
}

#[wasm_bindgen]
pub fn teardown() {
    let document = window()
        .and_then(|win| win.document())
        .expect("Could not access the document");
    if let Some(root_element) = document.get_element_by_id("simulator-root") {
        document
            .remove_child(&root_element)
            .expect("Failed to remove root element");
    }
}

async fn bloop() {
    info!("Starting bloop...");
    sleep(1000).await;
    info!("Bloop complete!");
}

#[wasm_bindgen]
pub fn main() {
    console_error_panic_hook::set_once();
    init_logging();

    let local_storage = LOCAL_STORAGE.init(
        window()
            .unwrap()
            .local_storage()
            .expect("Failed to access localStorage")
            .expect("No localStorage"),
    );
    let midi_reader = MIDI_READER.init(FakeMidiReader::default());
    let midi_writer = MIDI_WRITER.init(FakeMidiWriter::default());
    let storage_manager = STORAGE_MANAGER.init(LocalStorageManager::new(local_storage));
    let button_event_channel = foundation::application::channels::ButtonEventChannel::new();

    let document = window()
        .and_then(|win| win.document())
        .expect("Could not access the document");
    let root_element = document
        .get_element_by_id("simulator-root")
        .and_then(|el| {
            el.set_attribute("style", "display: grid; grid-template-columns: repeat(4, 1fr); grid-template-rows: 2em auto 2em; gap: 1rem;").ok()?;
            Some(el)
        })
        .expect("Could not find root element with id 'simulator-root'");

    // Buttons   1 3 5 7
    // Displays  1 2 3 4
    // Buttons   2 4 6 8

    root_element
            .set_attribute("style", "display: grid; grid-template-columns: repeat(4, 1fr); grid-template-rows: 2em auto 2em; gap: 1rem;")
            .unwrap();

    let _button_1_element =
        create_button_element(&document, &root_element).expect("Failed to create button 1 element");
    let _button_3_element =
        create_button_element(&document, &root_element).expect("Failed to create button 3 element");
    let _button_5_element =
        create_button_element(&document, &root_element).expect("Failed to create button 5 element");
    let _button_7_element =
        create_button_element(&document, &root_element).expect("Failed to create button 7 element");

    let display_1_element = create_display_element(&document, &root_element)
        .expect("Failed to create display 1 element");
    let display_2_element = create_display_element(&document, &root_element)
        .expect("Failed to create display 2 element");
    let display_3_element = create_display_element(&document, &root_element)
        .expect("Failed to create display 3 element");
    let display_4_element = create_display_element(&document, &root_element)
        .expect("Failed to create display 4 element");

    let _button_2_element =
        create_button_element(&document, &root_element).expect("Failed to create button 2 element");
    let _button_4_element =
        create_button_element(&document, &root_element).expect("Failed to create button 4 element");
    let _button_6_element =
        create_button_element(&document, &root_element).expect("Failed to create button 6 element");
    let _button_8_element =
        create_button_element(&document, &root_element).expect("Failed to create button 8 element");

    let l = EventListener::new();
    let f = Closure::wrap(Box::new(move |_event: web_sys::Event| {
        spawn_local(async move {
            bloop().await;
        });
        info!("Button clicked!");
    }) as Box<dyn FnMut(_)>);
    l.set_handle_event(f.as_ref().unchecked_ref());
    f.forget();

    _button_8_element
        .add_event_listener_with_event_listener("click", &l)
        .unwrap();

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

    display_1.clear(Rgb565::BLACK).unwrap();
    display_2.clear(Rgb565::BLACK).unwrap();
    display_3.clear(Rgb565::BLACK).unwrap();
    display_4.clear(Rgb565::BLACK).unwrap();

    Text::new("Hello, world!", Point::new(10, 30), text_style)
        .draw(display_1)
        .unwrap();
    Text::new("Hello, world!", Point::new(10, 30), text_style)
        .draw(display_3)
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

    let app = APP.init(
        ApplicationBuilder::new()
            .with_midi_reader(midi_reader)
            .with_midi_writer(midi_writer)
            .with_storage_manager(storage_manager)
            .with_button_event_channel(&button_event_channel)
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
