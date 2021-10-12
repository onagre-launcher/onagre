use serde::Deserialize;

use crate::SETTINGS;

lazy_static! {
    pub static ref WEB_CONFIG: Option<WebConfig> = {
        pop_launcher::config::find("web")
            .next()
            .map(|path| std::fs::read_to_string(path).ok())
            .flatten()
            .map(|config| ron::from_str::<WebConfig>(&config).ok())
            .flatten()
    };
    static ref EXTERNAL_MODES: Vec<String> = SETTINGS.modes.keys().cloned().collect();
}

#[derive(Debug, PartialEq)]
pub enum ActiveMode {
    Calc,
    DesktopEntry,
    Find,
    Files,
    Recent,
    Scripts,
    Terminal,
    Web(String),
    External(String),
    History,
}

#[derive(Debug, Deserialize)]
pub struct WebConfig {
    pub rules: Vec<Rule>,
}

#[derive(Debug, Deserialize)]
pub struct Rule {
    pub matches: Vec<String>,
    pub queries: Vec<WebQuery>,
}

#[derive(Debug, Deserialize)]
pub struct WebQuery {
    pub name: String,
    pub query: String,
}

impl From<&str> for ActiveMode {
    fn from(value: &str) -> Self {
        // Split at first space or get the full str
        let mode = value
            .split_once(" ")
            .map(|mode| mode.0)
            .unwrap_or_else(|| value);

        match mode {
            "" => ActiveMode::History,
            "calc" => ActiveMode::Calc,
            "find" => ActiveMode::Find,
            "files" => ActiveMode::Files,
            "recent" => ActiveMode::Recent,
            "scripts" => ActiveMode::Scripts,
            "run" => ActiveMode::Terminal,
            other => match WEB_CONFIG.as_ref() {
                Some(config) => {
                    let other = other.to_string();
                    let is_web_config = config
                        .rules
                        .iter()
                        .map(|rule| rule.matches.as_slice())
                        .any(|matches| matches.contains(&other));
                    if is_web_config {
                        ActiveMode::Web(other)
                    } else if EXTERNAL_MODES.contains(&other) {
                        ActiveMode::External(other)
                    } else {
                        ActiveMode::DesktopEntry
                    }
                }
                None => ActiveMode::DesktopEntry,
            },
        }
    }
}
