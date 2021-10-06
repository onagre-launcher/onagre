use crate::app::Message;
use crate::db::entity::DesktopEntryEntity;
use crate::entries::AsEntry;
use crate::freedesktop::{Extension, IconPath};
use crate::{ICON_FINDER, SETTINGS};
use iced::alignment::Horizontal;
use iced::{Alignment, Container, Image, Length, Row, Svg, Text};

use std::path::PathBuf;

impl<'a> AsEntry<'a> for DesktopEntryEntity {
    fn as_row(&self) -> Container<'a, Message> {
        let mut row = if SETTINGS.icons.is_some() {
            let icon_path = ICON_FINDER
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
                .flatten();

            match icon_path {
                Some(icon_path) => match &icon_path.extension {
                    Extension::Svg => Row::new().push(
                        Svg::from_path(&icon_path.path)
                            .height(Length::Units(32))
                            .width(Length::Units(32)),
                    ),

                    Extension::Png => Row::new().push(
                        Image::new(&icon_path.path)
                            .height(Length::Units(32))
                            .width(Length::Units(32)),
                    ),
                },
                None => Row::new(),
            }
        } else {
            Row::new()
        };

        row = row
            .push(
                Text::new(&self.name)
                    .width(Length::Fill)
                    .horizontal_alignment(Horizontal::Left),
            )
            .spacing(10)
            .align_items(Alignment::Center);

        Container::new(row)
    }
}
