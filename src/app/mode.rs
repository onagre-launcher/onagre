use crate::app::plugin_matchers::PluginMode;
use once_cell::sync::Lazy;
use serde::Deserialize;

pub(crate) static WEB_CONFIG: Lazy<Option<WebConfig>> = Lazy::new(|| {
    pop_launcher_toolkit::launcher::config::find("web")
        .next()
        .and_then(|path| std::fs::read_to_string(path).ok())
        .and_then(|config| ron::from_str::<WebConfig>(&config).ok())
});

#[derive(Debug, PartialEq, Clone)]
pub enum ActiveMode {
    History,
    DesktopEntry,
    Web(String),
    Plugin {
        plugin_name: String,
        modifier: String,
        history: bool,
    },
}

impl From<PluginMode> for ActiveMode {
    fn from(plugin_mode: PluginMode) -> Self {
        let mode = plugin_mode.plugin_name.as_str();
        match mode {
            "web" => ActiveMode::Web(plugin_mode.modifier),
            _other => ActiveMode::Plugin {
                plugin_name: plugin_mode.plugin_name,
                modifier: plugin_mode.modifier,
                history: plugin_mode.history,
            },
        }
    }
}

impl Default for ActiveMode {
    fn default() -> Self {
        ActiveMode::History
    }
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
