use alloc::string::String;
use alloc::vec::Vec;

use crate::FIRMWARE_VERSION;
use crate::generated::device_v1 as pb;

const PROTOCOL_VERSION: u32 = 1;

#[derive(Debug)]
pub struct ButtonEvent {
    pub button_id: u8,
    pub pressed: bool,
}

#[derive(Debug)]
pub struct Capabilities {
    button_count: u32,
    display_count: u32,
    preset_capacity: u32,
    supports_firmware_update: bool,
    supports_live_preview: bool,
}

#[derive(Debug)]
pub struct MidiCommand {
    placeholder: String,
}

#[derive(Debug)]
pub struct ButtonConfig {
    pub text: String,
    pub colour: u32,
    pub commands: Vec<MidiCommand>,
}

#[derive(Debug)]
pub struct DeviceConfig {
    pub buttons: Vec<ButtonConfig>,
}

#[derive(Debug)]
pub enum Message {
    // Common
    Ready,
    Ack(u32),
    // Device -> Client
    Button(ButtonEvent),
    Preset(DeviceConfig),
    Hello {
        device_model: String,
        capabilities: Capabilities,
    },
    Error(String),
    // Client -> Device
    RequestPresets,
    ClientHello {
        supported_protocol_version: u8,
        client_name: String,
        client_version: String,
    },
}

impl Message {
    pub fn to_protobuf(&self) -> pb::Envelope {
        match self {
            Message::Button(btn) => pb::Envelope {
                protocol_version: PROTOCOL_VERSION,
                payload: Some(pb::envelope::Payload::ButtonEvent(pb::ButtonEvent {
                    button_id: btn.button_id as u32,
                    pressed: btn.pressed,
                })),
            },
            Message::Hello {
                device_model,
                capabilities,
            } => pb::Envelope {
                protocol_version: PROTOCOL_VERSION,
                payload: Some(pb::envelope::Payload::Hello(pb::Hello {
                    firmware_version_major: FIRMWARE_VERSION.0,
                    firmware_version_minor: FIRMWARE_VERSION.1,
                    firmware_version_patch: FIRMWARE_VERSION.2,
                    device_model: device_model.clone(),
                    capabilities: Some(pb::Capabilities {
                        button_count: capabilities.button_count,
                        display_count: capabilities.display_count,
                        preset_capacity: capabilities.preset_capacity,
                        supports_firmware_update: capabilities.supports_firmware_update,
                        supports_live_preview: capabilities.supports_live_preview,
                    }),
                    hardware_revision: 0,
                })),
            },
            Message::Ready => pb::Envelope {
                protocol_version: PROTOCOL_VERSION,
                payload: Some(pb::envelope::Payload::Ready(pb::Ready {})),
            },
            _ => todo!(),
        }
    }

    pub fn from_protobuf(envelope: pb::Envelope) -> Option<Self> {
        match envelope.payload {
            Some(pb::envelope::Payload::ClientHello(h)) => Some(Message::ClientHello {
                supported_protocol_version: h.supported_protocol_version as u8,
                client_version: h.client_version,
                client_name: h.client_name,
            }),
            Some(pb::envelope::Payload::RequestPresets(_)) => Some(Message::RequestPresets),
            Some(pb::envelope::Payload::Ack(ack)) => Some(Message::Ack(ack.id)),
            _ => None,
        }
    }
}
