pub mod module;
mod table;

use std::time::{Duration, Instant};

use crate::synth::Message;

use iced::futures::{SinkExt, Stream};
use iced::stream;
use iced::futures::channel::mpsc::{self as iced_mpsc, Receiver};
use module::ModuleMessage;
use rodio::buffer::SamplesBuffer;
use rodio::{OutputStream, Sink};
use table::ModTable;

#[derive(Clone, Debug)]
pub enum Input {
    Close,
    UpdateSampleRate(usize),
    ModuleMessage(usize, ModuleMessage),
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

    fn render(&mut self, mut receiver: Receiver<Input>) {
        const BUFFER_SIZE: usize = 64;

        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        
        let mut dt = Instant::now();
        let buffer_time = Duration::from_secs_f32((BUFFER_SIZE) as f32 / self.sample_rate as f32);
        let buffer_time_messages = buffer_time - Duration::from_micros(10);

        println!("{buffer_time:?}");

        loop {
            let mut buffer = Vec::with_capacity(BUFFER_SIZE);

            for _ in 0..BUFFER_SIZE {
                let sample = self.table.process();
                buffer.push(sample);
            }

            while sink.len() > 16 && dt.elapsed() < buffer_time_messages {
                self.update(&mut receiver);
            }
            while sink.len() > 16 && dt.elapsed() < buffer_time {}

            let audio_buf = SamplesBuffer::new(1, self.sample_rate as u32, buffer);

            sink.append(audio_buf);

            dt = Instant::now();
        }
    }
}

pub fn render_audio() -> impl Stream<Item = Message> {
    stream::channel(100, |mut output| async move {
        let (sender, receiver) = iced_mpsc::channel(100);

        output.send(Message::AudioThreadReady(sender)).await.expect("Failed to intialize audio thread");
        tokio::time::sleep(Duration::from_secs(1)).await;

        let mut state = AudioState::new();

        state.render(receiver);
    })
}
