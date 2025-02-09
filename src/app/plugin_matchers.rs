use onagre_launcher_toolkit::launcher::IconSource;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct Plugin {
    pub name: String,
    pub icon: Option<IconSource>,
    pub history: bool,
    pub isolate: bool,
    pub help: Option<String>,
    pub regex: Option<Regex>,
}

#[derive(Debug, Clone)]
pub struct QueryData<'a> {
    pub plugin: &'a Plugin,
    pub modifier: String,
    pub query: String,
}

impl Plugin {
    pub fn matching(&self, text: &str) -> Option<QueryData> {
        self.regex
            .as_ref()
            .and_then(|regex| regex.captures(text))
            .and_then(|captures| captures.get(1))
            .map(|m| m.as_str())
            .map(|modifier| QueryData {
                plugin: self,
                modifier: modifier.to_string(),
                query: text.strip_prefix(modifier).unwrap_or(text).to_string(),
            })
    }
}

#[cfg(test)]
mod test {
    use crate::app::plugin_matchers::Plugin;
    use regex::Regex;
    use speculoos::prelude::*;

    #[test]
    fn should_split_entry() {
        let plugin = Plugin {
            name: "find".to_string(),
            icon: None,
            history: false,
            isolate: false,
            help: Some("find ".to_string()),
            regex: Some(Regex::new("^(find )+").unwrap()),
        };

        let match_ = plugin.matching("find some text");

        assert_that!(match_)
            .is_some()
            .matches(|m| m.plugin.icon.is_none())
            .matches(|m| m.plugin.name == "find")
            .matches(|m| m.modifier == "find")
            .matches(|m| m.query == " some text")
            .matches(|m| !m.plugin.history);
    }

    #[test]
    fn should_not_match_plugin() {
        let plugin = Plugin {
            name: "find".to_string(),
            icon: None,
            history: false,
            isolate: false,
            help: Some("find ".to_string()),
            regex: Some(Regex::new("^(find )+").unwrap()),
        };

        let match_ = plugin.matching("fin");

        assert_that!(match_).is_none();
    }

    #[test]
    fn should_split_entry_with_plugin_name_followed_by_white_space() {
        let plugin = Plugin {
            name: "find".to_string(),
            icon: None,
            history: false,
            isolate: false,
            help: Some("find ".to_string()),
            regex: Some(Regex::new("^(find )+").unwrap()),
        };

        let match_ = plugin.matching("find ");

        assert_that!(match_)
            .is_some()
            .matches(|m| m.plugin.icon.is_none())
            .matches(|m| m.plugin.name == "find")
            .matches(|m| m.query == " ")
            .matches(|m| m.modifier == "find")
            .matches(|m| !m.plugin.history);
    }
}
