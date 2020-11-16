pub(crate) mod desktop;
mod generic;

use crate::THEME;
use crate::{Message, Mode};
use desktop::DesktopEntry;
use fuzzy_matcher::skim::SkimMatcherV2;
use iced::Container;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Default, Clone)]
pub struct EntriesState {
    pub desktop_entries: Vec<Rc<DesktopEntry>>,
    pub desktop_entries_matches: Vec<Rc<DesktopEntry>>,
    pub custom_entries: HashMap<String, Vec<Rc<String>>>,
    pub custom_entries_matches: HashMap<String, Vec<Rc<String>>>,
}

pub trait Entries<T> {
    fn get_matches(&self, input: &str, matcher: &SkimMatcherV2) -> Vec<Rc<T>>;
    fn default_matches(&self) -> Vec<Rc<T>>;
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
            custom_entries.insert(mode_name, Vec::<Rc<String>>::with_capacity(256));
        });

        Self {
            desktop_entries: Vec::with_capacity(256),
            desktop_entries_matches: Vec::with_capacity(256),
            custom_entries,
            custom_entries_matches: Default::default(),
        }
    }

    pub fn get_mode_entries(&self, mode_key: &str) -> &Vec<Rc<String>> {
        self.custom_entries.get(mode_key).unwrap()
    }
}
