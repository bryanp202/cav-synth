use crate::audio;

use iced::{Task, Element, Subscription, window};
use iced::futures::channel::mpsc::Sender;
use iced::widget;


#[derive(Clone, Debug)]
pub enum Message {
    AudioThreadReady(Sender<audio::Input>),
    Close(window::Id),
}

pub struct Synth {
    audio_thread_connection: Option<Sender<audio::Input>>,
}

impl Synth {
    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                audio_thread_connection: None,
            },
            Task::none()
        )
    }

    pub fn title(&self) -> String {
        String::from("CavSynth")
    }

    pub fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {
            Message::AudioThreadReady(connection) => {
                self.audio_thread_connection = Some(connection);
                Task::none()
            },
            Message::Close(id) => {
                if let Some(connection) = &mut self.audio_thread_connection {
                    let _ = connection.try_send(audio::Input::Close).map_err(|err| println!("{err:?}"));
                }
                window::close(id)
            },
        }
    }

    pub fn view(&self) -> Element<Message> {
        widget::button("Hello World!").into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(
            [
                Subscription::run(audio::render_audio),
                window::close_requests().map(Message::Close),
            ]
        )
    }
}
