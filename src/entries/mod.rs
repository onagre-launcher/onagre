use iced::{Alignment, Container, Image, Length, Row, Svg, Text};
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Mutex;

use crate::db::desktop_entry::DesktopEntryEntity;
use crate::db::plugin::PluginCommandEntity;
use crate::db::web::WebEntity;
use crate::db::Database;
use crate::freedesktop::{Extension, IconPath};
use crate::ui::app::Message;
use crate::{db, THEME};
use iced_native::alignment::Horizontal;
use once_cell::sync::OnceCell;

pub(crate) mod db_entry;
pub(crate) mod pop_entry;

type History<T> = Mutex<HashMap<String, Rc<Vec<T>>>>;

#[derive(Debug)]
pub struct Cache {
    pub db: Database,
    de_history: OnceCell<Vec<DesktopEntryEntity>>,
    web_history: History<WebEntity>,
    plugin_history: History<PluginCommandEntity>,
}

impl Default for Cache {
    fn default() -> Self {
        Self {
            db: Database::default(),
            de_history: OnceCell::new(),
            web_history: Mutex::new(Default::default()),
            plugin_history: Mutex::new(Default::default()),
        }
    }
}

impl Cache {
    pub fn de_history(&self) -> &Vec<DesktopEntryEntity> {
        self.de_history.get_or_init(|| {
            self.db
                .get_all::<DesktopEntryEntity>(db::desktop_entry::COLLECTION)
        })
    }

    pub fn de_len(&self) -> usize {
        self.de_history.get().map(|de| de.len()).unwrap_or(0)
    }

    pub fn plugin_history(&self, plug_name: &str) -> Rc<Vec<PluginCommandEntity>> {
        let mut history = self.plugin_history.lock().unwrap();
        if history.get(plug_name).is_none() {
            let data = self.db.get_all::<PluginCommandEntity>(plug_name);
            history.insert(plug_name.to_string(), Rc::new(data));
        }

        Rc::clone(history.get(plug_name).unwrap())
    }

    pub fn plugin_history_len(&self, plug_name: &str) -> usize {
        self.plugin_history
            .lock()
            .unwrap()
            .get(plug_name)
            .map(|p| p.len())
            .unwrap_or(0)
    }

    pub fn web_history(&self, web_name: &str) -> Rc<Vec<WebEntity>> {
        let mut history = self.web_history.lock().unwrap();
        if history.get(web_name).is_none() {
            let data = self.db.get_all::<WebEntity>(web_name);
            history.insert(web_name.to_string(), Rc::new(data));
        }

        Rc::clone(history.get(web_name).unwrap())
    }

    pub fn web_history_len(&self, web_name: &str) -> usize {
        self.web_history
            .lock()
            .unwrap()
            .get(web_name)
            .map(|p| p.len())
            .unwrap_or(0)
    }
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
