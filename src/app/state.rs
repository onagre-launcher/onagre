use crate::app::cache::Cache;
use crate::app::mode::ActiveMode;
use crate::app::plugin_matchers::Plugin;

use super::entries::Entry;
use super::OnagreTheme;
use crate::db::desktop_entry::DEFAULT_PLUGIN;
use iced::futures::channel::mpsc::Sender;
use onagre_launcher_toolkit::launcher::{IconSource, Request};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug)]
pub struct Onagre {
    pub active_mode: ActiveMode,
    pub selected: usize,
    pub cache: Cache,
    pub exec_on_next_search: bool,
    pub plugin_matchers: PluginConfigCache,
    pub request_tx: Option<Sender<Request>>,
    pub entries: Vec<Box<dyn Entry>>,
    pub plugin_icon: Option<IconSource>,
    pub theme: OnagreTheme,
}

impl Onagre {
    pub fn new(theme: OnagreTheme) -> Self {
        let cache = Cache::default();
        let entries = cache
            .plugin_history(DEFAULT_PLUGIN)
            .iter()
            .map(|entry| Box::new(entry.clone()) as Box<dyn Entry>)
            .collect::<Vec<Box<dyn Entry>>>();
        
        Self {
            active_mode: ActiveMode::Default("".to_string()),
            selected: 0,
            cache,
            exec_on_next_search: false,
            plugin_matchers: PluginConfigCache::load(),
            request_tx: None,
            entries,
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
            None => ActiveMode::Default(query.to_string()),
        }
    }
}

impl Onagre {
    pub fn start_with_mode(query: &str, theme: OnagreTheme) -> Self {
        let mut onagre = Onagre::new(theme);
        onagre.active_mode = ActiveMode::Default(query.to_string());
        onagre
    }
}
