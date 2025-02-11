use crate::db;
use crate::db::desktop_entry::DesktopEntryEntity;
use crate::db::plugin::PluginCommandEntity;
use crate::db::web::WebEntity;
use crate::db::Database;
use once_cell::sync::OnceCell;
use std::collections::HashMap;
use std::sync::Mutex;

type History<T> = Mutex<HashMap<String, Vec<T>>>;

#[derive(Debug)]
pub struct Cache {
    pub db: Database,
    history_lenght: Mutex<HashMap<String, usize>>,
    de_history: OnceCell<Vec<DesktopEntryEntity<'static>>>,
    web_history: History<WebEntity<'static>>,
    plugin_history: History<PluginCommandEntity<'static>>,
}

impl Default for Cache {
    fn default() -> Self {
        Self {
            db: Database::default(),
            de_history: OnceCell::new(),
            web_history: Mutex::new(Default::default()),
            plugin_history: Mutex::new(Default::default()),
            history_lenght: Mutex::new(Default::default()),
        }
    }
}

impl Cache {
    pub fn de_history(&self) -> &Vec<DesktopEntryEntity<'static>> {
        self.de_history.get_or_init(|| {
            self.db
                .get_all::<DesktopEntryEntity>(db::desktop_entry::COLLECTION)
        })
    }

    pub fn de_len(&self) -> usize {
        self.de_history.get().map(|de| de.len()).unwrap_or(0)
    }

    pub fn plugin_history(&self, plug_name: &str) -> Vec<PluginCommandEntity<'static>> {
        let history = self.db.get_all::<PluginCommandEntity>(plug_name);
        let mut len = self.history_lenght.lock().unwrap();
        len.insert(plug_name.to_string(), history.len());
        history
    }

    pub fn history_len(&self, plug_name: &str) -> usize {
        *self
            .history_lenght
            .lock()
            .unwrap()
            .get(plug_name)
            .unwrap_or(&0)
    }

    pub fn web_history(&self, web_name: &str) -> Vec<WebEntity<'static>> {
        let history = self.db.get_all::<WebEntity>(web_name);
        let mut len = self.history_lenght.lock().unwrap();
        len.insert(web_name.to_string(), history.len());
        history
    }
}
