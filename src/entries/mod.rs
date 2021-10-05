use iced::{Alignment, Container, Image, Length, Row, Svg, Text};
use pop_launcher::IconSource;

use crate::backend::PopSearchResult;
use crate::freedesktop::{Extension, IconPath};
use crate::onagre::Message;
use crate::{ICON_FINDER, SETTINGS, THEME};
use iced_native::alignment::Horizontal;
use std::path::PathBuf;

impl<'a> PopSearchResult {
    pub(crate) fn to_row_selected(&self) -> Container<'a, Message> {
        self.as_row()
            .width(THEME.rows.lines.selected.width.into())
            .height(THEME.rows.lines.selected.height.into())
            .style(&THEME.rows.lines.selected)
            .padding(THEME.rows.lines.selected.padding)
    }

    pub(crate) fn to_row(&self) -> Container<'a, Message> {
        self.as_row()
            .width(THEME.rows.lines.default.width.into())
            .height(THEME.rows.lines.default.height.into())
            .style(&THEME.rows.lines.default)
            .padding(THEME.rows.lines.default.padding)
    }

    fn as_row(&self) -> Container<'a, Message> {
        let icon_path = self
            .icon
            .as_ref()
            .map(|icon| {
                ICON_FINDER.as_ref().map(|finder| {
                    let path_str = match icon {
                        IconSource::Name(icon) => icon,
                        IconSource::Mime(icon) => icon,
                        IconSource::Window(_) => todo!("What is this ?"),
                    };

                    let path = PathBuf::from(path_str.to_string());
                    if path.is_absolute() && path.exists() {
                        let extension = path.extension().unwrap().to_str().unwrap();
                        let extension = match extension {
                            "svg" => Some(Extension::Svg),
                            "png" => Some(Extension::Png),
                            _ => None,
                        };

                        extension.map(|extension| IconPath { path, extension })
                    } else {
                        finder.lookup(path_str, 32)
                    }
                })
            })
            .flatten()
            .flatten();

        let mut row = if let (Some(_icon_enabled), Some(icon_path)) = (&SETTINGS.icons, icon_path) {
            match &icon_path.extension {
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
