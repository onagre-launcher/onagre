use std::path::PathBuf;

pub struct Config {
    menus: Vec<Menu>,
}

pub struct Menu {
    title: String,
    input_command: InputSource,
    command_template: String,
    stylesheet: Option<PathBuf>,
}

pub enum InputSource {
    History,
    Command(String),
}
