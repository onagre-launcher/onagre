use crate::ui::mode::WEB_CONFIG;
use pop_launcher_toolkit::service::config::PluginConfig;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct Plugin {
    pub name: String,
    pub history: bool,
    pub help: Option<String>,
    pub regex: Option<Regex>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PluginMode {
    pub plugin_name: String,
    pub modifier: String,
    pub history: bool,
}

impl PluginMode {
    fn new_mode_web<S: AsRef<str>>(modifier: S) -> PluginMode {
        PluginMode {
            plugin_name: "web".to_string(),
            modifier: modifier.as_ref().to_string(),
            history: true,
        }
    }
}

impl Plugin {
    pub fn from_subscription(name: String, config: PluginConfig, regex: Option<Regex>) -> Self {
        Self {
            help: config.query.help.map(|h| h.to_string()),
            history: config.history,
            name,
            regex,
        }
    }

    fn to_mode(&self) -> PluginMode {
        PluginMode {
            plugin_name: self.name.clone(),
            modifier: self.name.clone(),
            history: self.history,
        }
    }

    fn new_mode<S: AsRef<str>>(&self, modifier: S) -> PluginMode {
        PluginMode {
            plugin_name: self.name.clone(),
            modifier: modifier.as_ref().to_string(),
            history: self.history,
        }
    }
}

pub fn match_web_plugins(text: &str) -> Option<(PluginMode, String)> {
    match WEB_CONFIG.as_ref() {
        None => None,

        // Fixme: we should attach the config to the plugin itself and check them on the fly
        Some(config) => text.split_once(' ').and_then(|(mode, query)| {
            let is_match = config
                .rules
                .iter()
                .flat_map(|rule| rule.matches.as_slice())
                .any(|matches| matches.contains(&mode.to_string()));

            if is_match {
                let mode = PluginMode::new_mode_web(mode);
                Some((mode, query.to_string()))
            } else {
                None
            }
        }),
    }
}

impl Plugin {
    pub fn try_match(&self, text: &str) -> Option<(PluginMode, String)> {
        self.match_plugin_help(text)
            .or(self.match_plugin_regex(text))
    }

    fn match_plugin_regex(&self, text: &str) -> Option<(PluginMode, String)> {
        // A dirty fix to prevent looping between modifier and search display
        // for the file mode, could this happen to other plugins ?
        if text == "~" {
            return None;
        };

        let is_match = self
            .regex
            .as_ref()
            .map(|regex| regex.is_match(text))
            .unwrap_or(false);

        if is_match {
            self.help.as_ref().and_then(|mode| {
                let query = text
                    .strip_prefix(mode)
                    .or(text.strip_prefix(&self.name))
                    .unwrap_or("");
                let mode = self.new_mode(mode);

                Some((mode, query.to_string())).filter(|(mode, _query)| !mode.modifier.is_empty())
            })
        } else {
            None
        }
    }

    fn match_plugin_help(&self, text: &str) -> Option<(PluginMode, String)> {
        text.split_once(&self.name).map(|(_, query)| {
            let mode = self.to_mode();
            (mode, query.to_string())
        })
    }
}

#[cfg(test)]
mod test {
    use crate::ui::plugin_matchers::{Plugin, PluginMode};
    use regex::Regex;

    #[test]
    fn should_split_entry() {
        let plugin = Plugin {
            name: "find".to_string(),
            history: false,
            help: Some("find ".to_string()),
            regex: Some(Regex::new("^(find )+").unwrap()),
        };

        let match_ = plugin.try_match("find some text");

        assert_eq!(
            match_,
            Some((
                PluginMode {
                    plugin_name: "find".to_string(),
                    modifier: "find ".to_string(),
                    history: false
                },
                "some text".to_string()
            ))
        );
    }

    #[test]
    fn should_not_split_entry_with_only_plugin_name() {
        let plugin = Plugin {
            name: "find".to_string(),
            history: false,
            help: Some("find ".to_string()),
            regex: Some(Regex::new("^(find )+").unwrap()),
        };

        let match_ = plugin.try_match("find");

        assert_eq!(match_, None);
    }

    #[test]
    fn should_split_entry_with_plugin_name_followed_by_white_space() {
        let plugin = Plugin {
            name: "find".to_string(),
            history: false,
            help: Some("find ".to_string()),
            regex: Some(Regex::new("^(find )+").unwrap()),
        };

        let match_ = plugin.try_match("find ");

        assert_eq!(
            match_,
            Some((
                PluginMode {
                    plugin_name: "find".to_string(),
                    modifier: "find ".to_string(),
                    history: false
                },
                "".to_string()
            ))
        );
    }
}
