#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate anyhow;

pub mod app;
pub mod config;
pub mod db;
pub mod entries;
pub mod freedesktop;
pub mod style;
pub mod subscriptions;

use std::sync::Mutex;
use crate::config::OnagreSettings;
use freedesktop::IconFinder;
use std::path::PathBuf;
use style::theme::Theme;

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
            .map(|path| path.join("onagre").join("theme.toml"))
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
        THEME
            .icon_theme
            .as_ref()
            .map(|theme_name| IconFinder::build(theme_name).ok())
            .flatten()
    };

}
