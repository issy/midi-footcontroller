use crate::application::channels::{
    ButtonEventChannel, DisplayStateUpdateChannel, MidiOutChannel, StorageStateUpdateChannel,
};
use crate::midi::{MidiReader, MidiWriter};
use crate::storage::StorageManager;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::mutex::Mutex;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::pixelcolor::Rgb565;

pub(crate) struct Displays<'a, D: DrawTarget<Color = Rgb565>> {
    pub(crate) display_1: &'a mut D,
    pub(crate) display_2: &'a mut D,
    pub(crate) display_3: &'a mut D,
    pub(crate) display_4: &'a mut D,
}

pub(crate) struct MidiStreams<'a, MR: MidiReader, MW: MidiWriter> {
    reader: Mutex<NoopRawMutex, &'a mut MR>,
    writer: Mutex<NoopRawMutex, &'a mut MW>,
}

pub(crate) struct InternalChannels {
    midi_out: MidiOutChannel,
    display_state_update: DisplayStateUpdateChannel,
    storage_state_update: StorageStateUpdateChannel,
    button_event: ButtonEventChannel,
}

pub struct Application<
    'a,
    D: DrawTarget<Color = Rgb565>,
    MR: MidiReader,
    MW: MidiWriter,
    SM: StorageManager,
> {
    pub(crate) displays: Mutex<NoopRawMutex, Displays<'a, D>>,
    pub(crate) midi_streams: MidiStreams<'a, MR, MW>,
    pub(crate) channels: InternalChannels,
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
    ) -> Self {
        Self {
            displays: Mutex::new(Displays {
                display_1,
                display_2,
                display_3,
                display_4,
            }),
            midi_streams: MidiStreams {
                reader: Mutex::new(midi_reader),
                writer: Mutex::new(midi_writer),
            },
            channels: InternalChannels {
                midi_out: MidiOutChannel::new(),
                display_state_update: DisplayStateUpdateChannel::new(),
                storage_state_update: StorageStateUpdateChannel::new(),
                button_event: ButtonEventChannel::new(),
            },
            storage_manager,
        }
    }

    pub async fn midi_thru_task(&self) -> ! {
        loop {
            if let Some(packet) = self
                .midi_streams
                .reader
                .lock()
                .await
                .read_midi_packet()
                .await
                .unwrap()
            {
                // TODO: If we decide to support MIDI command input in future, this would be a good place to process those
                self.channels.midi_out.send(packet).await;
            }
        }
    }

    pub async fn midi_out_task(&self) -> ! {
        loop {
            let packet = self.channels.midi_out.receive().await;
            self.midi_streams
                .writer
                .lock()
                .await
                .write_midi_packet(&packet)
                .await
                .unwrap();
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
            storage_manager: None,
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
        )
    }
}
