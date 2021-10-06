use iced::Container;

use crate::app::Message;
use crate::db::entity::DesktopEntryEntity;
use crate::entries::external_entry::ExternalEntries;
use crate::entries::pop_entry::PopSearchResult;
use crate::THEME;

pub(crate) mod db_entry;
pub(crate) mod external_entry;
pub(crate) mod pop_entry;

pub(crate) trait AsEntry<'a> {
    fn to_row_selected(&self) -> Container<'a, Message> {
        self.as_row()
            .width(THEME.rows.lines.selected.width.into())
            .height(THEME.rows.lines.selected.height.into())
            .style(&THEME.rows.lines.selected)
            .padding(THEME.rows.lines.selected.padding)
    }
    fn to_row(&self) -> Container<'a, Message> {
        self.as_row()
            .width(THEME.rows.lines.default.width.into())
            .height(THEME.rows.lines.default.height.into())
            .style(&THEME.rows.lines.default)
            .padding(THEME.rows.lines.default.padding)
    }

    fn as_row(&self) -> Container<'a, Message>;
}

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
