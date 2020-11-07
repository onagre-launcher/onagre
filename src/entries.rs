use crate::desktop::DesktopEntryInContent;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use std::collections::HashMap;
use crate::Mode;
use rayon::prelude::*;
use std::sync::Arc;

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
    pub fn new(modes: &[Mode]) -> Self {
        let mut custom_entries = HashMap::new();

        modes.iter()
            .map(Mode::as_str)
            .map(str::to_string)
            .for_each(|mode_name| {
                custom_entries.insert(mode_name, Vec::<String>::new());
            });

        Self {
            desktop_entries: vec![],
            custom_entries
        }
    }

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

    pub fn take_50_desktop_entries(&self) -> Vec<DesktopEntry> {
        self.desktop_entries
            .iter()
            .take(50)
            .cloned()
            .collect()
    }

    pub fn take_50_custom_entries(&self, mode_key: &str) -> Vec<String> {
        self.custom_entries
            .get(mode_key)
            .unwrap()
            .iter()
            .take(50)
            .cloned()
            .collect()
    }

    pub fn get_matches_custom_mode(&self, mode_key: &str, input: &str) -> Vec<String> {
        let matcher = SkimMatcherV2::default().ignore_case();

        if let Some(entries) = self.custom_entries.get(mode_key) {
            let entries: Vec<&String> = entries
                .par_iter()
                .map(|entry| (entry, matcher.fuzzy_match(&entry, input).unwrap_or(0)))
                .filter(|(_, score)| *score > 10i64)
                .map(|(entry, _)| entry)
                .collect();

            entries.iter()
                .take(50)
                .map(|entry| entry.to_owned().to_owned())
                .collect()

        } else {
            // FIXME, we need to keep previous result somewhere
            vec![]
        }
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
