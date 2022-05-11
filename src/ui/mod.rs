use crate::ui::app::Onagre;
use crate::THEME;
use iced::{window, Application, Settings};
use log::debug;

pub mod app;
pub mod mode;
pub mod plugin_matchers;
pub mod state;
pub mod style;
pub mod subscriptions;

pub fn run() -> iced::Result {
    debug!("Starting Onagre in debug mode");

    let default_font = THEME.font.as_ref().and_then(|font| style::font::load(font));

    Onagre::run(Settings {
        id: Some("onagre".to_string()),
        window: window::Settings {
            transparent: true,
            size: THEME.size,
            decorations: false,
            always_on_top: true,
            resizable: false,
            position: window::Position::Centered,
            ..Default::default()
        },
        default_text_size: THEME.font_size,
        antialiasing: true,
        default_font,
        ..Default::default()
    })
}
