use iced::alignment::Horizontal;
use iced::{Alignment, Container, Length, Row, Text};

use crate::app::Message;
use crate::db::entity::DesktopEntryEntity;
use crate::entries::AsEntry;
use crate::freedesktop::IconPath;

impl<'a> AsEntry<'a> for DesktopEntryEntity {
    fn as_row(&self, row: Row<'a, Message>) -> Container<'a, Message> {
        Container::new(
            row.push(
                Text::new(&self.name)
                    .width(Length::Fill)
                    .horizontal_alignment(Horizontal::Left),
            )
            .spacing(10)
            .align_items(Alignment::Center),
        )
    }

    fn get_icon(&self) -> Option<IconPath> {
        IconPath::from_path(&self.icon)
    }
}
