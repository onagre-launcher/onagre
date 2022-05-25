use crate::app::mode::WEB_CONFIG;
use crate::icons::IconPath;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct Plugin {
    pub name: String,
    pub icon: Option<IconPath>,
    pub history: bool,
    pub help: Option<String>,
    pub regex: Option<Regex>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct QueryData {
    pub icon: Option<IconPath>,
    pub plugin_name: String,
    pub modifier: String,
    pub query: String,
    pub history: bool,
}

impl QueryData {
    fn new_mode_web<S: AsRef<str>>(modifier: S, query: String) -> QueryData {
        QueryData {
            icon: None,
            plugin_name: "web".to_string(),
            modifier: modifier.as_ref().to_string(),
            history: true,
            query,
        }
    }
}

impl Plugin {
    fn to_query_data(&self, query: String) -> QueryData {
        QueryData {
            icon: None,
            plugin_name: self.name.clone(),
            modifier: self.name.clone(),
            query,
            history: self.history,
        }
    }

    fn get_query_data<S: AsRef<str>>(&self, modifier: S, query: String) -> QueryData {
        QueryData {
            icon: None,
            plugin_name: self.name.clone(),
            modifier: modifier.as_ref().to_string(),
            query,
            history: self.history,
        }
    }
}

pub fn match_web_plugins(text: &str) -> Option<QueryData> {
    text.split_once(' ').and_then(|(mode, query)| {
        if WEB_CONFIG.get(mode).is_some() {
            Some(QueryData::new_mode_web(mode, query.to_string()))
        } else {
            None
        }
    })
}

impl Plugin {
    pub fn try_match(&self, text: &str) -> Option<QueryData> {
        self.match_plugin_help(text)
            .or_else(|| self.match_plugin_regex(text))
    }

    fn match_plugin_regex(&self, text: &str) -> Option<QueryData> {
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
                    .or_else(|| text.strip_prefix(&self.name))
                    .unwrap_or("");
                if !mode.is_empty() {
                    Some(self.get_query_data(mode, query.to_string()))
                } else {
                    None
                }
            })
        } else {
            None
        }
    }

    fn match_plugin_help(&self, text: &str) -> Option<QueryData> {
        text.split_once(&self.name)
            .map(|(_, query)| self.to_query_data(query.to_string()))
    }
}

#[cfg(test)]
mod test {
    use crate::app::plugin_matchers::{Plugin, QueryData};
    use regex::Regex;

    #[test]
    fn should_split_entry() {
        let plugin = Plugin {
            name: "find".to_string(),
            icon: None,
            history: false,
            help: Some("find ".to_string()),
            regex: Some(Regex::new("^(find )+").unwrap()),
        };

        let match_ = plugin.try_match("find some text");

        assert_eq!(
            match_,
            Some(QueryData {
                icon: None,
                plugin_name: "find".to_string(),
                modifier: "find".to_string(),
                query: " some text".to_string(),
                history: false,
            })
        );
    }

    #[test]
    fn should_not_match_plugin() {
        let plugin = Plugin {
            name: "find".to_string(),
            icon: None,
            history: false,
            help: Some("find ".to_string()),
            regex: Some(Regex::new("^(find )+").unwrap()),
        };

        let match_ = plugin.try_match("fin");

        assert_eq!(match_, None);
    }

    #[test]
    fn should_split_entry_with_plugin_name_followed_by_white_space() {
        let plugin = Plugin {
            name: "find".to_string(),
            icon: None,
            history: false,
            help: Some("find ".to_string()),
            regex: Some(Regex::new("^(find )+").unwrap()),
        };

        let match_ = plugin.try_match("find ");

        assert_eq!(
            match_,
            Some(QueryData {
                icon: None,
                plugin_name: "find".to_string(),
                modifier: "find".to_string(),
                query: " ".to_string(),
                history: false,
            })
        );
    }
}
