use std::collections::VecDeque;

use crate::audio::module::{Module, ModuleMessage};

#[derive(Clone, Copy, Debug)]
pub enum MidiUpdate {
    KeyPress(u8, u8),
    KeyRelease(u8),
    PedalPress,
    PedalRelease,
}

const POLY_VOICE_COUNT: usize = 16;

#[derive(Clone, Copy, Default, Debug)]
struct Voice {
    pressed: bool,
    gate: f32,
    note: f32,
    velocity: f32,
    trigger: bool,
    ready: bool,
    on: bool,
}

impl Voice {
    fn new(gate: f32, note: u8, velocity: u8) -> Self {
        Self {
            pressed: true,
            gate,
            note: note as f32 / 127.0,
            velocity: velocity as f32 / 127.0,
            trigger: true,
            ready: false,
            on: true,
        }
    }
}

pub struct Midi {
    id: usize,
    gate: f32,
    trigger: bool,
    ready: bool,
    note: f32,
    velocity: f32,

    // Controls
    sustain: bool,
    pressed: bool,

    // Poly voices
    voices: [Voice; POLY_VOICE_COUNT],
    
    replace_queue: VecDeque<usize>,
}

impl Module for Midi {
    fn id(&self) -> usize {
        self.id
    }

    fn process(&mut self) {
        if self.trigger {
            self.ready = true;
            self.trigger = false;
        } else if self.ready {
            self.gate = 1.0;
            self.ready = false;
        }
        
        for voice in &mut self.voices {
            if voice.trigger {
                voice.ready = true;
                voice.trigger = false;
            } else if voice.ready {
                voice.gate = 1.0;
                voice.ready = false;
            }
        }
    }

    fn update(&mut self, msg: ModuleMessage) {
        match msg {
            ModuleMessage::ComponentChange(msg_union) => match unsafe {msg_union.midi} {
                MidiUpdate::KeyPress(note, velocity) => {
                    // Mono
                    self.pressed = true;
                    self.gate = 0.0;
                    self.trigger = true;
                    self.note = note as f32 / 127.0;
                    self.velocity = velocity as f32 / 127.0;

                    // Poly
                    let new_voice;
                    if let Some(voice) = self.voices.iter().position(|voice| !voice.on ) {
                        new_voice = voice;
                    } else {
                        new_voice = self.replace_queue.pop_front().unwrap();
                    }
                    self.voices[new_voice] = Voice::new(0.0, note, velocity);
                    self.replace_queue.push_back(new_voice);
                },
                MidiUpdate::KeyRelease(note) => {
                    let note_signal = note as f32 / 127.0;

                    if self.note == note_signal {
                        if !self.sustain {
                            self.gate = 0.0;
                            self.ready = false;
                            self.trigger = false;
                        }
                        self.pressed = false;
                    }

                    // Poly
                    for (i, voice) in self.voices.iter_mut().enumerate() {
                        if voice.note == note_signal {
                            if !self.sustain && voice.on {
                                voice.ready = false;
                                voice.trigger = false;
                                voice.gate = 0.0;
                                voice.on = false;
                                
                                let queue_pos = self.replace_queue.iter().position(|voice_num| *voice_num == i).unwrap();
                                self.replace_queue.remove(queue_pos);
                            }
                            voice.pressed = false;
                        }
                    }
                },
                MidiUpdate::PedalPress => self.sustain = true,
                MidiUpdate::PedalRelease => {
                    self.sustain = false; 
                    if !self.pressed {
                        self.gate = 0.0;
                        self.ready = false;
                        self.trigger = false;
                    }

                    // Poly
                    for (i, voice) in self.voices.iter_mut().enumerate() {
                        if voice.on && !voice.pressed {
                            voice.ready = false;
                            voice.trigger = false;
                            voice.gate = 0.0;
                            voice.on = false;
                            
                            let queue_pos = self.replace_queue.iter().position(|voice_num| *voice_num == i).unwrap();
                            self.replace_queue.remove(queue_pos);
                        }
                    }
                },
            }
        }
        // println!("{:?}", self.replace_queue);
    }

    fn get_output(&self, target_output: usize) -> f32 {
        match target_output {
            0 => self.gate,
            1 => self.note,
            2 => self.velocity,
            _ => {
                let voice = (target_output - 3) / 3;
                let data_type = target_output % 3;

                match data_type {
                    0 => self.voices[voice].gate,
                    1 => self.voices[voice].note,
                    2 => self.voices[voice].velocity,
                    _ => unreachable!(),
                }
            },
        }
    }

    fn modulate(&mut self, component: usize, value: f32) {}
}

impl Midi {
    pub fn new(id: usize) -> Midi {
        Self {
            id,
            gate: 0.0,
            trigger: false,
            ready: false,
            note: 0.0,
            velocity: 0.0,
            sustain: false,
            pressed: false,

            voices: [Voice::default(); POLY_VOICE_COUNT],
            replace_queue: VecDeque::with_capacity(POLY_VOICE_COUNT),
        }
    }
}