#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate anyhow;

use clap::{App, Arg};

mod app;
mod config;
mod db;
pub mod entries;
mod freedesktop;
mod style;
mod subscriptions;

use crate::config::OnagreSettings;
use crate::style::theme::Theme;
use freedesktop::IconFinder;
use std::path::PathBuf;
use std::sync::Mutex;

lazy_static! {
    pub static ref SETTINGS_PATH: Mutex<PathBuf> = {
        let path = dirs::config_dir()
            .ok_or_else(|| anyhow!("Config not found"))
            .map(|path| path.join("onagre").join("config.toml"))
            .unwrap();

        Mutex::new(path)
    };
    pub static ref THEME_PATH: Mutex<PathBuf> = {
        let path = dirs::config_dir()
            .ok_or_else(|| anyhow!("Theme config not found"))
            .map(|path| path.join("onagre").join("settings.toml"))
            .unwrap();

        Mutex::new(path)
    };
    pub static ref THEME: Theme = Theme::load();
    pub static ref SETTINGS: OnagreSettings = {
        match OnagreSettings::get() {
            Err(err) => {
                error!("Unable to load config file : {:?}", err);
                OnagreSettings::default()
            }
            Ok(settings) => settings,
        }
    };
    pub static ref ICON_FINDER: Option<IconFinder> = {
        SETTINGS
            .icons
            .as_ref()
            .map(|theme_name| IconFinder::build(theme_name).ok())
            .flatten()
    };
}

pub fn main() -> iced::Result {
    env_logger::init();

    let default_config_path = dirs::config_dir().unwrap().join("onagre");
    let default_settings = default_config_path.join("config.toml");
    let default_settings = default_settings.to_str().unwrap();

    let default_theme = default_config_path.join("theme.toml");
    let default_theme = default_theme.to_str().unwrap();

    let matches = App::new("Onagre")
        .setting(clap::AppSettings::ColorAuto)
        .setting(clap::AppSettings::ColoredHelp)
        .version(env!("CARGO_PKG_VERSION"))
        .author("Paul Delafosse <paul.delafosse@protonmail.com>")
        .about("A general purpose app launcher for wayland and X")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .takes_value(true)
                .help("Path to an alternate onagre config file")
                .default_value(default_settings),
        )
        .arg(
            Arg::with_name("theme")
                .short("t")
                .long("theme")
                .takes_value(true)
                .help("Path to an alternate onagre theme file")
                .default_value(default_theme),
        )
        .get_matches();

    // User defined config, $XDG_CONFIG_DIR/onagre/theme.toml otherwise
    if let Some(config_path) = matches.value_of("config") {
        let path = PathBuf::from(config_path).canonicalize();
        if let Ok(path) = path {
            *SETTINGS_PATH.lock().unwrap() = path;
        }

        debug!(
            "Using alternate config : {:?}",
            SETTINGS_PATH.lock().unwrap()
        );
    }

    // User defined theme config, $XDG_CONFIG_DIR/onagre/theme.toml otherwise
    if let Some(theme_path) = matches.value_of("theme") {
        let path = PathBuf::from(theme_path).canonicalize();
        if let Ok(path) = path {
            *THEME_PATH.lock().unwrap() = path;
        }

        debug!("Using alternate theme : {:?}", THEME_PATH.lock().unwrap());
    }

    app::run()
}
