use crate::application::channels::MidiOutChannel;
use crate::midi::MidiReader;

async fn midi_thru_task<MR: MidiReader>(mut reader: MR, midi_out_channel: MidiOutChannel) -> ! {
    loop {
        if let Some(packet) = reader.read_midi_packet().await.unwrap() {
            midi_out_channel.send(packet).await;
        }
    }
}
