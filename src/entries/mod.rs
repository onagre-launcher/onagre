pub mod cache;

use crate::freedesktop::desktop::DesktopEntryInContent;
use crate::freedesktop::icons::{Extension, IconFinder, IconPath};
use crate::onagre::{Message, Mode};
use crate::SETTINGS;
use crate::THEME;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use iced::{Container, HorizontalAlignment, Image, Length, Row, Text};
use iced_native::Svg;
use std::collections::HashMap;

// Calling Hashmap::get(key: &Mode).unwrap() should always be safe since we initialize all
// known mode on startup and never add or remove them at runtime
#[derive(Debug, Default, Clone)]
pub struct EntriesState {
    pub mode_entries: HashMap<Mode, Vec<Entry>>,
    pub mode_matches: HashMap<Mode, Vec<usize>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    pub weight: u32,
    pub display_name: String,
    pub exec: Option<String>,
    pub search_terms: Option<String>,
    pub icon: Option<IconPath>,
}

impl PartialEq<Entry> for Entry {
    fn eq(&self, other: &Entry) -> bool {
        self.display_name == other.display_name
    }
}

impl Eq for Entry {}

impl PartialOrd<Entry> for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.display_name.cmp(&other.display_name))
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.display_name.cmp(&other.display_name)
    }
}

impl<'a> Entry {
    pub fn from_custom_entry(display_name: String) -> Self {
        Self {
            weight: 0,
            display_name,
            exec: None,
            search_terms: None,
            icon: None,
        }
    }

    pub fn from_desktop_entry(
        desktop_entry: DesktopEntryInContent,
        finder: Option<&IconFinder>,
    ) -> Self {
        let search_terms = desktop_entry
            .keywords
            .as_ref()
            .map(|keywords| format!("{} {}", &desktop_entry.name, keywords.replace(";", " ")));

        let icon = match finder {
            None => None,
            Some(finder) => desktop_entry.get_icon(32, finder),
        };

        let exec = Some(desktop_entry.exec);
        let display_name = desktop_entry.name;
        Entry {
            weight: 0,
            display_name,
            exec,
            search_terms,
            icon,
        }
    }

    // get the search term for desktop entries
    // or the display name if custom search terms can't be found
    fn get_search_terms(&self) -> &str {
        if let Some(terms) = &self.search_terms {
            terms
        } else {
            &self.display_name
        }
    }

    pub(crate) fn to_row(&self) -> Container<'a, Message> {
        self.as_row()
            .width(THEME.rows.lines.default.width.into())
            .height(THEME.rows.lines.default.height.into())
            .style(&THEME.rows.lines.default)
            .padding(THEME.rows.lines.default.padding)
    }

    pub(crate) fn to_row_selected(&self) -> Container<'a, Message> {
        self.as_row()
            .width(THEME.rows.lines.selected.width.into())
            .height(THEME.rows.lines.selected.height.into())
            .style(&THEME.rows.lines.selected)
            .padding(THEME.rows.lines.selected.padding)
    }

    fn as_row(&self) -> Container<'a, Message> {
        let mut row = if SETTINGS.icons.is_some() && self.icon.is_some() {
            let icon = self.icon.as_ref().unwrap();
            match &icon.extension {
                Extension::SVG => Row::new().push(
                    Svg::from_path(&icon.path)
                        .height(Length::Units(32))
                        .width(Length::Units(32)),
                ),

                Extension::PNG => Row::new().push(
                    Image::new(&icon.path)
                        .height(Length::Units(32))
                        .width(Length::Units(32)),
                ),
            }
        } else {
            Row::new()
        };

        row = row
            .push(
                Text::new(&self.display_name)
                    .width(Length::Fill)
                    .horizontal_alignment(HorizontalAlignment::Left),
            )
            .spacing(10)
            .align_items(iced::Align::Center);

        Container::new(row)
    }
}

pub trait Entries<T> {
    fn get_matches(&self, input: &str, matcher: &SkimMatcherV2) -> Vec<usize>;
    fn default_matches(&self) -> Vec<usize>;
}

impl Entries<Entry> for Vec<Entry> {
    fn get_matches(&self, input: &str, matcher: &SkimMatcherV2) -> Vec<usize> {
        let mut entries: Vec<(usize, &Entry, i64)> = self
            .iter()
            .enumerate()
            .map(|(idx, entry)| {
                (
                    idx,
                    entry,
                    matcher
                        .fuzzy_match(&entry.get_search_terms(), input)
                        .unwrap_or(0),
                )
            })
            .filter(|(_, _, score)| *score > 10i64)
            .collect();

        entries.sort_unstable_by(|(_, prev_entry, prev), (_, cur_entry, cur)| {
            // sort by match score + entry weight
            (cur + cur_entry.weight as i64).cmp(&(prev + prev_entry.weight as i64))
        });

        // Take only the first results ordered
        entries.iter().take(50).map(|(idx, _, _)| *idx).collect()
    }

    fn default_matches(&self) -> Vec<usize> {
        self.iter()
            .enumerate()
            .take(50)
            .map(|(idx, _)| idx)
            .collect()
    }
}
