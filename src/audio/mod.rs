mod module;
mod sample;
mod table;

use std::time::Duration;

use crate::synth::Message;

use iced::futures::{SinkExt, Stream};
use iced::stream;
use iced::futures::channel::mpsc::{self as iced_mpsc, Receiver};
use module::{ModuleId, ModuleMessage, ModuleMessageUnion};
use table::ModTable;

#[derive(Clone, Debug)]
pub enum Input {
    Close,
    UpdateSampleRate(usize),
    ModuleMessage(ModuleId, ModuleMessage),
}

struct AudioState {
    sample_rate: usize,
    table: ModTable,
}

impl AudioState {
    const DEFAULT_SAMPLE_RATE: usize = 48000;

    fn new() -> Self {
        Self {
            sample_rate: Self::DEFAULT_SAMPLE_RATE,
            table: ModTable::new(),
        }
    }

    fn update(&mut self, receiver: &mut Receiver<Input>) {
        if let Ok(Some(input)) = receiver.try_next() {
            match input {
                Input::Close => std::process::exit(0),
                Input::UpdateSampleRate(sample_rate) => self.sample_rate = sample_rate,
                Input::ModuleMessage(id, msg) => {
                    self.table.update(id, msg);
                }
            }
        }
    }

    fn render(&mut self) {
        let out = self.table.process();
        println!("{out:?}");
    }
}

pub fn render_audio() -> impl Stream<Item = Message> {
    stream::channel(100, |mut output| async move {
        let (sender, mut receiver) = iced_mpsc::channel(100);

        output.send(Message::AudioThreadReady(sender)).await.expect("Failed to intialize audio thread");
        tokio::time::sleep(Duration::from_secs(1)).await;

        let mut state = AudioState::new();

        loop {
            state.render();
            state.update(&mut receiver);
        }
    })
}
