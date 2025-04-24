use std::{path::PathBuf, sync::Arc};

use anyhow::anyhow;
use app::OnagreTheme;
use clap::Parser;
use tracing::{debug, info};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use app::style::{Scale, Theme};

pub mod app;
pub mod config;
pub mod db;
pub mod freedesktop;

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

    let theme_path = cli.theme.and_then(|path| {
        info!("Using alternate theme : {path:?}");
        path.canonicalize().ok()
    });

    let theme_path = theme_path.unwrap_or_else(|| {
        dirs::config_dir()
            .ok_or_else(|| anyhow!("Theme config not found"))
            .map(|path| path.join("onagre").join("theme.scss"))
            .unwrap()
    });

    let theme = Theme::load(theme_path);

    let theme = if let Some(scale) = cli.scale {
        info!("Using scale value : {:?}", scale);
        theme.scale(scale)
    } else {
        theme
    };

    let theme = Arc::new(theme);

    if let Some(mode) = cli.mode {
        debug!("Mode parameter: {:?}", mode);
        app::run(Some(mode), OnagreTheme(theme))
    } else {
        app::run(None, OnagreTheme(theme))
    }
}
