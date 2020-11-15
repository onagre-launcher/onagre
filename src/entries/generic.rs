use crate::entries::{Entries, ToRow};
use crate::Message;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use iced::{Container, HorizontalAlignment, Length, Row, Text};
use rayon::prelude::*;

pub type GenericEntries = Vec<String>;
pub type GenericEntry = String;

impl<'a> ToRow<'a> for GenericEntry {
    fn as_row(&self) -> Container<'a, Message> {
        Container::new(
            Row::new().push(
                Text::new(self.as_str())
                    .width(Length::Fill)
                    .horizontal_alignment(HorizontalAlignment::Left),
            ),
        )
    }
}

impl Entries<GenericEntry> for GenericEntries {
    fn get_matches(&self, input: &str, matcher: &SkimMatcherV2) -> Vec<String> {
        let mut entries: Vec<(&String, i64)> = self
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
    }

    fn default_matches(&self) -> Vec<String> {
        self.iter().take(50).cloned().collect()
    }
}
