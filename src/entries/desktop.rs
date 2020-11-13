use crate::entries::{Entries, ToRow};
use crate::freedesktop::desktop::DesktopEntryInContent;
use crate::freedesktop::icons::{Extension, IconFinder, IconPath};
use crate::Message;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use iced::{Container, HorizontalAlignment, Image, Length, Row, Text};
use iced_native::Svg;
use rayon::prelude::*;

#[derive(Debug, Clone)]
pub struct DesktopEntry {
    /// The number of time this entry has been launched already
    pub weight: u32,
    pub display_name: String,
    pub exec: String,
    pub search_terms: String,
    pub icon: Option<IconPath>,
}

impl Entries<DesktopEntry> for Vec<DesktopEntry> {
    fn get_matches(&self, input: &str, matcher: &SkimMatcherV2) -> Vec<DesktopEntry> {
        let mut entries: Vec<(&DesktopEntry, i64)> = self
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
            .cloned()
            .map(|(entry, _)| entry.to_owned())
            .collect()
    }

    fn default_matches(&self) -> Vec<DesktopEntry> {
        self.iter().take(50).cloned().collect()
    }
}

impl<'a> ToRow<'a> for DesktopEntry {
    fn as_row(&self) -> Container<'a, Message> {
        let mut row = Row::new();
        row = if let Some(icon) = &self.icon {
            match &icon.extension {
                Extension::SVG => row.push(
                    Svg::from_path(&icon.path)
                        .height(Length::Units(32))
                        .width(Length::Units(32)),
                ),
                Extension::PNG => row.push(
                    Image::new(&icon.path)
                        .height(Length::Units(32))
                        .width(Length::Units(32)),
                ),
            }
        } else {
            row
        };

        row = row.push(
            Text::new(&self.display_name)
                .width(Length::Fill)
                .horizontal_alignment(HorizontalAlignment::Left),
        );

        Container::new(row)
    }
}

impl From<&DesktopEntryInContent> for DesktopEntry {
    fn from(desktop_entry: &DesktopEntryInContent) -> Self {
        let mut search_terms = desktop_entry.name.clone();
        if let Some(keywords) = &desktop_entry.keywords {
            search_terms.push_str(&keywords.replace(";", " "));
        }

        DesktopEntry {
            weight: 0,
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
