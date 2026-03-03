mod display_task;
pub mod state;

use crate::midi::{MidiReader, MidiWriter};
use embedded_graphics::draw_target::DrawTarget;

struct Displays<'a, D: DrawTarget> {
    display_1: &'a mut D,
    display_2: &'a mut D,
    display_3: &'a mut D,
    display_4: &'a mut D,
}

struct MidiStreams<'a, MR: MidiReader, MW: MidiWriter> {
    reader: &'a mut MR,
    writer: &'a mut MW,
}
