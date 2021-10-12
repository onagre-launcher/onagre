use std::cmp::Reverse;
use std::fmt::Debug;

use serde::de::DeserializeOwned;
use serde::Serialize;

pub mod desktop_entry;
pub mod run;
pub mod web;

#[derive(Clone, Debug)]
pub struct Database {
    inner: sled::Db,
}

impl Default for Database {
    fn default() -> Self {
        let path = dirs::data_dir().expect("Cannot open data dir");

        let path = path.join("onagre");

        debug!("Opening database {:?}", path);

        Database {
            inner: sled::open(path).unwrap(),
        }
    }
}

impl Database {
    pub fn insert<T>(&self, entity: &T) -> sled::Result<()>
    where
        T: Sized + Entity + Serialize,
    {
        let json = serde_json::to_string(entity).expect("Serialization error");

        let result = self
            .inner
            .open_tree(T::COLLECTION)?
            .insert(entity.get_key(), json.as_bytes())
            .map(|_res| ());

        self.inner.flush().expect("Failed to flush database");
        result
    }

    pub fn get_by_key<T>(&self, key: &str) -> Option<T>
    where
        T: Entity + DeserializeOwned,
    {
        self.inner
            .open_tree(T::COLLECTION)
            .unwrap()
            .get(key.as_bytes())
            .ok()
            .flatten()
            .map(|data| data.to_vec())
            .map(String::from_utf8)
            .map(Result::unwrap)
            .map(|raw_data| serde_json::from_str(&raw_data))
            .map(Result::unwrap)
    }

    pub fn get_all<T>(&self) -> Vec<T>
    where
        T: Entity + DeserializeOwned + Debug,
    {
        let mut results: Vec<T> = self
            .inner
            .open_tree(T::COLLECTION)
            .unwrap()
            .iter()
            .map(|res| res.expect("Database error"))
            .map(|(_k, v)| String::from_utf8(v.to_vec()).unwrap())
            .map(|entity_string| serde_json::from_str(entity_string.as_str()))
            .flat_map(Result::ok)
            .collect();

        results.sort_by_key(|b| Reverse(b.get_weight()));
        debug!("Fetching {} history from database", T::COLLECTION);
        trace!("{:?}", results);
        results
    }
}

pub trait Entity {
    fn get_key(&self) -> Vec<u8>;
    fn get_weight(&self) -> u8;
    const COLLECTION: &'static str;
}
