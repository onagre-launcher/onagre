use crate::app::plugin_matchers::QueryData;
use onagre_launcher_toolkit::plugins::web::Config as WebConfig;
use once_cell::sync::Lazy;

pub(crate) static WEB_CONFIG: Lazy<WebConfig> =
    Lazy::new(onagre_launcher_toolkit::plugins::web::load);

#[derive(Debug, PartialEq, Clone, Default)]
pub enum ActiveMode {
    #[default]
    History,
    DesktopEntry,
    Web {
        modifier: String,
    },
    Plugin {
        plugin_name: String,
        modifier: String,
        history: bool,
    },
}

impl From<QueryData> for ActiveMode {
    fn from(query_data: QueryData) -> Self {
        let mode = query_data.plugin_name.as_str();
        match mode {
            "web" => ActiveMode::Web {
                modifier: query_data.modifier,
            },
            _other => ActiveMode::Plugin {
                plugin_name: query_data.plugin_name,
                modifier: query_data.modifier,
                history: query_data.history,
            },
        }
    }
}
