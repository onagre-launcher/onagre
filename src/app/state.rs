use crate::app::cache::Cache;
use crate::app::mode::ActiveMode;
use crate::app::plugin_matchers::{match_web_plugins, Plugin};
use crate::app::{Message, INPUT_ID};

use iced::futures::channel::mpsc::Sender;
use iced::widget::text_input;
use iced::Task;
use onagre_launcher_toolkit::launcher::{IconSource, Request, SearchResult};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::debug;

use super::entries::Entry;
use super::OnagreTheme;

#[derive(Debug)]
pub struct Onagre {
    pub input_value: SearchInput,
    pub selected: Selection,
    pub cache: Cache,
    pub pop_search: Vec<SearchResult>,
    pub exec_on_next_search: bool,
    pub plugin_matchers: PluginConfigCache,
    pub request_tx: Option<Sender<Request>>,
    pub entries: Vec<Box<dyn Entry>>,
    pub plugin_icon: Option<IconSource>,
    pub theme: OnagreTheme,
}

impl Onagre {
    pub fn new(theme: OnagreTheme) -> Self {
        Self {
            input_value: SearchInput::default(),
            selected: Selection::Reset,
            cache: Cache::default(),
            pop_search: Vec::new(),
            exec_on_next_search: false,
            plugin_matchers: PluginConfigCache::load(),
            request_tx: None,
            entries: vec![],
            plugin_icon: None,
            theme,
        }
    }

    pub fn get_theme(&self) -> &crate::Theme {
        self.theme.0.as_ref()
    }

    pub fn clone_theme(&self) -> Arc<crate::Theme> {
        self.theme.0.clone()
    }
}

#[derive(Debug)]
pub struct PluginConfigCache {
    pub(crate) inner: HashMap<String, Plugin>,
}

impl PluginConfigCache {
    pub fn load() -> Self {
        let mut cache = HashMap::new();
        for (path, config, regex) in onagre_launcher_toolkit::service::load::from_paths() {
            let name = path
                .parent()
                .expect("Plugin config should have a parent directory")
                .file_name()
                .expect("Plugin directory should have an utf8 filename")
                .to_string_lossy()
                .to_string();

            let plugin = Plugin {
                name: name.clone(),
                icon: config.icon,
                history: config.history,
                help: config.query.help.map(|h| h.to_string()),
                regex,
            };

            cache.insert(name, plugin);
        }

        PluginConfigCache { inner: cache }
    }
    pub fn get_plugin_icon(&self, plugin_name: &str) -> Option<IconSource> {
        self.inner.get(plugin_name).and_then(|de| de.icon.clone())
    }

    pub fn insert(&mut self, key: String, plugin: Plugin) {
        self.inner.insert(key, plugin);
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Selection {
    // The selection is the content of the search bar, not something we got from pop-launcher
    // moving down will change selection to `History(0)`
    Reset,
    // This means we are trying to activate and item from the history
    // We need to issue a `Request::Search` before activating it.
    History(usize),
    // The selected item is one of the pop-launcher response items
    // It's safe to call `Request::Activate`.
    PopLauncher(usize),
}

impl Onagre {
    pub fn get_active_mode(&self) -> &ActiveMode {
        &self.input_value.mode
    }

    pub fn get_input(&self) -> String {
        if self.get_theme().plugin_hint().is_none() {
            self.input_value.input_display.clone()
        } else {
            self.input_value.pop_query.clone()
        }
    }

    pub fn with_mode(mode_query: &str, theme: OnagreTheme) -> Self {
        let plugin_matchers = PluginConfigCache::load();
        let plugin_split = match_web_plugins(mode_query).or_else(|| {
            plugin_matchers
                .inner
                .values()
                .map(|matcher| matcher.try_match(mode_query))
                .find_map(|match_| match_)
        });

        let mode = plugin_split
            .as_ref()
            .map(|split| ActiveMode::from(split.clone()))
            .unwrap_or_default();
        let modifier_display = plugin_split
            .as_ref()
            .map(|query_data| query_data.modifier.clone())
            .unwrap_or_default();
        let input_display = plugin_split
            .map(|query_data| query_data.query)
            .unwrap_or_default();

        Onagre {
            selected: Selection::History(0),
            cache: Default::default(),
            pop_search: Default::default(),
            input_value: SearchInput {
                mode,
                modifier_display,                  // dgg
                input_display,                     // <search str>
                pop_query: mode_query.to_string(), // dgg <search str>
            },
            exec_on_next_search: false,
            plugin_matchers,
            request_tx: Default::default(),
            entries: vec![],
            plugin_icon: None,
            theme,
        }
    }

    pub fn set_input(&mut self, input: &str) {
        let previous_modi = self.input_value.modifier_display.clone();

        if !previous_modi.is_empty() {
            self.set_input_with_modifier(input, previous_modi);
        } else {
            self.set_input_without_modifier(input);
        };

        let pop_query = match &self.input_value.mode {
            ActiveMode::History | ActiveMode::DesktopEntry => {
                self.input_value.input_display.clone()
            }
            ActiveMode::Web { modifier, .. } => {
                format!("{modifier} {}", self.input_value.input_display)
            }
            ActiveMode::Plugin { modifier, .. } => {
                format!("{modifier}{}", self.input_value.input_display)
            }
        };

        self.input_value.pop_query = pop_query;
        debug!(
            "State: mode={:?}, input={}",
            self.input_value.mode, self.input_value.input_display
        );
    }

    fn set_input_without_modifier(&mut self, input: &str) {
        let plugin_split = match_web_plugins(input).or_else(|| {
            self.plugin_matchers
                .inner
                .values()
                .map(|matcher| matcher.try_match(input))
                .find_map(|match_| match_)
        });

        if let Some(query_data) = plugin_split {
            self.input_value.modifier_display = query_data.modifier.clone();
            self.input_value.mode = ActiveMode::from(query_data.clone());
            // If plugin-hint is disabled use the full input,
            // otherwise use the split value
            self.input_value.input_display = if self.get_theme().plugin_hint().is_none() {
                input.to_string()
            } else {
                query_data.query
            };
        } else {
            self.input_value.input_display = input.to_string();

            if input.is_empty() {
                self.input_value.mode = ActiveMode::History
            } else {
                self.input_value.mode = ActiveMode::DesktopEntry
            }
        }
    }

    fn set_input_with_modifier(&mut self, input: &str, previous_modi: String) {
        if input.is_empty() {
            self.input_value.modifier_display = "".to_string();
            self.input_value.input_display = if self.get_theme().plugin_hint().is_none() {
                input.to_string()
            } else {
                previous_modi
            };
            self.input_value.mode = ActiveMode::DesktopEntry;
            let _: Task<Message> = text_input::move_cursor_to_end(INPUT_ID.clone());
        } else {
            self.input_value.input_display = input.to_string();
        }
    }
}

#[derive(Debug, Default)]
pub struct SearchInput {
    pub mode: ActiveMode,
    pub modifier_display: String,
    pub input_display: String,
    pub pop_query: String,
}
