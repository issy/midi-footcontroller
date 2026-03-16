use foundation::storage::state::Presets;
use foundation::storage::{StorageManager, StorageManagerLoadError, StorageManagerSaveError};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::vec::Vec;
use web_sys::Storage;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
enum ButtonType {
    Momentary,
    Toggle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
enum Colour {
    Red,
    Green,
    Blue,
    Yellow,
    Orange,
    Purple,
    Cyan,
    White,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ButtonConfig {
    name: String,
    button_type: ButtonType,
    colour: Colour,
    on_actions: Vec<MidiCommand>,
    off_actions: Vec<MidiCommand>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Preset {
    name: String,
    buttons: Vec<ButtonConfig>,
}

impl From<ButtonType> for foundation::storage::state::ButtonType {
    fn from(value: ButtonType) -> Self {
        match value {
            ButtonType::Momentary => foundation::storage::state::ButtonType::Momentary,
            ButtonType::Toggle => foundation::storage::state::ButtonType::Toggle,
        }
    }
}

impl From<Colour> for foundation::protocol::Colour {
    fn from(value: Colour) -> Self {
        match value {
            Colour::Red => foundation::protocol::Colour::Red,
            Colour::Green => foundation::protocol::Colour::Green,
            Colour::Blue => foundation::protocol::Colour::Blue,
            Colour::Yellow => foundation::protocol::Colour::Yellow,
            Colour::Orange => foundation::protocol::Colour::Orange,
            Colour::Purple => foundation::protocol::Colour::Purple,
            Colour::Cyan => foundation::protocol::Colour::Cyan,
            Colour::White => foundation::protocol::Colour::White,
        }
    }
}

impl From<MidiCommand> for foundation::storage::state::MidiCommand {
    fn from(value: MidiCommand) -> Self {
        match value {
            MidiCommand::ProgramChange { channel, program } => {
                foundation::storage::state::MidiCommand::ProgramChange { channel, program }
            }
            MidiCommand::ControllerChange {
                channel,
                controller,
                value,
            } => foundation::storage::state::MidiCommand::ControllerChange {
                channel,
                controller,
                value,
            },
            MidiCommand::NoteOn {
                channel,
                note,
                velocity,
            } => foundation::storage::state::MidiCommand::NoteOn {
                channel,
                note,
                velocity,
            },
            MidiCommand::NoteOff {
                channel,
                note,
                velocity,
            } => foundation::storage::state::MidiCommand::NoteOff {
                channel,
                note,
                velocity,
            },
        }
    }
}

impl From<ButtonConfig> for foundation::storage::state::ButtonConfig {
    fn from(value: ButtonConfig) -> Self {
        foundation::storage::state::ButtonConfig {
            name: heapless::String::from_str(value.name.as_str()).unwrap(),
            button_type: value.button_type.into(),
            colour: value.colour.into(),
            on_actions: heapless::Vec::from_iter(
                value
                    .on_actions
                    .iter()
                    .map(|m| m.clone().into())
                    .collect::<Vec<_>>(),
            ),
            off_actions: heapless::Vec::from_iter(
                value
                    .off_actions
                    .iter()
                    .map(|m| m.clone().into())
                    .collect::<Vec<_>>(),
            ),
        }
    }
}

impl From<Preset> for foundation::storage::state::StoredPreset {
    fn from(value: Preset) -> Self {
        foundation::storage::state::StoredPreset {
            name: heapless::String::from_str(value.name.as_str()).unwrap(),
            buttons: heapless::Vec::from_iter(
                value
                    .buttons
                    .iter()
                    .map(|b| b.clone().into())
                    .collect::<Vec<_>>(),
            ),
        }
    }
}

pub struct LocalStorageManager<'a> {
    local_storage: &'a mut Storage,
}

const STORAGE_KEY_PRESETS: &str = "presets";
const STORAGE_KEY_PRESET_ID: &str = "preset_id";

impl StorageManager for LocalStorageManager<'_> {
    fn load_presets(&self) -> Result<Presets, StorageManagerLoadError> {
        let value = self
            .local_storage
            .get_item(STORAGE_KEY_PRESETS)
            .map_err(|_| StorageManagerLoadError::ErrorReadingFromStorage)?
            .ok_or(StorageManagerLoadError::NoValueStored)?;

        let presets: Vec<Preset> = serde_json::from_slice(value.as_bytes())
            .map_err(|_| StorageManagerLoadError::ErrorDeserializingData)?;
        let val = presets.into_iter().map(|p| p.into()).collect();
        Ok(val)
    }

    fn save_presets(&mut self, presets: &Presets) -> Result<(), StorageManagerSaveError> {
        todo!()
    }
}
