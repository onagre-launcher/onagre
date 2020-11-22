use crate::entries::Entry;
use crate::onagre::Mode;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub async fn get_cached_entries(modes: Vec<Mode>) -> HashMap<Mode, Vec<Entry>> {
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

async fn get_cache(mode: &Mode) -> Vec<Entry> {
    let cache_file = get_cache_mode_path(mode);
    if let Ok(file_content) = fs::read_to_string(cache_file) {
        let entries: Result<Vec<Entry>, serde_json::Error> = serde_json::from_str(&file_content);
        entries.unwrap_or_default()
    } else {
        Default::default()
    }
}

pub fn flush_mode_cache(mode: &Mode, entries: &[Entry]) {
    let entries: Vec<&Entry> = entries.iter().take(50).collect();

    if let Ok(content) = serde_json::to_string(&entries) {
        let _ = std::fs::write(get_cache_mode_path(mode), content);
    }
}

fn get_cache_mode_path(mode: &Mode) -> PathBuf {
    let cache_filename = format!("onagre-{}", mode.to_string().to_lowercase());
    let cache_file = dirs::cache_dir().unwrap();
    cache_file.join(cache_filename)
}
