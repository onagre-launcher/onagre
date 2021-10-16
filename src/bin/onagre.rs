use std::path::PathBuf;

use log::debug;
use onagre::app;
use onagre::{SETTINGS_PATH, THEME_PATH};
use structopt::StructOpt;

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
