use crate::application::channels::{
    ButtonEventChannel, DisplayIdentifier, DisplayStateUpdateChannel, MidiOutChannel,
    StorageStateEvent, StorageStateUpdateChannel,
};
use crate::layout::DisplayLayout;
use crate::midi::{MidiReader, MidiWriter};
use crate::storage::StorageManager;
use core::cell::RefCell;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::pixelcolor::Rgb565;
use log::info;

pub struct Displays<'a, D: DrawTarget<Color = Rgb565>> {
    pub(crate) display_1: &'a mut D,
    pub(crate) display_2: &'a mut D,
    pub(crate) display_3: &'a mut D,
    pub(crate) display_4: &'a mut D,
}

impl<'a, D: DrawTarget<Color = Rgb565>> Displays<'a, D> {
    pub fn new(
        display_1: &'a mut D,
        display_2: &'a mut D,
        display_3: &'a mut D,
        display_4: &'a mut D,
    ) -> Self {
        Self {
            display_1,
            display_2,
            display_3,
            display_4,
        }
    }
}

pub(crate) struct MidiStreams<'a, MR: MidiReader, MW: MidiWriter> {
    reader: RefCell<&'a mut MR>,
    writer: RefCell<&'a mut MW>,
}

pub(crate) struct InternalChannels {
    midi_out: MidiOutChannel,
    display_state_update: DisplayStateUpdateChannel,
    storage_state_update: StorageStateUpdateChannel,
    button_event: ButtonEventChannel,
}

pub struct Application<'a, MR: MidiReader, MW: MidiWriter, SM: StorageManager> {
    pub(crate) midi_streams: MidiStreams<'a, MR, MW>,
    pub(crate) channels: InternalChannels,
    pub(crate) storage_manager: &'a mut SM,
    // TODO: Add protocol streams
    // TODO: Add buttons
}

impl<'a, MR: MidiReader, MW: MidiWriter, SM: StorageManager> Application<'a, MR, MW, SM> {
    pub fn new(
        midi_reader: &'a mut MR,
        midi_writer: &'a mut MW,
        storage_manager: &'a mut SM,
    ) -> Self {
        Self {
            midi_streams: MidiStreams {
                reader: RefCell::new(midi_reader),
                writer: RefCell::new(midi_writer),
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
                .borrow_mut()
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
                .borrow_mut()
                .write_midi_packet(&packet)
                .await
                .unwrap();
        }
    }

    pub async fn button_task(&self) -> ! {
        loop {
            let button_event = self.channels.button_event.receive().await;
        }
    }

    /// Read app state updates and render them to the display
    pub async fn display_task<D: DrawTarget<Color = Rgb565>>(
        &self,
        displays: &mut Displays<'_, D>,
    ) -> ! {
        let mut display_1_layout = DisplayLayout::new(displays.display_1);
        let mut display_2_layout = DisplayLayout::new(displays.display_2);
        let mut display_3_layout = DisplayLayout::new(displays.display_3);
        let mut display_4_layout = DisplayLayout::new(displays.display_4);

        loop {
            let update_message = self.channels.display_state_update.receive().await;
            let target = match update_message.display_identifier {
                DisplayIdentifier::DisplayOne => &mut display_1_layout,
                DisplayIdentifier::DisplayTwo => &mut display_2_layout,
                DisplayIdentifier::DisplayThree => &mut display_3_layout,
                DisplayIdentifier::DisplayFour => &mut display_4_layout,
            };
            // TODO: Update layout for display
        }
    }

    pub async fn storage_read_task(&self) -> ! {
        info!("Loading presets");
        let _presets = self
            .storage_manager
            .load_presets()
            .expect("Failed to load presets from storage");
        loop {
            let event = self.channels.storage_state_update.receive().await;
            match event {
                StorageStateEvent::PresetUpdate { .. } => todo!(),
                StorageStateEvent::SavePreset => todo!(),
            }
        }
    }
}

#[derive(Default)]
pub struct ApplicationBuilder<'a, MR: MidiReader, MW: MidiWriter, SM: StorageManager> {
    midi_reader: Option<&'a mut MR>,
    midi_writer: Option<&'a mut MW>,
    storage_manager: Option<&'a mut SM>,
}

impl<'a, MR: MidiReader, MW: MidiWriter, SM: StorageManager> ApplicationBuilder<'a, MR, MW, SM> {
    pub fn new() -> Self {
        Self {
            midi_reader: None,
            midi_writer: None,
            storage_manager: None,
        }
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

    pub fn build(self) -> Application<'a, MR, MW, SM> {
        Application::new(
            self.midi_reader.expect("MIDI reader is required"),
            self.midi_writer.expect("MIDI writer is required"),
            self.storage_manager.expect("Storage manager is required"),
        )
    }
}
