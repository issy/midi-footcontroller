use foundation::midi::{MidiPacket, MidiReader, MidiWriter};

#[derive(Default)]
pub struct FakeMidiWriter;

#[derive(Debug)]
pub enum MyError {
    Foo,
}

impl MidiWriter for FakeMidiWriter {
    type Error = MyError;

    async fn write_midi_packet(&mut self, packet: &MidiPacket) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[derive(Default)]
pub struct FakeMidiReader;

impl MidiReader for FakeMidiReader {
    type Error = MyError;

    async fn read_midi_packet(&mut self) -> Result<Option<MidiPacket>, Self::Error> {
        Ok(None)
    }
}
