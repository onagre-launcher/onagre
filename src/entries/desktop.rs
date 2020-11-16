use crate::entries::{Entries, ToRow};
use crate::freedesktop::desktop::DesktopEntryInContent;
use crate::freedesktop::icons::{Extension, IconFinder, IconPath};
use crate::Message;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use iced::{Container, HorizontalAlignment, Image, Length, Row, Text};
use iced_native::Svg;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct DesktopEntry {
    /// The number of time this entry has been launched already
    pub weight: u32,
    pub display_name: String,
    pub exec: String,
    pub search_terms: String,
    pub icon: Option<IconPath>,
}

impl Entries<DesktopEntry> for Vec<Rc<DesktopEntry>> {
    fn get_matches(&self, input: &str, matcher: &SkimMatcherV2) -> Vec<Rc<DesktopEntry>> {
        let mut entries: Vec<(&Rc<DesktopEntry>, i64)> = self
            .iter()
            .map(|entry| {
                (
                    entry,
                    matcher.fuzzy_match(&entry.search_terms, input).unwrap_or(0),
                )
            })
            .filter(|(_, score)| *score > 10i64)
            .collect();

        entries.sort_unstable_by(|(_, prev), (_, cur)| cur.cmp(prev));

        entries
            .iter()
            .take(50)
            .map(|(entry, _)| Rc::clone(entry))
            .collect()
    }

    fn default_matches(&self) -> Vec<Rc<DesktopEntry>> {
        self.iter().take(50).map(|rc| Rc::clone(rc)).collect()
    }
}

impl<'a> ToRow<'a> for Rc<DesktopEntry> {
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

impl From<DesktopEntryInContent> for DesktopEntry {
    fn from(desktop_entry: DesktopEntryInContent) -> Self {
        let mut search_terms = desktop_entry.name.clone();
        if let Some(keywords) = &desktop_entry.keywords {
            search_terms.push_str(&keywords.replace(";", " "));
        }

        DesktopEntry {
            weight: 0,
            display_name: desktop_entry.name,
            search_terms,
            exec: desktop_entry.exec,
            icon: None,
        }
    }
}

impl DesktopEntry {
    pub fn with_icon(desktop_entry: DesktopEntryInContent, finder: &IconFinder) -> Self {
        let icon = desktop_entry.get_icon(32, finder).ok();
        let mut entry = Self::from(desktop_entry);
        entry.icon = icon;
        entry
    }
}
