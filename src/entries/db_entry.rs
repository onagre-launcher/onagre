use std::path::PathBuf;

use iced::alignment::Horizontal;
use iced::{Alignment, Container, Length, Row, Text};

use crate::app::Message;
use crate::db::entity::DesktopEntryEntity;
use crate::entries::AsEntry;
use crate::freedesktop::{Extension, IconPath};
use crate::ICON_FINDER;

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
        ICON_FINDER
            .as_ref()
            .map(|finder| {
                let path = PathBuf::from(&self.icon);
                if path.is_absolute() && path.exists() {
                    let extension = path.extension().unwrap().to_str().unwrap();
                    let extension = match extension {
                        "svg" => Some(Extension::Svg),
                        "png" => Some(Extension::Png),
                        _ => None,
                    };

                    extension.map(|extension| IconPath { path, extension })
                } else {
                    finder.lookup(&self.icon, 48)
                }
            })
            .flatten()
    }
}
