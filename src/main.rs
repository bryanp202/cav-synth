mod synth;
mod audio;

use synth::Synth;

use iced::window::Settings;

fn main() -> iced::Result {
    let settings = Settings {
        exit_on_close_request: false,
        ..Default::default()
    };
    iced::application(Synth::title, Synth::update, Synth::view)
        .window(settings)
        .subscription(Synth::subscription)
        .run_with(Synth::new)
}
