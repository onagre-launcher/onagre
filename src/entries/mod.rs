use iced::{Alignment, Container, Image, Length, Row, Svg, Text};

use crate::db::desktop_entry::DesktopEntryEntity;
use crate::db::run::RunCommandEntity;
use crate::db::web::WebEntity;
use crate::entries::pop_entry::PopSearchResult;
use crate::freedesktop::{Extension, IconPath};
use crate::ui::app::Message;
use crate::THEME;
use iced_native::alignment::Horizontal;

pub(crate) mod db_entry;
pub(crate) mod pop_entry;

#[derive(Debug)]
pub struct EntryCache {
    pub pop_search: Vec<PopSearchResult>,
    pub de_history: Vec<DesktopEntryEntity>,
    pub web_history: Vec<WebEntity>,
    pub terminal: Vec<RunCommandEntity>,
}

pub(crate) trait AsEntry<'a> {
    fn to_row_selected(&self, row: Row<'a, Message>) -> Container<'a, Message> {
        self.as_row(row)
            .width(Length::Fill)
            .height(Length::Shrink)
            .style(&THEME.rows.lines.selected)
            .padding(THEME.rows.lines.selected.padding)
    }

    fn to_row_unselected(&self, row: Row<'a, Message>) -> Container<'a, Message> {
        self.as_row(row)
            .width(Length::Fill)
            .height(Length::Shrink)
            .style(&THEME.rows.lines.default)
            .padding(THEME.rows.lines.default.padding)
    }

    fn to_row(&self, selected: Option<usize>, idx: usize) -> Container<'a, Message> {
        let icon = THEME
            .icon_theme
            .as_ref()
            .and_then(|_| self.get_icon())
            .map(|icon| match &icon.extension {
                Extension::Svg => Row::new().push(
                    Svg::from_path(&icon.path)
                        .height(Length::Units(THEME.icon_size))
                        .width(Length::Units(THEME.icon_size)),
                ),

                Extension::Png => Row::new().push(
                    Image::new(&icon.path)
                        .height(Length::Units(THEME.icon_size))
                        .width(Length::Units(THEME.icon_size)),
                ),
            })
            .unwrap_or_else(Row::new)
            .spacing(0);

        match selected {
            Some(selected) if idx == selected => self.to_row_selected(icon),
            _ => self.to_row_unselected(icon),
        }
    }

    fn as_row(&self, row: Row<'a, Message>) -> Container<'a, Message> {
        Container::new(
            row.push(
                Text::new(self.get_display_name())
                    .width(Length::Fill)
                    .horizontal_alignment(Horizontal::Left),
            )
            .spacing(10)
            .align_items(Alignment::Center),
        )
    }
    fn get_display_name(&self) -> &str;
    fn get_icon(&self) -> Option<IconPath>;
}
