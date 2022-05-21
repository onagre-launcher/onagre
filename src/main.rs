use std::path::PathBuf;

use anyhow::anyhow;
use log::debug;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use structopt::StructOpt;
use ui::style::Theme;

pub mod config;
pub mod db;
pub mod entries;
pub mod font;
pub mod freedesktop;
pub mod icons;
pub mod ui;

pub static THEME_PATH: Lazy<Mutex<PathBuf>> = Lazy::new(|| {
    Mutex::new(
        dirs::config_dir()
            .ok_or_else(|| anyhow!("Theme config not found"))
            .map(|path| path.join("onagre").join("theme.toml"))
            .unwrap(),
    )
});

pub static THEME: Lazy<Theme> = Lazy::new(Theme::load);

#[derive(StructOpt)]
#[structopt(name = "onagre", author = "Paul D. <paul.delafosse@protonmail.com>")]
struct Cli {
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

    ui::run()
}
