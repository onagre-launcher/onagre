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

use freedesktop::IconFinder;
use once_cell::sync::Lazy;
use std::path::PathBuf;
use std::sync::Mutex;
use style::theme::Theme;

pub static THEME_PATH: Lazy<Mutex<PathBuf>> = Lazy::new(|| {
    Mutex::new(
        dirs::config_dir()
            .ok_or_else(|| anyhow!("Theme config not found"))
            .map(|path| path.join("onagre").join("theme.toml"))
            .unwrap(),
    )
});

pub static THEME: Lazy<Theme> = Lazy::new(|| Theme::load());

pub static ICON_FINDER: Lazy<Option<IconFinder>> = Lazy::new(|| {
    THEME
        .icon_theme
        .as_ref()
        .map(|theme_name| IconFinder::build(theme_name).ok())
        .flatten()
});
