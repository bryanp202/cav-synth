use super::Message;

use iced::stream;
use iced::futures::channel::mpsc::{self as iced_mpsc};
use iced::futures::{SinkExt, Stream, StreamExt};

use midir::{self, MidiInput, Ignore};

pub enum Input {
    Close,
}

pub fn listen() -> impl Stream<Item = Message> {
    stream::channel(100, |mut output| async move {
        let (sender, mut receiver) = iced_mpsc::channel(100);
        let _ = output.send(Message::MidiThreadReady(sender)).await;

        let mut midi_in = MidiInput::new("cav-synth").expect("No midi found");
        midi_in.ignore(Ignore::TimeAndActiveSense);

        let in_ports = midi_in.ports();
        println!("Midi port count: {}", in_ports.len());
        if in_ports.len() < 1 {
            panic!("No midi ports found");
        }
        let in_port = &in_ports[0];

        let _conn_in = midi_in.connect(
            in_port, 
            "synth-midi", 
            move |_stamp, message, _| {
                match message[0] {
                    144 => {
                        if message[2] != 0 {
                            output.try_send(Message::KeyPress(message[1], message[2])).unwrap();
                        } else {
                            output.try_send(Message::KeyRelease(message[1])).unwrap();
                        }
                    }

                    _ => (),
                }
                
            },
            (),
        );

        loop {
            receiver.select_next_some().await;
        }
    })
}