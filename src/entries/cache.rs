use crate::entries::DesktopEntry;
use crate::freedesktop::icons::{IconPath};
use std::path::PathBuf;
use anyhow::Result;

pub struct Cacher;

impl Cacher {
    fn flush_desktop_entries(entries: &Vec<DesktopEntry>) -> Result<()> {
        let lines: String = entries.into_iter()
            .map(|entry| entry.to_cache())
            .collect::<Vec<String>>()
            .join("\n");

        let cache = dirs::cache_dir().unwrap().join("onagre-drun");
        std::fs::write(cache, lines)
            .map_err(|err| anyhow!("Fyle system erro writing cache {}", err))
    }

    fn get_desktop_entries() -> Result<Vec<DesktopEntry>> {
        let cache = dirs::cache_dir().unwrap().join("onagre-drun");
        std::fs::read_to_string(cache)
            .map(|content| content.split("\n")
                .into_iter()
                .map(DesktopEntry::from_cache)
                .collect()
            )
            .map_err(|err| anyhow!("Unable to get destkop entries from cache file {}", err))
    }
}

trait Cache {
    fn to_cache(&self) -> String;
    fn from_cache(line: &str) -> Self;
}

impl Cache for IconPath {
    fn to_cache(&self) -> String {
        self.path.to_string_lossy().to_string()
    }

    fn from_cache(line: &str) -> Self {
        IconPath::try_from(PathBuf::from(line)).expect("Invalid icon path in cache file")
    }
}

impl Cache for Option<IconPath> {
    fn to_cache(&self) -> String {
        match self {
            None => "".to_string(),
            Some(icon) => icon.to_cache(),
        }
    }

    fn from_cache(line: &str) -> Self {
        match line {
            "" => None,
            other => Some(IconPath::from_cache(other))
        }
    }
}

impl Cache for DesktopEntry {
    fn to_cache(&self) -> String {
        format!("{},{},{},{},{}", self.weight, self.display_name, self.exec, self.search_terms, self.icon.to_cache())
    }

    fn from_cache(line: &str) -> Self {
        let values: Vec<&str> = line.split(',').collect();
        DesktopEntry {
            weight: str::parse::<u32>(values[0]).unwrap(),
            display_name: values[1].to_string(),
            exec: values[2].to_string(),
            search_terms: values[3].to_string(),
            icon: Cache::from_cache(values[4]),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::entries::DesktopEntry;
    use crate::freedesktop::icons::{IconPath, Extension};
    use std::path::PathBuf;
    use crate::entries::cache::Cacher;

    #[test]
    fn write_to_cache() {
        let entries = vec![
            DesktopEntry {
                weight: 0,
                display_name: "App".to_string(),
                exec: "echo App".to_string(),
                search_terms: "App theApp application".to_string(),
                icon: None,
            },
            DesktopEntry {
                weight: 0,
                display_name: "Onagre".to_string(),
                exec: "onagre".to_string(),
                search_terms: "file launcher awesome app ".to_string(),
                icon: Some(IconPath {
                    path: PathBuf::from("/home/user/hello.png"),
                    extension: Extension::SVG,
                }),
            }
        ];

        Cacher::flush_desktop_entries(&entries).unwrap();

        let cache_file = dirs::cache_dir().unwrap().join("onagre-drun");

        let cached_entries = Cacher::get_desktop_entries();
        let cached_entries = cached_entries.unwrap();

        assert_eq!(cached_entries.len(), 2);
        assert_eq!(cached_entries[0].display_name, "App");
        assert_eq!(cached_entries[1].display_name, "Onagre");
    }
}


