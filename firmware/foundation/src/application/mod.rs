mod channels;
pub mod state;

use crate::midi::{MidiReader, MidiWriter};
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::pixelcolor::Rgb565;

struct Displays<'a, D: DrawTarget<Color = Rgb565>> {
    display_1: &'a mut D,
    display_2: &'a mut D,
    display_3: &'a mut D,
    display_4: &'a mut D,
}

struct MidiStreams<'a, MR: MidiReader, MW: MidiWriter> {
    reader: &'a mut MR,
    writer: &'a mut MW,
}
