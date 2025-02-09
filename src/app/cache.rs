use crate::db::desktop_entry::DesktopEntryEntity;
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
    pub fn plugin_history(&self, plug_name: &str) -> Vec<DesktopEntryEntity<'static>> {
        let history = self.db.get_all(plug_name);
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
}
