use crate::freedesktop::desktop::DesktopEntryInContent;
use crate::freedesktop::icons::{IconFinder, IconPath};
use crate::Mode;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use rayon::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct DesktopEntry {
    pub display_name: String,
    pub exec: String,
    pub search_terms: String,
    pub icon: Option<IconPath>,
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

        modes.iter().map(Mode::to_string).for_each(|mode_name| {
            custom_entries.insert(mode_name, Vec::<String>::new());
        });

        Self {
            desktop_entries: vec![],
            custom_entries,
        }
    }

    pub fn get_matches(&self, input: &str, matcher: &SkimMatcherV2) -> Vec<DesktopEntry> {
        let mut entries: Vec<(&DesktopEntry, i64)> = self
            .desktop_entries
            .iter()
            .map(|entry| {
                (
                    entry,
                    matcher.fuzzy_match(&entry.search_terms, input).unwrap_or(0),
                )
            })
            .filter(|(_, score)| *score > 10i64)
            .collect();

        entries.par_sort_unstable_by(|(_, prev), (_, cur)| cur.cmp(prev));

        entries
            .iter()
            .take(50)
            .map(|(entry, _)| entry.to_owned())
            .cloned()
            .collect()
    }

    pub fn take_50_desktop_entries(&self) -> Vec<DesktopEntry> {
        self.desktop_entries.iter().take(50).cloned().collect()
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

    pub fn get_matches_custom_mode(
        &self,
        mode_key: &str,
        input: &str,
        matcher: &SkimMatcherV2,
    ) -> Vec<String> {
        if let Some(entries) = self.custom_entries.get(mode_key) {
            let mut entries: Vec<(&String, i64)> = entries
                .par_iter()
                .map(|entry| (entry, matcher.fuzzy_match(&entry, &input).unwrap_or(0)))
                .filter(|(_, score)| *score > 10i64)
                .collect();

            // sort by match score
            entries.par_sort_unstable_by(|(_, prev), (_, cur)| cur.cmp(prev));

            // Take only the first results oredered
            entries
                .iter()
                .map(|(entry, _)| entry.to_owned().to_owned())
                .take(50)
                .collect()
        } else {
            // FIXME, we need to keep previous result somewhere
            vec![]
        }
    }
}

impl From<&DesktopEntryInContent> for DesktopEntry {
    fn from(desktop_entry: &DesktopEntryInContent) -> Self {
        let mut search_terms = desktop_entry.name.clone();
        if let Some(keywords) = &desktop_entry.keywords {
            search_terms.push_str(&keywords.replace(";", " "));
        }

        DesktopEntry {
            display_name: desktop_entry.name.clone(),
            search_terms,
            exec: desktop_entry.exec.clone(),
            icon: None,
        }
    }
}

impl DesktopEntry {
    pub fn with_icon(desktop_entry: &DesktopEntryInContent, finder: &IconFinder) -> Self {
        let mut entry = Self::from(desktop_entry);
        let icon = desktop_entry.get_icon(32, finder).ok();
        entry.icon = icon;
        entry
    }
}
