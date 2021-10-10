use iced::{Alignment, Container, Image, Length, Row, Svg, Text};

use crate::app::Message;
use crate::db::entity::DesktopEntryEntity;
use crate::entries::external_entry::ExternalEntries;
use crate::entries::pop_entry::PopSearchResult;
use crate::freedesktop::{Extension, IconPath};
use crate::{SETTINGS, THEME};
use iced::alignment::Horizontal;

pub(crate) mod db_entry;
pub(crate) mod external_entry;
pub(crate) mod pop_entry;

#[derive(Debug)]
pub enum Entries {
    Pop(Vec<PopSearchResult>),
    External(ExternalEntries),
    History(Vec<DesktopEntryEntity>),
    None,
}

impl Entries {
    pub(crate) fn len(&self) -> usize {
        match self {
            Entries::Pop(pop_entry) => pop_entry.len(),
            Entries::External(external_entry) => external_entry.len(),
            Entries::None => 0,
            Entries::History(history) => history.len(),
        }
    }

    pub fn _empty(&self) -> bool {
        self.len() == 0
    }
}

pub(crate) trait AsEntry<'a> {
    fn to_row_selected(&self, row: Row<'a, Message>) -> Container<'a, Message> {
        self.as_row(row)
            .width(THEME.rows.lines.selected.width.into())
            .height(THEME.rows.lines.selected.height.into())
            .style(&THEME.rows.lines.selected)
            .padding(THEME.rows.lines.selected.padding)
    }

    fn to_row_unselected(&self, row: Row<'a, Message>) -> Container<'a, Message> {
        self.as_row(row)
            .width(THEME.rows.lines.default.width.into())
            .height(THEME.rows.lines.default.height.into())
            .style(&THEME.rows.lines.default)
            .padding(THEME.rows.lines.default.padding)
    }

    fn to_row(&self, selected: usize, idx: usize) -> Container<'a, Message> {
        let icon = SETTINGS
            .icons
            .as_ref()
            .map(|_| self.get_icon())
            .flatten()
            .map(|icon| match &icon.extension {
                Extension::Svg => Row::new().push(
                    Svg::from_path(&icon.path)
                        .height(Length::Units(32))
                        .width(Length::Units(32)),
                ),

                Extension::Png => Row::new().push(
                    Image::new(&icon.path)
                        .height(Length::Units(32))
                        .width(Length::Units(32)),
                ),
            })
            .unwrap_or_else(Row::new);

        if idx == selected {
            self.to_row_selected(icon)
        } else {
            self.to_row_unselected(icon)
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
