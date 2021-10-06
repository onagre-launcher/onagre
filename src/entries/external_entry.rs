use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use iced::alignment::Horizontal;
use iced::{Alignment, Container, Length, Row, Text};
use serde::{Deserialize, Serialize};

use crate::app::Message;
use crate::entries::AsEntry;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalEntries(Vec<ExternalCommandEntry>);

impl ExternalEntries {
    pub fn new(entries: Vec<ExternalCommandEntry>) -> Self {
        Self { 0: entries }
    }

    pub fn get(&self) -> &[ExternalCommandEntry] {
        &self.0
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn extend_from_slice(&mut self, other: &ExternalEntries) {
        self.0.extend_from_slice(other.get());
    }

    pub fn match_external(
        &self,
        search_term: &str,
        matcher: &SkimMatcherV2,
    ) -> Vec<ExternalCommandEntry> {
        let mut entries: Vec<(&ExternalCommandEntry, i64)> = self
            .0
            .iter()
            .map(|entry| {
                (
                    entry,
                    matcher.fuzzy_match(&entry.value, search_term).unwrap_or(0),
                )
            })
            .filter(|(_, score)| *score > 10i64)
            .collect();

        entries.sort_unstable_by(|(_, prev), (_, cur)| {
            // sort by match score + entry weight
            cur.cmp(prev)
        });

        entries
            .into_iter()
            .map(|(entry, _score)| entry)
            .cloned()
            .collect()
    }
}

impl Default for ExternalEntries {
    fn default() -> Self {
        ExternalEntries(Vec::with_capacity(0))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalCommandEntry {
    pub value: String,
}

impl ExternalCommandEntry {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}

impl<'a> AsEntry<'a> for ExternalCommandEntry {
    fn as_row(&self) -> Container<'a, Message> {
        Container::new(
            Row::new()
                .push(
                    Text::new(&self.value)
                        .width(Length::Fill)
                        .horizontal_alignment(Horizontal::Left),
                )
                .spacing(10)
                .align_items(Alignment::Center),
        )
    }
}
