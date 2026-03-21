use crate::generated::device_v1 as pb;
use crate::generated::device_v1::Envelope;
use crate::midi::MidiPacket;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt::Debug;
use serde::{Deserialize, Serialize};

const PROTOCOL_VERSION: u32 = 1;

#[derive(Debug)]
pub struct ButtonEvent {
    button_id: u8,
    pressed: bool,
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
    // TODO: Fix
    placeholder: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Colour {
    Red = 1,
    Green = 2,
    Blue = 3,
    Yellow = 4,
    Orange = 5,
    Purple = 6,
    Cyan = 7,
    White = 8,
}

impl From<Option<Colour>> for pb::Colour {
    fn from(colour: Option<Colour>) -> Self {
        match colour {
            Some(Colour::Red) => pb::Colour::Red,
            Some(Colour::Green) => pb::Colour::Green,
            Some(Colour::Blue) => pb::Colour::Blue,
            Some(Colour::Yellow) => pb::Colour::Yellow,
            Some(Colour::Orange) => pb::Colour::Orange,
            Some(Colour::Purple) => pb::Colour::Purple,
            Some(Colour::Cyan) => pb::Colour::Cyan,
            Some(Colour::White) => pb::Colour::White,
            None => pb::Colour::Unspecified,
        }
    }
}

#[derive(Debug)]
pub struct ButtonConfig {
    text: String,
    colour: Option<Colour>,
    commands: Vec<MidiCommand>,
}

#[derive(Debug)]
pub struct DevicePreset {
    preset_id: u8,
    buttons: Vec<ButtonConfig>,
}

#[derive(Debug)]
pub enum Message {
    // Common
    Ready,
    Ack(u32),
    // Device -> Client
    Button(ButtonEvent),
    PresetChange(u8),
    ListPresets(Vec<DevicePreset>),
    Hello {
        device_model: String,
        capabilities: Capabilities,
        firmware_version: (u32, u32, u32),
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
    // Device -> Client
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
                firmware_version:
                    (firmware_version_major, firmware_version_minor, firmware_version_patch),
            } => pb::Envelope {
                protocol_version: PROTOCOL_VERSION,
                payload: Some(pb::envelope::Payload::Hello(pb::Hello {
                    firmware_version_major: *firmware_version_major,
                    firmware_version_minor: *firmware_version_minor,
                    firmware_version_patch: *firmware_version_patch,
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
            Message::PresetChange(preset_id) => pb::Envelope {
                protocol_version: PROTOCOL_VERSION,
                payload: Some(pb::envelope::Payload::PresetChange(pb::PresetChange {
                    preset_id: *preset_id as u32,
                })),
            },
            Message::ListPresets(presets) => pb::Envelope {
                protocol_version: PROTOCOL_VERSION,
                payload: Some(pb::envelope::Payload::AllPresets(pb::Presets {
                    presets: presets
                        .iter()
                        .map(|preset| pb::DevicePreset {
                            id: preset.preset_id as u32,
                            buttons: preset
                                .buttons
                                .iter()
                                .map(|btn| pb::ButtonConfig {
                                    text: btn.text.clone(),
                                    colour: Into::<pb::Colour>::into(btn.colour.clone()) as i32,
                                    commands: btn
                                        .commands
                                        .iter()
                                        .map(|cmd| pb::MidiCommand {
                                            placeholder: cmd.placeholder.clone(),
                                        })
                                        .collect(),
                                })
                                .collect(),
                        })
                        .collect(),
                })),
            },
            _ => todo!(),
        }
    }

    // Client -> Device
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

enum RxState {
    ReadingLength,
    ReadingPayload { len: usize, buf: Vec<u8> },
}

enum ReadError {
    ErrorReadingLength,
    ErrorReadingPayload,
}

trait ProtocolReader {
    /// Asynchronously read a payload, returning None if the stream ends
    fn read_payload(&mut self) -> impl Future<Output = Result<Option<pb::Envelope>, ReadError>>;
}

enum ParseState {
    Idle,
    ReadingLength,
    ReadingPayload,
}

struct ProtocolParser {
    state: Option<ParseState>,
    data: Vec<u8>,
    index: usize,
}

struct DefaultProtocolReader {
    parser: ProtocolParser,
}

impl ProtocolReader for DefaultProtocolReader {
    async fn read_payload(&mut self) -> Result<Option<Envelope>, ReadError> {
        Ok(None)
    }
}

/// Slop

/// CRC8 using a simple polynomial
fn crc8(data: &[u8]) -> u8 {
    let mut crc = 0u8;
    for &b in data {
        crc ^= b;
        for _ in 0..8 {
            crc = if (crc & 0x80) != 0 {
                crc << 1 ^ 0x07
            } else {
                crc << 1
            };
        }
    }
    crc
}

pub struct FrameDecoder<const N: usize> {
    state: State,
    len_buf: [u8; 2],
    len_pos: usize,
    payload: [u8; N],
    payload_pos: usize,
    expected_len: usize,
}

#[derive(Debug)]
enum State {
    Sync1,
    Sync2,
    Len,
    Payload,
    Crc,
}

impl<const N: usize> FrameDecoder<N> {
    pub const fn new() -> Self {
        Self {
            state: State::Sync1,
            len_buf: [0; 2],
            len_pos: 0,
            payload: [0; N],
            payload_pos: 0,
            expected_len: 0,
        }
    }

    /// Push a single byte
    /// Returns Some(&payload) when a full valid frame is received
    pub fn push(&mut self, byte: u8) -> Option<&[u8]> {
        match self.state {
            State::Sync1 => {
                if byte == 0xAA {
                    self.state = State::Sync2;
                }
            }
            State::Sync2 => {
                if byte == 0x55 {
                    self.state = State::Len;
                    self.len_pos = 0;
                } else {
                    // Stay in sync1 if second header byte fails
                    self.state = State::Sync1;
                }
            }
            State::Len => {
                self.len_buf[self.len_pos] = byte;
                self.len_pos += 1;
                if self.len_pos == 2 {
                    self.expected_len = u16::from_le_bytes(self.len_buf) as usize;
                    if self.expected_len == 0 || self.expected_len > N {
                        // Invalid length - resync
                        self.state = State::Sync1;
                    } else {
                        self.payload_pos = 0;
                        self.state = State::Payload;
                    }
                }
            }
            State::Payload => {
                self.payload[self.payload_pos] = byte;
                self.payload_pos += 1;
                if self.payload_pos == self.expected_len {
                    self.state = State::Crc;
                }
            }
            State::Crc => {
                let calc_crc = crc8(&self.payload[..self.expected_len]);
                if calc_crc == byte {
                    // Success
                    self.state = State::Sync1;
                    return Some(&self.payload[..self.expected_len]);
                } else {
                    // CRC failed - discard frame
                    self.state = State::Sync1;
                }
            }
        }
        None
    }
}
