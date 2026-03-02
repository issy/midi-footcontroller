use embedded_graphics::geometry::Dimensions;
use embedded_graphics::prelude::Primitive;
use embedded_graphics::prelude::RgbColor;
use embedded_graphics::primitives::{PrimitiveStyleBuilder, StyledDrawable};
use embedded_graphics::{
    Drawable,
    mono_font::{MonoTextStyle, ascii::FONT_6X9},
    pixelcolor::Rgb565,
    prelude::{Point, WebColors},
    text::Text,
};
use embedded_graphics_web_simulator::{
    display::WebSimulatorDisplay, output_settings::OutputSettingsBuilder,
};
use web_sys::window;

fn main() {
    console_error_panic_hook::set_once();

    let document = window()
        .and_then(|win| win.document())
        .expect("Could not access the document");
    let body = document.body().expect("Could not access document.body");
    let text_node = document.create_text_node("Hello, world from Vanilla Rust!");
    body.append_child(text_node.as_ref())
        .expect("Failed to append text");

    let style = MonoTextStyle::new(&FONT_6X9, Rgb565::CSS_ORANGE);
    let output_settings = OutputSettingsBuilder::new()
        .scale(4)
        .pixel_spacing(0)
        .build();
    let mut text_display = WebSimulatorDisplay::new(
        (128, 64),
        &output_settings,
        document.get_element_by_id("app").as_ref(),
    );
    text_display.flush().unwrap();
    text_display
        .bounding_box()
        .draw_styled(
            &PrimitiveStyleBuilder::new()
                .fill_color(Rgb565::BLACK)
                .build(),
            &mut text_display,
        )
        .unwrap();
    Text::new("Hello, world!", Point::new(10, 30), style)
        .draw(&mut text_display)
        .unwrap();
    text_display.flush().unwrap();

    if embedded_graphics::primitives::Circle::new(Point::new(29, 29), 70)
        .into_styled(embedded_graphics::primitives::PrimitiveStyle::with_stroke(
            Rgb565::CSS_WHITE,
            1,
        ))
        .draw(&mut text_display)
        .is_err()
    {
        web_sys::console::log_1(&"Couldn't draw circle".into());
    }

    web_sys::console::log_1(&"What da hell".into());
}
