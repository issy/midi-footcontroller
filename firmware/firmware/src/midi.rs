use esp_hal::Async;
use esp_hal::uart::{RxError, TxError, UartRx, UartTx};
use foundation::midi::{MidiPacket, MidiParser, MidiReader, MidiWriter};

pub(crate) struct UartMidiReader<'a, 'b> {
    uart: &'a mut UartRx<'b, Async>,
    parser: MidiParser,
}

impl<'a, 'b> UartMidiReader<'a, 'b> {
    fn new(uart: &'a mut UartRx<'b, Async>) -> Self {
        Self {
            uart,
            parser: MidiParser::default(),
        }
    }
}

impl<'a, 'b> MidiReader for UartMidiReader<'a, 'b> {
    type Error = RxError;

    async fn read_midi_packet(&mut self) -> Result<Option<MidiPacket>, Self::Error> {
        let mut buf = [0u8; 1];
        self.uart.read_async(&mut buf).await?;

        Ok(self.parser.feed(buf[0]))
    }
}

pub(crate) struct UartMidiWriter<'a, 'b> {
    uart: &'a mut UartTx<'b, Async>,
}

impl<'a, 'b> UartMidiWriter<'a, 'b> {
    fn new(uart: &'a mut UartTx<'b, Async>) -> Self {
        Self { uart }
    }
}

impl<'a, 'b> MidiWriter for UartMidiWriter<'a, 'b> {
    type Error = TxError;

    async fn write_midi_packet(&mut self, packet: &MidiPacket) -> Result<(), Self::Error> {
        self.uart
            .write_async(&packet.data[..packet.len as usize])
            .await?;
        Ok(())
    }
}
