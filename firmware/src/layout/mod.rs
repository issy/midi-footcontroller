use core::fmt::Write;
use core::ops::Add;
use core::ops::Div;
use core::result::{Result, Result::Ok};
use embedded_graphics::{
    mono_font::{MonoTextStyleBuilder, ascii::FONT_10X20},
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
    text::Text,
};
use heapless::String;

type DisplayText = String<16>;

pub struct DisplayLayout<'a, D> {
    display: &'a mut D,
    top_text: DisplayText,
    top_box_color: Rgb565,
    bottom_text: DisplayText,
    bottom_box_color: Rgb565,
    text_style: embedded_graphics::mono_font::MonoTextStyle<'a, Rgb565>,
}

impl<'a, D> DisplayLayout<'a, D>
where
    D: DrawTarget<Color = Rgb565>,
{
    pub fn new(display: &'a mut D) -> Self {
        let text_style = MonoTextStyleBuilder::new()
            .font(&FONT_10X20)
            .text_color(Rgb565::WHITE)
            .build();

        let mut top_text = String::new();
        top_text.write_str("Hello").unwrap();
        let mut bottom_text = String::new();
        bottom_text.write_str("World").unwrap();

        Self {
            display,
            top_text,
            top_box_color: Rgb565::GREEN,
            bottom_text,
            bottom_box_color: Rgb565::BLUE,
            text_style,
        }
    }

    pub fn draw_boxes(&mut self) -> Result<(), D::Error> {
        let display_size = self.display.bounding_box().size;

        // Top box
        Rectangle::new(
            Point::zero(),
            Size::new(display_size.width, display_size.height.div(3)),
        )
        .into_styled(PrimitiveStyle::with_fill(self.top_box_color))
        .draw(self.display)?;

        // Bottom box
        Rectangle::new(
            Point::new(
                0,
                display_size
                    .height
                    .div(3)
                    .add(display_size.height.div_ceil(3)) as i32,
            ),
            Size::new(display_size.width, display_size.height.div(3)),
        )
        .into_styled(PrimitiveStyle::with_fill(self.bottom_box_color))
        .draw(self.display)?;

        Ok(())
    }

    pub fn draw_top_text(&mut self) -> Result<(), D::Error> {
        Text::new(self.top_text.as_str(), Point::new(5, 10), self.text_style).draw(self.display)?;
        Ok(())
    }

    pub fn draw_bottom_text(&mut self) -> Result<(), D::Error> {
        Text::new(
            self.bottom_text.as_str(),
            Point::new(5, 210),
            self.text_style,
        )
        .draw(self.display)?;
        Ok(())
    }

    pub fn clear_middle(&mut self) -> Result<(), D::Error> {
        let display_size = self.display.bounding_box().size;
        Rectangle::new(
            Point::new(0, display_size.height.div(3) as i32),
            Size::new(display_size.width, display_size.height.div_ceil(3)),
        )
        .into_styled(PrimitiveStyle::with_fill(Rgb565::BLACK))
        .draw(self.display)
    }

    pub fn set_top_box_colour(&mut self, colour: Rgb565) {
        self.top_box_color = colour;
    }

    pub fn set_bottom_box_colour(&mut self, colour: Rgb565) {
        self.bottom_box_color = colour;
    }

    pub fn set_top_text(&mut self, text: DisplayText) {
        self.top_text = text;
    }

    pub fn set_bottom_text(&mut self, text: DisplayText) {
        self.bottom_text = text;
    }

    pub fn draw(&mut self) -> Result<(), D::Error> {
        self.draw_boxes()?;
        self.draw_top_text()?;
        self.draw_bottom_text()?;
        self.clear_middle()?;
        Ok(())
    }
}
