#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate anyhow;

use clap::{App, Arg};

mod config;
pub mod entries;
mod freedesktop;
mod onagre;
mod style;
mod subscriptions;

use crate::config::OnagreSettings;
use crate::style::theme::Theme;

lazy_static! {
    pub static ref THEME: Theme = Theme::load();
    pub static ref SETTINGS: OnagreSettings = OnagreSettings::get().unwrap_or_default();
}

pub fn main() -> iced::Result {
    let default_config_path = dirs::data_dir().unwrap().join("onagre");

    let default_settings = default_config_path.join("config.toml");
    let default_settings = default_settings.to_str().unwrap();
    let default_theme = default_config_path.join("theme.toml");
    let default_theme = default_theme.to_str().unwrap();

    let mut modes = SETTINGS
        .modes
        .keys()
        .map(|mode| mode.as_str())
        .collect::<Vec<&str>>();

    modes.push("drun");
    modes.push("run");

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
                .help("path to an alternate onagre config file")
                .default_value(default_settings),
        )
        .arg(
            Arg::with_name("theme")
                .short("t")
                .long("theme")
                .takes_value(true)
                .help("path to an alternate onagre theme file")
                .default_value(default_theme),
        )
        .arg(
            Arg::with_name("dmenu")
                .short("d")
                .long("dmenu")
                .help("run onagre in dmenu mode (read from stdin and write to stdout)"),
        )
        .arg(
            Arg::with_name("modes")
                .short("m")
                .long("modes")
                .takes_value(true)
                .default_value("drun")
                .multiple(true)
                .possible_values(modes.as_slice())
                .help("load one or more onagre custom modes"),
        )
        .get_matches();
    onagre::run()
}
