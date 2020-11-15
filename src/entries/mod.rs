pub(crate) mod desktop;
mod generic;

use crate::THEME;
use crate::{Message, Mode};
use desktop::DesktopEntry;
use fuzzy_matcher::skim::SkimMatcherV2;
use iced::Container;
use std::collections::HashMap;

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

    pub fn get_mode_entries(&self, mode_key: &str) -> &Vec<String> {
        self.custom_entries.get(mode_key).unwrap()
    }
}
