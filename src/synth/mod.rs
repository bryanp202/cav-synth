mod midi;

use crate::audio;
use crate::audio::module::{ModuleMessage, ModuleMessageUnion};
use crate::audio::module::midi::MidiUpdate;

use iced::{window, Element, Length, Subscription, Task};
use iced::futures::channel::mpsc::Sender;
use iced::widget::{button, column, row, slider};


#[derive(Clone, Debug)]
pub enum Message {
    AudioThreadReady(Sender<audio::Input>),
    Close(window::Id),
    ComponentChange(usize, ModuleMessage),
    MidiThreadReady(Sender<midi::Input>),

    // MIDI
    KeyPress(u8, u8),
    KeyRelease(u8),
    PedalPress,
    PedalRelease,

    // Testing
    DelaySlider(f32),
}

pub struct Synth {
    audio_thread_connection: Option<Sender<audio::Input>>,
    midi_thread_connection: Option<Sender<midi::Input>>,

    // Testing
    delay_slider_value: f32,
}

impl Synth {
    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                audio_thread_connection: None,
                midi_thread_connection: None,

                // Testing
                delay_slider_value: 100.0,
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
                if let Some(connection) = &mut self.midi_thread_connection {
                    let _ = connection.try_send(midi::Input::Close).map_err(|err| println!("{err:?}"));
                }
                window::close(id)
            },
            Message::ComponentChange(id, input) => {
                if let Some(connection) = &mut self.audio_thread_connection {
                    let _ = connection.try_send(audio::Input::ModuleMessage(id, input));
                }
                Task::none()
            },
            Message::MidiThreadReady(connection) => {
                self.midi_thread_connection = Some(connection);
                Task::none()
            },

            // MIDI
            Message::KeyPress(note, velocity) => {
                if let Some(connection) = &mut self.audio_thread_connection {
                    let input = ModuleMessage::ComponentChange(ModuleMessageUnion {midi: MidiUpdate::KeyPress(note, velocity)});
                    let _ = connection.try_send(audio::Input::ModuleMessage(0, input));
                }
                Task::none()
            },
            Message::KeyRelease(note) => {
                if let Some(connection) = &mut self.audio_thread_connection {
                    let input = ModuleMessage::ComponentChange(ModuleMessageUnion {midi: MidiUpdate::KeyRelease(note)});
                    let _ = connection.try_send(audio::Input::ModuleMessage(0, input));
                }
                Task::none()
            },
            Message::PedalPress => {
                if let Some(connection) = &mut self.audio_thread_connection {
                    let input = ModuleMessage::ComponentChange(ModuleMessageUnion {midi: MidiUpdate::PedalPress});
                    let _ = connection.try_send(audio::Input::ModuleMessage(0, input));
                }
                Task::none()
            },
            Message::PedalRelease => {
                if let Some(connection) = &mut self.audio_thread_connection {
                    let input = ModuleMessage::ComponentChange(ModuleMessageUnion {midi: MidiUpdate::PedalRelease});
                    let _ = connection.try_send(audio::Input::ModuleMessage(0, input));
                }
                Task::none()
            },

            // Testing
            Message::DelaySlider(time) => {
                if let Some(connection) = &mut self.audio_thread_connection {
                    let input = ModuleMessage::ComponentChange(ModuleMessageUnion {delay: audio::module::delay::DelayUpdate::Time(time / 100.0)});
                    let _ = connection.try_send(audio::Input::ModuleMessage(15, input));
                }
                self.delay_slider_value = time;
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        column![
            row![
                button("Sine!")
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .on_press(Message::ComponentChange(
                        1,
                        ModuleMessage::ComponentChange(
                            ModuleMessageUnion {analog: audio::module::analog::AnalogOscillatorUpdate::Shape(audio::module::analog::WaveShape::Sine)}
                        ),
                    )),
                button("Saw!")
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .on_press(Message::ComponentChange(
                        1,
                        ModuleMessage::ComponentChange(
                            ModuleMessageUnion {analog: audio::module::analog::AnalogOscillatorUpdate::Shape(audio::module::analog::WaveShape::Saw)}
                        ),
                    )),
            ],
            row![
                button("Square!")
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .on_press(Message::ComponentChange(
                        1,
                        ModuleMessage::ComponentChange(
                            ModuleMessageUnion {analog: audio::module::analog::AnalogOscillatorUpdate::Shape(audio::module::analog::WaveShape::Square)}
                        ),
                    )),
                button("Triangle!")
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .on_press(Message::ComponentChange(
                        1,
                        ModuleMessage::ComponentChange(
                            ModuleMessageUnion {analog: audio::module::analog::AnalogOscillatorUpdate::Shape(audio::module::analog::WaveShape::Triangle)}
                        ),
                    )),
            ],
            slider(0.0..=600.0, self.delay_slider_value, Message::DelaySlider)
                .width(Length::Fill),
        ].into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(
            [
                Subscription::run(audio::render_audio),
                Subscription::run(midi::listen),
                window::close_requests().map(Message::Close),
            ]
        )
    }
}
