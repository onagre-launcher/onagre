use crate::db;
use crate::db::desktop_entry::DesktopEntryEntity;
use crate::db::plugin::PluginCommandEntity;
use crate::db::web::WebEntity;
use crate::db::Database;
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Debug)]
pub struct Cache {
    pub db: Database,
    history_lenght: Mutex<HashMap<String, usize>>,
}

impl Default for Cache {
    fn default() -> Self {
        Self {
            db: Database::default(),
            history_lenght: Mutex::new(Default::default()),
        }
    }
}

impl Cache {
    pub fn de_history(&self) -> Vec<DesktopEntryEntity<'static>> {
        let history = self
            .db
            .get_all::<DesktopEntryEntity>(db::desktop_entry::COLLECTION);
        let mut len = self.history_lenght.lock().unwrap();
        len.insert(db::desktop_entry::COLLECTION.to_string(), history.len());
        history
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
