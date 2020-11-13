use crate::entries::ToRow;
use crate::Message;
use iced::{Container, HorizontalAlignment, Length, Row, Text};

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
