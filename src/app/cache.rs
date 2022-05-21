use crate::db;
use crate::db::desktop_entry::DesktopEntryEntity;
use crate::db::plugin::PluginCommandEntity;
use crate::db::web::WebEntity;
use crate::db::Database;
use once_cell::sync::OnceCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Mutex;

type History<T> = Mutex<HashMap<String, Rc<Vec<T>>>>;

#[derive(Debug)]
pub struct Cache<'a> {
    pub db: Database,
    de_history: OnceCell<Vec<DesktopEntryEntity<'a>>>,
    web_history: History<WebEntity<'a>>,
    plugin_history: History<PluginCommandEntity<'a>>,
}

impl Default for Cache<'_> {
    fn default() -> Self {
        Self {
            db: Database::default(),
            de_history: OnceCell::new(),
            web_history: Mutex::new(Default::default()),
            plugin_history: Mutex::new(Default::default()),
        }
    }
}

impl Cache<'_> {
    pub fn de_history(&self) -> &Vec<DesktopEntryEntity> {
        self.de_history.get_or_init(|| {
            self.db
                .get_all::<DesktopEntryEntity>(db::desktop_entry::COLLECTION)
        })
    }

    pub fn de_len(&self) -> usize {
        self.de_history.get().map(|de| de.len()).unwrap_or(0)
    }

    pub fn plugin_history(&self, plug_name: &str) -> Rc<Vec<PluginCommandEntity>> {
        let mut history = self.plugin_history.lock().unwrap();
        if history.get(plug_name).is_none() {
            let data = self.db.get_all::<PluginCommandEntity>(plug_name);
            history.insert(plug_name.to_string(), Rc::new(data));
        }

        Rc::clone(history.get(plug_name).unwrap())
    }

    pub fn plugin_history_len(&self, plug_name: &str) -> usize {
        self.plugin_history
            .lock()
            .unwrap()
            .get(plug_name)
            .map(|p| p.len())
            .unwrap_or(0)
    }

    pub fn web_history(&self, web_name: &str) -> Rc<Vec<WebEntity>> {
        let mut history = self.web_history.lock().unwrap();
        if history.get(web_name).is_none() {
            let data = self.db.get_all::<WebEntity>(web_name);
            history.insert(web_name.to_string(), Rc::new(data));
        }

        Rc::clone(history.get(web_name).unwrap())
    }

    pub fn web_history_len(&self, web_name: &str) -> usize {
        self.web_history
            .lock()
            .unwrap()
            .get(web_name)
            .map(|p| p.len())
            .unwrap_or(0)
    }
}
