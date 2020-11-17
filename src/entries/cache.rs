use crate::entries::Entry;
use crate::Mode;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

pub async fn get_cached_entries(modes: Vec<Mode>) -> HashMap<Mode, Vec<Arc<RwLock<Entry>>>> {
    let mut entry_map = HashMap::new();

    for mode in modes {
        let entries = get_cache(&mode).await;
        debug!(
            "loaded {} cached entries for mode {:?}",
            entries.len(),
            mode
        );
        entry_map.insert(mode, entries);
    }

    entry_map
}

async fn get_cache(mode: &Mode) -> Vec<Arc<RwLock<Entry>>> {
    let cache_file = get_cache_mode_path(mode);
    if let Ok(file_content) = fs::read_to_string(cache_file) {
        let entries: Result<Vec<Entry>, serde_json::Error> = serde_json::from_str(&file_content);
        if let Ok(entries) = entries {
            entries.into_iter().map(RwLock::new).map(Arc::new).collect()
        } else {
            vec![]
        }
    } else {
        vec![]
    }
}

pub fn flush_mode_cache(mode: &Mode, entries: &[Arc<RwLock<Entry>>]) {
    let mut owned_entries = vec![];
    for entry in entries.iter().take(50) {
        owned_entries.push(entry.read().unwrap().clone());
    }

    if let Ok(content) = serde_json::to_string(&owned_entries) {
        debug!("Flushing {} entries for mode {:?}", entries.len(), mode);
        let _ = std::fs::write(get_cache_mode_path(mode), content);
    }
}

fn get_cache_mode_path(mode: &Mode) -> PathBuf {
    let cache_filename = format!("onagre-{}", mode.to_string().to_lowercase());
    let cache_file = dirs::cache_dir().unwrap();
    cache_file.join(cache_filename)
}
