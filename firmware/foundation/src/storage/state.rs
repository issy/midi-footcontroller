use crate::midi::MidiPacket;
use crate::protocol::Colour;
use heapless::{String, Vec};

pub const MAX_PRESETS: usize = 128;
pub const MAX_STRING_LENGTH: usize = 16;
pub const NUM_OF_BUTTONS: usize = 8;
pub const MAX_BUTTON_ACTIONS: usize = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MidiCommand {
    ProgramChange {
        channel: u8,
        program: u8,
    },
    ControllerChange {
        channel: u8,
        controller: u8,
        value: u8,
    },
    NoteOn {
        channel: u8,
        note: u8,
        velocity: u8,
    },
    NoteOff {
        channel: u8,
        note: u8,
        velocity: u8,
    },
}

impl Into<MidiPacket> for MidiCommand {
    fn into(self) -> MidiPacket {
        match self {
            MidiCommand::ProgramChange { channel, program } => {
                MidiPacket::program_change(channel, program)
            }
            MidiCommand::ControllerChange {
                channel,
                controller,
                value,
            } => MidiPacket::control_change(channel, controller, value),
            MidiCommand::NoteOn {
                channel,
                note,
                velocity,
            } => MidiPacket::note_on(channel, note, velocity),
            MidiCommand::NoteOff {
                channel,
                note,
                velocity,
            } => MidiPacket::note_off(channel, note, velocity),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonType {
    Momentary,
    Toggle,
}

#[derive(Debug, Clone)]
pub struct ButtonConfig {
    pub name: String<MAX_STRING_LENGTH>,
    pub button_type: ButtonType,
    pub colour: Colour,
    // For momentary buttons, only on_actions are used. For toggle buttons, both on_actions and off_actions are used.
    pub on_actions: Vec<MidiCommand, MAX_BUTTON_ACTIONS>,
    pub off_actions: Vec<MidiCommand, MAX_BUTTON_ACTIONS>,
}

#[derive(Debug, Clone)]
pub struct StoredPreset {
    pub name: String<MAX_STRING_LENGTH>,
    pub buttons: Vec<ButtonConfig, NUM_OF_BUTTONS>,
}

#[derive(Debug, Clone)]
pub struct PresetsState {
    pub presets: Vec<StoredPreset, MAX_PRESETS>,
    pub current_preset_id: u8,
}
