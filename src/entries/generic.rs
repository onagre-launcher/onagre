use crate::entries::{Entries, ToRow};
use crate::Message;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use iced::{Container, HorizontalAlignment, Length, Row, Text};
use std::rc::Rc;

pub type GenericEntries = Vec<Rc<String>>;

impl<'a> ToRow<'a> for Rc<String> {
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

impl Entries<String> for GenericEntries {
    fn get_matches(&self, input: &str, matcher: &SkimMatcherV2) -> Vec<Rc<String>> {
        let mut entries: Vec<(&Rc<String>, i64)> = self
            .iter()
            .map(|entry| (entry, matcher.fuzzy_match(&entry, &input).unwrap_or(0)))
            .filter(|(_, score)| *score > 10i64)
            .collect();

        // sort by match score
        entries.sort_unstable_by(|(_, prev), (_, cur)| cur.cmp(prev));

        // Take only the first results ordered
        entries
            .iter()
            .map(|(entry, _)| entry)
            .take(50)
            .map(|entry| Rc::clone(entry))
            .collect()
    }

    fn default_matches(&self) -> Vec<Rc<String>> {
        self.iter().take(50).map(|this| Rc::clone(this)).collect()
    }
}
