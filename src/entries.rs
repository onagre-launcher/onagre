use crate::desktop::DesktopEntryInContent;
use std::collections::HashMap;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

#[derive(Debug, Clone)]
pub struct DesktopEntry {
    pub name: String,
    pub exec: String,
}

#[derive(Debug, Default, Clone)]
pub struct Entries {
    pub desktop_entries: Vec<DesktopEntry>,
    pub custom_entries: HashMap<String, Vec<String>>,
}

#[derive(Debug, Default, Clone)]
pub struct MatchedEntries {
    pub desktop_entries: Vec<DesktopEntry>,
    pub custom_entries: HashMap<String, Vec<String>>,
}

impl Entries {
    pub fn get_matches(&self, input: &str) -> Vec<DesktopEntry> {
        let matcher = SkimMatcherV2::default().ignore_case();

        self.desktop_entries
            .iter()
            .map(|entry| (entry, matcher.fuzzy_match(&entry.name, input).unwrap_or(0)))
            .filter(|(_, score)| *score > 10i64)
            .map(|(entry, _)| entry)
            .take(50)
            .cloned()
            .collect()
    }

    pub fn get_matches_custom_mode(&self, mode_key: &str, input: &str) -> Vec<String> {
        let matcher = SkimMatcherV2::default().ignore_case();

        self.custom_entries.get(mode_key).unwrap()
            .iter()
            .map(|entry| (entry, matcher.fuzzy_match(&entry, input).unwrap_or(0)))
            .filter(|(_, score)| *score > 10i64)
            .map(|(entry, _)| entry)
            .take(50)
            .cloned()
            .collect()
    }
}

impl From<&DesktopEntryInContent> for DesktopEntry {
    fn from(desktop_entry: &DesktopEntryInContent) -> Self {
        DesktopEntry {
            name: desktop_entry.name.clone(),
            exec: desktop_entry.exec.clone(),
        }
    }
}
