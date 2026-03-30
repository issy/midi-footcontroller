use crate::application::channels::MidiOutChannel;
use crate::midi::{MidiReader, MidiWriter};

async fn midi_thru_task<MR: MidiReader>(mut reader: MR, midi_out_channel: MidiOutChannel) -> ! {
    loop {
        if let Some(packet) = reader.read_midi_packet().await.unwrap() {
            // TODO: If we decide to support MIDI command input in future, this would be a good place to process those
            midi_out_channel.send(packet).await;
        }
    }
}

async fn midi_out_task<MW: MidiWriter>(mut writer: MW, midi_out_channel: MidiOutChannel) -> ! {
    loop {
        let packet = midi_out_channel.receive().await;
        writer.write_midi_packet(&packet).await.unwrap();
    }
}
