#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate anyhow;

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
    onagre::run()
}
