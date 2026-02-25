#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct MidiPacket {
    pub len: u8,       // Number of valid bytes
    pub data: [u8; 3], // MIDI messages are max 3 bytes (except SysEx)
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
