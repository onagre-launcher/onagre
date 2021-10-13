#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate anyhow;

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
use structopt::StructOpt;

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

#[derive(StructOpt)]
#[structopt(name = "onagre", author = "Paul D. <paul.delafosse@protonmail.com>")]
struct Cli {
    #[structopt(
        name = "config",
        short = "c",
        long = "config",
        takes_value = true,
        help = "Path to an alternate onagre config file"
    )]
    config: Option<PathBuf>,

    #[structopt(
        name = "theme",
        short = "t",
        long = "theme",
        takes_value = true,
        help = "Path to an alternate onagre theme file"
    )]
    theme: Option<PathBuf>,
}



pub fn main() -> iced::Result {
    env_logger::init();
    let cli = Cli::from_args();
    // User defined theme config, $XDG_CONFIG_DIR/onagre/theme.toml otherwise
    if let Some(theme_path) = cli.theme {
        let path = theme_path.canonicalize();
        if let Ok(path) = path {
            *THEME_PATH.lock().unwrap() = path;
        }

        debug!("Using alternate theme : {:?}", THEME_PATH.lock().unwrap());
    }

    // User defined config, $XDG_CONFIG_DIR/onagre/theme.toml otherwise
    if let Some(config_path) = cli.config {
        let path = config_path.canonicalize();
        if let Ok(path) = path {
            *SETTINGS_PATH.lock().unwrap() = path;
        }

        debug!(
            "Using alternate config : {:?}",
            SETTINGS_PATH.lock().unwrap()
        );
    }

    app::run()
}
