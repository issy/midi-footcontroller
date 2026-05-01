use foundation::Convertible;
use foundation::storage::state::{Presets, StoredPreset};
use foundation::storage::{StorageManager, StorageManagerLoadError, StorageManagerSaveError};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::vec::Vec;
use web_sys::Storage;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
enum MidiCommand {
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

impl Convertible<foundation::storage::state::ButtonType> for ButtonType {
    fn to(self) -> foundation::storage::state::ButtonType {
        match self {
            ButtonType::Momentary => foundation::storage::state::ButtonType::Momentary,
            ButtonType::Toggle => foundation::storage::state::ButtonType::Toggle,
        }
    }

    fn from(value: foundation::storage::state::ButtonType) -> Self {
        match value {
            foundation::storage::state::ButtonType::Momentary => ButtonType::Momentary,
            foundation::storage::state::ButtonType::Toggle => ButtonType::Toggle,
        }
    }
}

impl Convertible<foundation::protocol::Colour> for Colour {
    fn to(self) -> foundation::protocol::Colour {
        match self {
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

    fn from(value: foundation::protocol::Colour) -> Self {
        match value {
            foundation::protocol::Colour::Red => Colour::Red,
            foundation::protocol::Colour::Green => Colour::Green,
            foundation::protocol::Colour::Blue => Colour::Blue,
            foundation::protocol::Colour::Yellow => Colour::Yellow,
            foundation::protocol::Colour::Orange => Colour::Orange,
            foundation::protocol::Colour::Purple => Colour::Purple,
            foundation::protocol::Colour::Cyan => Colour::Cyan,
            foundation::protocol::Colour::White => Colour::White,
        }
    }
}

impl Convertible<foundation::storage::state::MidiCommand> for MidiCommand {
    fn to(self) -> foundation::storage::state::MidiCommand {
        match self {
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

    fn from(value: foundation::storage::state::MidiCommand) -> Self {
        match value {
            foundation::storage::state::MidiCommand::ProgramChange { channel, program } => {
                MidiCommand::ProgramChange { channel, program }
            }
            foundation::storage::state::MidiCommand::ControllerChange {
                channel,
                controller,
                value,
            } => MidiCommand::ControllerChange {
                channel,
                controller,
                value,
            },
            foundation::storage::state::MidiCommand::NoteOn {
                channel,
                note,
                velocity,
            } => MidiCommand::NoteOn {
                channel,
                note,
                velocity,
            },
            foundation::storage::state::MidiCommand::NoteOff {
                channel,
                note,
                velocity,
            } => MidiCommand::NoteOff {
                channel,
                note,
                velocity,
            },
        }
    }
}

impl Convertible<foundation::storage::state::ButtonConfig> for ButtonConfig {
    fn to(self) -> foundation::storage::state::ButtonConfig {
        foundation::storage::state::ButtonConfig {
            name: heapless::String::from_str(self.name.as_str()).unwrap(),
            button_type: self.button_type.to(),
            colour: self.colour.to(),
            on_actions: heapless::Vec::from_iter(
                self.on_actions.iter().map(|m| m.to()).collect::<Vec<_>>(),
            ),
            off_actions: heapless::Vec::from_iter(
                self.off_actions.iter().map(|m| m.to()).collect::<Vec<_>>(),
            ),
        }
    }

    fn from(value: foundation::storage::state::ButtonConfig) -> Self {
        ButtonConfig {
            name: value.name.to_string(),
            button_type: Convertible::from(value.button_type),
            colour: Convertible::from(value.colour),
            on_actions: value
                .on_actions
                .into_iter()
                .map(|m| Convertible::from(m))
                .collect(),
            off_actions: value
                .off_actions
                .into_iter()
                .map(|m| Convertible::from(m))
                .collect(),
        }
    }
}

impl Convertible<StoredPreset> for Preset {
    fn to(self) -> StoredPreset {
        StoredPreset {
            name: heapless::String::from_str(self.name.as_str()).unwrap(),
            buttons: heapless::Vec::from_iter(
                self.buttons
                    .iter()
                    .map(|b| b.clone().to())
                    .collect::<Vec<_>>(),
            ),
        }
    }

    fn from(value: StoredPreset) -> Self {
        Preset {
            name: value.name.to_string(),
            buttons: value
                .buttons
                .into_iter()
                .map(|b| Convertible::from(b))
                .collect(),
        }
    }
}

pub struct LocalStorageManager<'a> {
    local_storage: &'a mut Storage,
}

impl<'a> LocalStorageManager<'a> {
    pub fn new(local_storage: &'a mut Storage) -> Self {
        LocalStorageManager { local_storage }
    }
}

const STORAGE_KEY_PRESETS: &str = "presets";
const STORAGE_KEY_PRESET_ID: &str = "preset_id";

impl StorageManager for LocalStorageManager<'_> {
    fn load_presets(&self) -> Result<Presets, StorageManagerLoadError> {
        let value = self
            .local_storage
            .get_item(STORAGE_KEY_PRESETS)
            .map_err(|_| StorageManagerLoadError::ErrorReadingFromStorage)?
            .or_else(|| Some("[]".to_string()))
            .unwrap();

        let deserialized: Vec<Preset> = serde_json::from_slice(value.as_bytes())
            .map_err(|_| StorageManagerLoadError::ErrorDeserializingData)?;
        let mapped = deserialized.into_iter().map(|p| p.to()).collect();
        Ok(mapped)
    }

    fn save_presets(&mut self, presets: &Presets) -> Result<(), StorageManagerSaveError> {
        let mapped: Vec<Preset> = presets
            .into_iter()
            .map(|p| Convertible::from(p.clone()))
            .collect();
        let serialized = serde_json::to_string(&mapped)
            .map_err(|_| StorageManagerSaveError::ErrorDeserializingData)?;

        self.local_storage
            .set_item(STORAGE_KEY_PRESETS, serialized.as_str())
            .map_err(|_| StorageManagerSaveError::ErrorWritingToStorage)
    }
}
