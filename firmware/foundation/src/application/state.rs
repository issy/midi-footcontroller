use crate::application::{Displays, MidiStreams};
use crate::midi::{MidiReader, MidiWriter};
use embedded_graphics::draw_target::DrawTarget;

pub struct Application<'a, D: DrawTarget, MR: MidiReader, MW: MidiWriter> {
    displays: Displays<'a, D>,
    midi_streams: MidiStreams<'a, MR, MW>,
    // TODO: Add protocol streams
    // TODO: Add buttons
}

impl<'a, D: DrawTarget, MR: MidiReader, MW: MidiWriter> Application<'a, D, MR, MW> {
    pub fn new(
        display_1: &'a mut D,
        display_2: &'a mut D,
        display_3: &'a mut D,
        display_4: &'a mut D,
        midi_reader: &'a mut MR,
        midi_writer: &'a mut MW,
    ) -> Self {
        // Maybe a good idea to create the channels here?
        Self {
            displays: Displays {
                display_1,
                display_2,
                display_3,
                display_4,
            },
            midi_streams: MidiStreams {
                reader: midi_reader,
                writer: midi_writer,
            },
        }
    }
}

pub struct ApplicationBuilder<'a, D: DrawTarget, MR: MidiReader, MW: MidiWriter> {
    display_1: Option<&'a mut D>,
    display_2: Option<&'a mut D>,
    display_3: Option<&'a mut D>,
    display_4: Option<&'a mut D>,
    midi_reader: Option<&'a mut MR>,
    midi_writer: Option<&'a mut MW>,
}

impl<'a, D: DrawTarget, MR: MidiReader, MW: MidiWriter> ApplicationBuilder<'a, D, MR, MW> {
    pub fn new() -> Self {
        Self {
            display_1: None,
            display_2: None,
            display_3: None,
            display_4: None,
            midi_reader: None,
            midi_writer: None,
        }
    }

    pub fn with_display(mut self, display: &'a mut D) -> Self {
        if self.display_1.is_none() {
            self.display_1 = Some(display);
        } else if self.display_2.is_none() {
            self.display_2 = Some(display);
        } else if self.display_3.is_none() {
            self.display_3 = Some(display);
        } else if self.display_4.is_none() {
            self.display_4 = Some(display);
        } else {
            panic!("All 4 displays are already set");
        }
        self
    }

    pub fn with_midi_reader(mut self, reader: &'a mut MR) -> Self {
        self.midi_reader = Some(reader);
        self
    }

    pub fn with_midi_writer(mut self, writer: &'a mut MW) -> Self {
        self.midi_writer = Some(writer);
        self
    }

    pub fn build(self) -> Application<'a, D, MR, MW> {
        Application::new(
            self.display_1.expect("Display 1 is required"),
            self.display_2.expect("Display 2 is required"),
            self.display_3.expect("Display 3 is required"),
            self.display_4.expect("Display 4 is required"),
            self.midi_reader.expect("MIDI reader is required"),
            self.midi_writer.expect("MIDI writer is required"),
        )
    }
}
