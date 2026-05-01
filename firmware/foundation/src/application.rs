use crate::channels::{
    ButtonEvent, ButtonEventReceiver, ButtonIdentifier, DisplayIdentifier,
    DisplayStateUpdateChannel, MidiOutChannel, StorageStateEvent, StorageStateUpdateChannel,
};
use crate::layout::DisplayLayout;
use crate::midi::{MidiReader, MidiWriter};
use crate::storage::StorageManager;
use crate::time::TimeSource;
use core::cell::RefCell;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::pixelcolor::Rgb565;
use hashbrown::HashMap;
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

pub(crate) struct InternalChannels<'a> {
    midi_out: MidiOutChannel,
    display_state_update: DisplayStateUpdateChannel,
    storage_state_update: StorageStateUpdateChannel,
    button_event: &'a ButtonEventReceiver<'a>,
}

pub struct Application<'a, MR: MidiReader, MW: MidiWriter, SM: StorageManager, TS: TimeSource> {
    pub(crate) midi_streams: MidiStreams<'a, MR, MW>,
    pub(crate) channels: InternalChannels<'a>,
    pub(crate) storage_manager: &'a mut SM,
    pub(crate) time_source: &'a TS, // TODO: Add protocol streams
                                    // TODO: Add buttons
}

impl<'a, MR: MidiReader, MW: MidiWriter, SM: StorageManager, TS: TimeSource>
    Application<'a, MR, MW, SM, TS>
{
    pub fn new(
        midi_reader: &'a mut MR,
        midi_writer: &'a mut MW,
        storage_manager: &'a mut SM,
        button_event_receiver: &'a mut ButtonEventReceiver<'a>,
        time_source: &'a TS,
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
                button_event: button_event_receiver,
            },
            storage_manager,
            time_source,
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
        info!("Starting button task (inside task)");

        // TODO: Maybe create an enum to represent a single button's state machine
        let mut button_pressed_at = HashMap::<ButtonIdentifier, u64>::with_capacity(8);
        let mut button_released_at = HashMap::<ButtonIdentifier, u64>::with_capacity(8);

        loop {
            #[cfg(target_arch = "wasm32")]
            let button_event = self.channels.button_event.recv().await.unwrap();
            #[cfg(not(target_arch = "wasm32"))]
            let button_event = self.channels.button_event.receive().await;
            info!("Received button event: {:?}", button_event);
            match button_event {
                ButtonEvent::Pressed { button_identifier } => {
                    // TODO: We can do some logic here to register double-presses
                    button_released_at.remove(&button_identifier);
                    button_pressed_at.insert(button_identifier, self.time_source.now());
                    // TODO: Trigger momentary button pressed event?
                }
                ButtonEvent::Released { button_identifier } => {
                    if let Some(pressed_at) = button_pressed_at.remove(&button_identifier) {
                        let duration_pressed = self
                            .time_source
                            .duration(pressed_at, self.time_source.now());
                        if duration_pressed.as_secs().ge(&1) {
                            // TODO: Long press?
                        }
                    }
                    // TODO: Trigger momentary button released event?
                    // TODO: Trigger regular button press event?
                }
            }
        }
    }

    /// Read app state updates and render them to the display
    pub async fn display_task<D: DrawTarget<Color = Rgb565>>(
        &self,
        displays: &mut Displays<'_, D>,
    ) -> !
    where
        <D as DrawTarget>::Error: core::fmt::Debug,
    {
        let mut display_1_layout = DisplayLayout::new(displays.display_1);
        let mut display_2_layout = DisplayLayout::new(displays.display_2);
        let mut display_3_layout = DisplayLayout::new(displays.display_3);
        let mut display_4_layout = DisplayLayout::new(displays.display_4);

        loop {
            let update_message = self.channels.display_state_update.receive().await;
            let target = match update_message.display_identifier {
                DisplayIdentifier::Display1 => &mut display_1_layout,
                DisplayIdentifier::Display2 => &mut display_2_layout,
                DisplayIdentifier::Display3 => &mut display_3_layout,
                DisplayIdentifier::Display4 => &mut display_4_layout,
            };
            target.set_top_box_colour(update_message.top_row_color.into());
            target.set_bottom_box_colour(update_message.bottom_row_color.into());

            target.set_top_text(update_message.top_row_text);
            target.set_bottom_text(update_message.bottom_row_text);

            target.draw().unwrap();
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
pub struct ApplicationBuilder<
    'a,
    MR: MidiReader,
    MW: MidiWriter,
    SM: StorageManager,
    TS: TimeSource,
> {
    midi_reader: Option<&'a mut MR>,
    midi_writer: Option<&'a mut MW>,
    storage_manager: Option<&'a mut SM>,
    button_event_receiver: Option<&'a mut ButtonEventReceiver<'a>>,
    time_source: Option<&'a TS>,
}

impl<'a, MR: MidiReader, MW: MidiWriter, SM: StorageManager, TS: TimeSource>
    ApplicationBuilder<'a, MR, MW, SM, TS>
{
    pub fn new() -> Self {
        Self {
            midi_reader: None,
            midi_writer: None,
            storage_manager: None,
            button_event_receiver: None,
            time_source: None,
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

    pub fn with_button_event_receiver(
        mut self,
        button_event_receiver: &'a mut ButtonEventReceiver<'a>,
    ) -> Self {
        self.button_event_receiver = Some(button_event_receiver);
        self
    }

    pub fn with_time_source(mut self, time_source: &'a TS) -> Self {
        self.time_source = Some(time_source);
        self
    }

    pub fn build(self) -> Application<'a, MR, MW, SM, TS> {
        Application::new(
            self.midi_reader.expect("MIDI reader is required"),
            self.midi_writer.expect("MIDI writer is required"),
            self.storage_manager.expect("Storage manager is required"),
            self.button_event_receiver
                .expect("Button event channel is required"),
            self.time_source.expect("Time source is required"),
        )
    }
}
