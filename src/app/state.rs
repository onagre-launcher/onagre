use crate::app::cache::Cache;
use crate::app::mode::ActiveMode;
use crate::app::plugin_matchers::Plugin;

use iced::futures::channel::mpsc::Sender;
use onagre_launcher_toolkit::launcher::{IconSource, Request, SearchResult};
use std::collections::HashMap;
use std::sync::Arc;

use super::entries::Entry;
use super::OnagreTheme;

#[derive(Debug)]
pub struct Onagre {
    pub active_mode: ActiveMode,
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
            active_mode: ActiveMode::History,
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
                isolate: config.query.isolate,
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

    pub fn get_active_mode(&self, query: &str, current: &ActiveMode) -> ActiveMode {
        let query = match current.modifier() {
            Some(modifier) if query.is_empty() => modifier[..modifier.len() - 1].to_string(),
            Some(modifier) => format!("{modifier}{query}"),
            None => query.to_string(),
        };
        
        let plugin_split = self
            .inner
            .values()
            .map(|plugin| plugin.matching(&query))
            .find_map(|match_| match_);

        let mode = plugin_split.as_ref().map(|split| ActiveMode::Plugin {
            plugin_name: split.plugin.name.clone(),
            modifier: split.modifier.clone(),
            query: split.query.clone(),
            history: split.plugin.history,
            isolate: split.plugin.isolate,
        });

        match mode {
            Some(mode) => mode,
            None if query.is_empty() => ActiveMode::History,
            None => ActiveMode::Default(query.to_string()),
        }
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
    pub fn start_with_mode(query: &str, theme: OnagreTheme) -> Self {
        let plugin_matchers = PluginConfigCache::load();

        Onagre {
            selected: Selection::History(0),
            cache: Default::default(),
            pop_search: Default::default(),
            active_mode: ActiveMode::History,
            exec_on_next_search: false,
            plugin_matchers,
            request_tx: Default::default(),
            entries: vec![],
            plugin_icon: None,
            theme,
        }
    }
}

#[derive(Debug, Default)]
pub struct SearchInput {
    pub mode: ActiveMode,
}
