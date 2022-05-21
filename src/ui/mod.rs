use crate::font::DEFAULT_FONT;
use crate::ui::app::Onagre;
use crate::{font, THEME};
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

    let default_font = THEME
        .font
        .as_ref()
        .and_then(|font| font::load(font))
        .unwrap_or(DEFAULT_FONT);

    Onagre::run(Settings {
        id: Some("onagre".to_string()),
        window: window::Settings {
            transparent: true,
            size: THEME.size,
            decorations: false,
            always_on_top: true,
            resizable: false,
            position: window::Position::Centered,
            min_size: None,
            max_size: None,
            icon: None,
        },
        default_text_size: THEME.font_size,
        text_multithreading: false,
        antialiasing: true,
        exit_on_close_request: false,
        default_font: Some(default_font),
        flags: (),
        try_opengles_first: false,
    })
}
