use crate::application::channels::{
    ButtonEventChannel, DisplayStateUpdateChannel, MidiOutChannel, StorageStateUpdateChannel,
};
use crate::midi::{MidiReader, MidiWriter};
use crate::storage::StorageManager;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::pixelcolor::Rgb565;

pub(crate) struct Displays<'a, D: DrawTarget<Color = Rgb565>> {
    pub(crate) display_1: &'a mut D,
    pub(crate) display_2: &'a mut D,
    pub(crate) display_3: &'a mut D,
    pub(crate) display_4: &'a mut D,
}

pub(crate) struct MidiStreams<'a, MR: MidiReader, MW: MidiWriter> {
    reader: &'a mut MR,
    writer: &'a mut MW,
}

pub(crate) struct InternalChannels<'a> {
    midi_out: &'a mut MidiOutChannel,
    display_state_update: &'a mut DisplayStateUpdateChannel,
    storage_state_update: &'a mut StorageStateUpdateChannel,
    button_event: &'a mut ButtonEventChannel,
}

pub struct Application<
    'a,
    D: DrawTarget<Color = Rgb565>,
    MR: MidiReader,
    MW: MidiWriter,
    SM: StorageManager,
> {
    pub(crate) displays: Displays<'a, D>,
    pub(crate) midi_streams: MidiStreams<'a, MR, MW>,
    pub(crate) channels: InternalChannels<'a>,
    pub(crate) storage_manager: &'a mut SM,
    // TODO: Add protocol streams
    // TODO: Add buttons
}

impl<'a, D: DrawTarget<Color = Rgb565>, MR: MidiReader, MW: MidiWriter, SM: StorageManager>
    Application<'a, D, MR, MW, SM>
{
    pub fn new(
        display_1: &'a mut D,
        display_2: &'a mut D,
        display_3: &'a mut D,
        display_4: &'a mut D,
        midi_reader: &'a mut MR,
        midi_writer: &'a mut MW,
        storage_manager: &'a mut SM,
        midi_out_channel: &'a mut MidiOutChannel,
        display_state_update_channel: &'a mut DisplayStateUpdateChannel,
        storage_state_update_channel: &'a mut StorageStateUpdateChannel,
        button_event_channel: &'a mut ButtonEventChannel,
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
            channels: InternalChannels {
                midi_out: midi_out_channel,
                display_state_update: display_state_update_channel,
                storage_state_update: storage_state_update_channel,
                button_event: button_event_channel,
            },
            storage_manager,
        }
    }
}

#[derive(Default)]
pub struct ApplicationBuilder<
    'a,
    D: DrawTarget<Color = Rgb565>,
    MR: MidiReader,
    MW: MidiWriter,
    SM: StorageManager,
> {
    display_1: Option<&'a mut D>,
    display_2: Option<&'a mut D>,
    display_3: Option<&'a mut D>,
    display_4: Option<&'a mut D>,
    midi_reader: Option<&'a mut MR>,
    midi_writer: Option<&'a mut MW>,
    storage_manager: Option<&'a mut SM>,
    midi_out_channel: Option<&'a mut MidiOutChannel>,
    display_state_update_channel: Option<&'a mut DisplayStateUpdateChannel>,
    storage_state_update_channel: Option<&'a mut StorageStateUpdateChannel>,
    button_event_channel: Option<&'a mut ButtonEventChannel>,
}

impl<'a, D: DrawTarget<Color = Rgb565>, MR: MidiReader, MW: MidiWriter, SM: StorageManager>
    ApplicationBuilder<'a, D, MR, MW, SM>
{
    pub fn new() -> Self {
        Self {
            display_1: None,
            display_2: None,
            display_3: None,
            display_4: None,
            midi_reader: None,
            midi_writer: None,
            midi_out_channel: None,
            display_state_update_channel: None,
            storage_state_update_channel: None,
            storage_manager: None,
            button_event_channel: None,
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

    pub fn with_channels(
        mut self,
        midi_out_channel: &'a mut MidiOutChannel,
        display_state_update_channel: &'a mut DisplayStateUpdateChannel,
        storage_state_update_channel: &'a mut StorageStateUpdateChannel,
        button_event_channel: &'a mut ButtonEventChannel,
    ) -> Self {
        self.midi_out_channel = Some(midi_out_channel);
        self.display_state_update_channel = Some(display_state_update_channel);
        self.storage_state_update_channel = Some(storage_state_update_channel);
        self.button_event_channel = Some(button_event_channel);
        self
    }

    pub fn with_storage_manager(mut self, manager: &'a mut SM) -> Self {
        self.storage_manager = Some(manager);
        self
    }

    pub fn build(self) -> Application<'a, D, MR, MW, SM> {
        Application::new(
            self.display_1.expect("Display 1 is required"),
            self.display_2.expect("Display 2 is required"),
            self.display_3.expect("Display 3 is required"),
            self.display_4.expect("Display 4 is required"),
            self.midi_reader.expect("MIDI reader is required"),
            self.midi_writer.expect("MIDI writer is required"),
            self.storage_manager.expect("Storage manager is required"),
            self.midi_out_channel.expect("MIDI out channel required"),
            self.display_state_update_channel
                .expect("Display state update channel required"),
            self.storage_state_update_channel
                .expect("Storage state update channel required"),
            self.button_event_channel
                .expect("Button event channel required"),
        )
    }
}
