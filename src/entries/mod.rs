pub(crate) mod desktop;
mod generic;

use crate::THEME;
use crate::{Message, Mode};
use desktop::DesktopEntry;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use iced::Container;
use rayon::prelude::*;
use std::collections::HashMap;

pub trait Entries<T> {
    fn get_matches(&self, input: &str, matcher: &SkimMatcherV2) -> Vec<T>;
    fn default_matches(&self) -> Vec<T>;
}

pub trait ToRow<'a> {
    fn to_row(&self) -> Container<'a, Message> {
        self.as_row()
            .width(THEME.rows.lines.default.width.into())
            .height(THEME.rows.lines.default.height.into())
            .style(&THEME.rows.lines.default)
            .padding(THEME.rows.lines.default.padding)
    }

    fn to_row_selected(&self) -> Container<'a, Message> {
        self.as_row()
            .width(THEME.rows.lines.selected.width.into())
            .height(THEME.rows.lines.selected.height.into())
            .style(&THEME.rows.lines.selected)
            .padding(THEME.rows.lines.selected.padding)
    }

    fn as_row(&self) -> Container<'a, Message>;
}

#[derive(Debug, Default, Clone)]
pub struct EntriesState {
    pub desktop_entries: Vec<DesktopEntry>,
    pub custom_entries: HashMap<String, Vec<String>>,
}

#[derive(Debug, Default, Clone)]
pub struct MatchedEntries {
    pub desktop_entries: Vec<DesktopEntry>,
    pub custom_entries: HashMap<String, Vec<String>>,
}

impl EntriesState {
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
