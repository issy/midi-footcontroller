use crate::layout::DisplayText;
use crate::midi::MidiPacket;
use crate::protocol::Colour;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;

pub type MidiOutChannel = embassy_sync::channel::Channel<CriticalSectionRawMutex, MidiPacket, 128>;

pub struct DisplayStateUpdateMessage {
    pub(crate) display_index: i8,
    pub(crate) top_row_text: DisplayText,
    pub(crate) top_row_color: Colour,
    pub(crate) bottom_row_text: DisplayText,
    pub(crate) bottom_row_color: Colour,
}

pub type DisplayStateUpdateChannel =
    embassy_sync::channel::Channel<CriticalSectionRawMutex, DisplayStateUpdateMessage, 16>;

pub enum ButtonEvent {
    Pressed { button_index: i8 },
    Released { button_index: i8 },
}

pub type ButtonEventChannel =
    embassy_sync::channel::Channel<CriticalSectionRawMutex, ButtonEvent, 16>;

// TODO: Add channel for state updates
pub enum StorageStateEvent {
    PresetUpdate {
        preset_name: DisplayText,
        // Display 1
        display_1_top_row_text: Option<DisplayText>,
        display_1_top_row_color: Option<Colour>,
        display_1_bottom_row_text: Option<DisplayText>,
        display_1_bottom_row_color: Option<Colour>,
        // Display 2
        display_2_top_row_text: Option<DisplayText>,
        display_2_top_row_color: Option<Colour>,
        display_2_bottom_row_text: Option<DisplayText>,
        display_2_bottom_row_color: Option<Colour>,
        // Display 3
        display_3_top_row_text: Option<DisplayText>,
        display_3_top_row_color: Option<Colour>,
        display_3_bottom_row_text: Option<DisplayText>,
        display_3_bottom_row_color: Option<Colour>,
        // Display 4
        display_4_top_row_text: Option<DisplayText>,
        display_4_top_row_color: Option<Colour>,
        display_4_bottom_row_text: Option<DisplayText>,
        display_4_bottom_row_color: Option<Colour>,
        // TODO: Button actions
    },
    SavePreset,
}

pub type StorageStateUpdateChannel =
    embassy_sync::channel::Channel<CriticalSectionRawMutex, StorageStateEvent, 16>;
