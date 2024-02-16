use once_cell::sync::Lazy;
use std::borrow::Cow;
use std::cmp::Reverse;
use std::fmt::Debug;
use std::sync::Arc;
use tracing::{debug, trace};

use redb::{ReadableTable, TableDefinition};
use serde::de::DeserializeOwned;
use serde::Serialize;

pub mod desktop_entry;
pub mod plugin;
pub mod web;

pub static DB: Lazy<Database> = Lazy::new(Database::default);

#[derive(Clone, Debug)]
pub struct Database {
    inner: Arc<redb::Database>,
}

impl Default for Database {
    fn default() -> Self {
        let path = dirs::data_dir().expect("Cannot open data dir");

        let path = path.join("onagre-db");
        let path = path.as_path();

        debug!("Opening database {:?}", path);

        let database = match redb::Database::open(path) {
            Ok(db) => db,
            Err(_err) => redb::Database::create(path).expect("failed to create database"),
        };

        Database {
            inner: Arc::new(database),
        }
    }
}

impl Database {
    pub fn insert<'a, T>(&self, collection: &str, entity: &T) -> Result<(), redb::Error>
    where
        T: Sized + Entity<'a> + Serialize,
    {
        let json = serde_json::to_string(entity).expect("Serialization error");
        let db = self.inner.clone();
        let write_tnx = db.begin_write()?;
        {
            let definition = TableDefinition::<&str, &str>::new(collection);
            let mut table = write_tnx.open_table(definition)?;
            table.insert(entity.get_key().as_ref(), json.as_str())?;
        }
        write_tnx.commit()?;
        Ok(())
    }

    pub fn get_by_key<'a, T>(&self, collection: &str, key: &str) -> Option<T>
    where
        T: Entity<'a> + DeserializeOwned,
    {
        let definition = TableDefinition::<&str, &str>::new(collection);
        let db = self.inner.clone();
        let Ok(read_txn) = db.begin_write() else {
            return None;
        };

        let table = read_txn
            .open_table(definition)
            .expect("failed to open database");
        table
            .get(key)
            .ok()
            .flatten()
            .map(|data| serde_json::from_str(data.value()))
            .and_then(Result::ok)
    }

    pub fn get_all<'a, T>(&self, collection: &str) -> Vec<T>
    where
        T: Entity<'a> + DeserializeOwned + Debug,
    {
        let definition = TableDefinition::<&str, &str>::new(collection);
        let db = self.inner.clone();
        let Ok(read_txn) = db.begin_write() else {
            return vec![];
        };
        let table = read_txn.open_table(definition).unwrap();
        let mut results: Vec<T> = table
            .iter()
            .unwrap()
            .filter_map(Result::ok)
            .map(|(_key, value)| serde_json::from_str(value.value()))
            .flat_map(Result::ok)
            .collect();

        results.sort_by_key(|b| Reverse(b.get_weight()));
        debug!(
            "Got {} database entries from for '{collection}'",
            results.len()
        );
        trace!("{:?}", results);
        results
    }
}

pub trait Entity<'a> {
    fn get_key(&self) -> Cow<'a, str>;
    fn get_weight(&self) -> u8;
}
