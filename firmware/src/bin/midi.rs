#[derive(Copy, Clone, Debug)]
pub enum NoteName {
    C,
    Cs,
    D,
    Ds,
    E,
    F,
    Fs,
    G,
    Gs,
    A,
    As,
    B,
}

impl MidiNote {
    pub fn from_name(name: NoteName, octave: i8) -> Self {
        let base = match name {
            NoteName::C => 0,
            NoteName::Cs => 1,
            NoteName::D => 2,
            NoteName::Ds => 3,
            NoteName::E => 4,
            NoteName::F => 5,
            NoteName::Fs => 6,
            NoteName::G => 7,
            NoteName::Gs => 8,
            NoteName::A => 9,
            NoteName::As => 10,
            NoteName::B => 11,
        };
        let note = 12 * (octave + 1) + base; // MIDI octave numbering: C4 = 60
        MidiNote::new(note as i16)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct MidiNote(u8); // internal 0–127

impl MidiNote {
    /// Create a note, clamped to valid MIDI range 0–127
    pub fn new(note: i16) -> Self {
        let clamped = note.clamp(0, 127) as u8;
        MidiNote(clamped)
    }

    /// Get the raw note number
    pub fn number(self) -> u8 {
        self.0
    }

    /// Transpose by N semitones, clamped
    pub fn transpose(self, semitones: i16) -> Self {
        MidiNote::new(self.0 as i16 + semitones)
    }

    /// Common middle C constant
    pub const fn middle_c() -> Self {
        MidiNote(60)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct MidiPacket {
    pub len: u8,       // Number of valid bytes
    pub data: [u8; 3], // MIDI messages are max 3 bytes (except SysEx)
}

impl MidiPacket {
    /// Note On message (channel 0-15)
    pub fn note_on(channel: u8, note: u8, velocity: u8) -> Self {
        MidiPacket {
            len: 3,
            data: [0x90 | (channel & 0x0F), note & 0x7F, velocity & 0x7F],
        }
    }

    /// Note Off message (channel 0-15)
    pub fn note_off(channel: u8, note: u8, velocity: u8) -> Self {
        MidiPacket {
            len: 3,
            data: [0x80 | (channel & 0x0F), note & 0x7F, velocity & 0x7F],
        }
    }

    /// Control Change (CC) message
    pub fn control_change(channel: u8, controller: u8, value: u8) -> Self {
        MidiPacket {
            len: 3,
            data: [0xB0 | (channel & 0x0F), controller & 0x7F, value & 0x7F],
        }
    }

    /// Program Change (2-byte message)
    pub fn program_change(channel: u8, program: u8) -> Self {
        MidiPacket {
            len: 2,
            data: [0xC0 | (channel & 0x0F), program & 0x7F, 0],
        }
    }

    /// Channel Pressure / Aftertouch (2-byte message)
    pub fn channel_pressure(channel: u8, pressure: u8) -> Self {
        MidiPacket {
            len: 2,
            data: [0xD0 | (channel & 0x0F), pressure & 0x7F, 0],
        }
    }

    /// Pitch Bend (3-byte message)
    pub fn pitch_bend(channel: u8, value: u16) -> Self {
        let lsb = (value & 0x7F) as u8;
        let msb = ((value >> 7) & 0x7F) as u8;
        MidiPacket {
            len: 3,
            data: [0xE0 | (channel & 0x0F), lsb, msb],
        }
    }

    /// Real-Time messages (1-byte)
    pub fn timing_clock() -> Self {
        MidiPacket {
            len: 1,
            data: [0xF8, 0, 0],
        }
    }
    pub fn start() -> Self {
        MidiPacket {
            len: 1,
            data: [0xFA, 0, 0],
        }
    }
    pub fn stop() -> Self {
        MidiPacket {
            len: 1,
            data: [0xFC, 0, 0],
        }
    }
}

pub struct MidiParser {
    running_status: Option<u8>,
    data: [u8; 2], // store data bytes
    index: usize,  // number of data bytes collected
}

impl MidiParser {
    pub const fn new() -> Self {
        Self {
            running_status: None,
            data: [0; 2],
            index: 0,
        }
    }

    // Feed a single byte; returns Some(MidiPacket) when a full message is ready
    pub fn feed(&mut self, byte: u8) -> Option<MidiPacket> {
        // Real-time messages (0xF8-0xFF) are single-byte and can appear anytime
        if byte >= 0xF8 {
            return Some(MidiPacket {
                len: 1,
                data: [byte, 0, 0],
            });
        }

        // Status byte (>= 0x80, < 0xF0)
        if byte & 0x80 != 0 {
            self.running_status = Some(byte);
            self.index = 0;
            return None; // wait for data bytes
        }

        // Must have a running status to interpret data bytes
        let status = self.running_status?;
        let needed_bytes = match status & 0xF0 {
            0xC0 | 0xD0 => 1, // Program Change, Channel Pressure
            _ => 2,           // Note On/Off, CC, Pitch Bend
        };

        self.data[self.index] = byte;
        self.index += 1;

        if self.index >= needed_bytes {
            // Construct packet
            let mut packet = MidiPacket {
                len: 0,
                data: [0; 3],
            };
            packet.data[0] = status;
            packet.data[1..1 + needed_bytes].copy_from_slice(&self.data[..needed_bytes]);
            packet.len = (1 + needed_bytes) as u8;
            self.index = 0;
            return Some(packet);
        }

        None
    }
}
