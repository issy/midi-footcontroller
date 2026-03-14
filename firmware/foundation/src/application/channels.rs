use crate::layout::DisplayText;
use crate::midi::MidiPacket;
use crate::protocol::Colour;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;

pub type MidiOutChannel = Channel<CriticalSectionRawMutex, MidiPacket, 128>;

pub struct DisplayStateUpdateMessage {
    pub(crate) display_index: i8,
    pub(crate) top_row_text: DisplayText,
    pub(crate) top_row_color: Colour,
    pub(crate) bottom_row_text: DisplayText,
    pub(crate) bottom_row_color: Colour,
}

pub type DisplayStateUpdateChannel =
    Channel<CriticalSectionRawMutex, DisplayStateUpdateMessage, 16>;

pub enum ButtonEvent {
    Pressed { button_index: i8 },
    Released { button_index: i8 },
}

pub type ButtonEventChannel = Channel<CriticalSectionRawMutex, ButtonEvent, 16>;

// TODO: Add channel for state updates
// TODO: Add channel for button events
