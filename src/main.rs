use std::path::PathBuf;
use std::sync::Mutex;

use anyhow::anyhow;
use clap::Parser;
use once_cell::sync::{Lazy, OnceCell};
use tracing::{debug, info};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use app::style::Theme;

pub mod app;
pub mod config;
pub mod db;
pub mod freedesktop;
pub mod icons;

pub static THEME_PATH: Lazy<Mutex<PathBuf>> = Lazy::new(|| {
    Mutex::new(
        dirs::config_dir()
            .ok_or_else(|| anyhow!("Theme config not found"))
            .map(|path| path.join("onagre").join("theme.scss"))
            .unwrap(),
    )
});

static THEME_SCALE: OnceCell<f32> = OnceCell::new();

pub static THEME: Lazy<Theme> = Lazy::new(Theme::load);

#[derive(Parser)]
#[command(name = "onagre", author = "Paul D. <paul.delafosse@protonmail.com>")]
struct Cli {
    #[arg(
        long = "theme",
        short = 't',
        help = "Path to an alternate onagre theme file"
    )]
    theme: Option<PathBuf>,

    #[arg(long = "scale", short = 's', help = "Change the scale of onagre theme")]
    scale: Option<f32>,

    #[arg(long = "mode", short = 'm', help = "The mode parameter as a string")]
    mode: Option<String>,
}

pub fn main() -> iced::Result {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "onagre=info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Starting onagre");
    let cli = Cli::parse();

    // User defined theme config, $XDG_CONFIG_DIR/onagre/theme.toml otherwise
    if let Some(theme_path) = cli.theme {
        let path = theme_path.canonicalize();
        if let Ok(path) = path {
            *THEME_PATH.lock().unwrap() = path;
        }

        info!("Using alternate theme : {:?}", THEME_PATH.lock().unwrap());
    }

    if let Some(scale) = cli.scale {
        THEME_SCALE.get_or_init(|| scale);
        info!("Using scale value : {:?}", scale);
    }

    if let Some(mode) = cli.mode {
        debug!("Mode parameter: {:?}", mode);

        app::run(Some(mode))
    } else {
        app::run(None)
    }
}
