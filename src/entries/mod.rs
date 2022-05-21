use iced::{Container, Image, Length, Row, Svg, Text};
use iced_native::Alignment;
use std::borrow::Cow;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Mutex;

use crate::db::desktop_entry::DesktopEntryEntity;
use crate::db::plugin::PluginCommandEntity;
use crate::db::web::WebEntity;
use crate::db::Database;
use crate::icons::{fallback_icon, Extension, IconPath};
use crate::ui::app::Message;
use crate::ui::style::rows::RowStyles;
use crate::{db, THEME};
use iced_native::widget::Column;
use once_cell::sync::OnceCell;

pub(crate) mod db_entry;
pub(crate) mod pop_entry;

type History<T> = Mutex<HashMap<String, Rc<Vec<T>>>>;

#[derive(Debug)]
pub struct Cache<'a> {
    pub db: Database,
    de_history: OnceCell<Vec<DesktopEntryEntity<'a>>>,
    web_history: History<WebEntity<'a>>,
    plugin_history: History<PluginCommandEntity<'a>>,
}

impl Default for Cache<'_> {
    fn default() -> Self {
        Self {
            db: Database::default(),
            de_history: OnceCell::new(),
            web_history: Mutex::new(Default::default()),
            plugin_history: Mutex::new(Default::default()),
        }
    }
}

impl Cache<'_> {
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
    fn to_row(&self, selected: Option<usize>, idx: usize) -> Container<'a, Message> {
        let selected = selected.map(|selected| selected == idx).unwrap_or(false);
        let theme = if selected {
            &THEME.app_container.rows.row_selected
        } else {
            &THEME.app_container.rows.row
        };

        let icon = THEME.icon_theme.as_ref().and_then(|_| self.get_icon());
        let icon = match icon {
            Some(icon) => match &icon.extension {
                Extension::Svg => Container::new(
                    Svg::from_path(&icon.path)
                        .height(Length::Units(THEME.icon_size))
                        .width(Length::Units(THEME.icon_size)),
                ),
                Extension::Png => Container::new(
                    Image::new(&icon.path)
                        .height(Length::Units(THEME.icon_size))
                        .width(Length::Units(THEME.icon_size)),
                ),
            },
            None => Container::new(fallback_icon())
                .height(Length::Units(THEME.icon_size))
                .width(Length::Units(THEME.icon_size)),
        };

        let icon = icon
            .style(&theme.icon)
            .align_y(theme.icon.align_y)
            .align_x(theme.icon.align_x)
            .width(theme.icon.width)
            .height(theme.icon.height)
            .padding(theme.icon.padding.to_iced_padding());

        let row = Row::new()
            .push(icon)
            .height(Length::Fill)
            .width(Length::Fill)
            // See : https://github.com/iced-rs/iced/pull/1044
            .align_items(Alignment::Fill);

        self.as_row(row, theme)
    }

    fn as_row(&self, row: Row<'a, Message>, theme: &'a RowStyles) -> Container<'a, Message> {
        let title_row = Container::new(
            Row::new().push(Text::new(self.get_display_name()).size(theme.title.font_size)),
        )
        .style(&theme.title)
        .padding(theme.title.padding.to_iced_padding())
        .width(theme.title.width)
        .height(theme.title.height)
        .align_x(theme.title.align_x)
        .align_y(theme.title.align_y);

        let description_row = self.get_description().map(|description| {
            Container::new(
                Row::new().push(Text::new(description.as_ref()).size(theme.description.font_size)),
            )
            .style(&theme.description)
            .padding(theme.description.padding.to_iced_padding())
            .width(theme.description.width)
            .height(theme.description.height)
            .align_x(theme.description.align_x)
            .align_y(theme.description.align_y)
        });

        let column = Column::new().push(title_row);

        let column = match description_row {
            Some(description) if !theme.hide_description => column.push(description),
            _ => column,
        };

        Container::new(row.push(column))
            .style(theme)
            .padding(theme.padding.to_iced_padding())
            .width(theme.width)
            .height(theme.height)
            .align_x(theme.align_x)
            .align_y(theme.align_y)
    }

    fn get_display_name(&self) -> &str;
    fn get_icon(&self) -> Option<IconPath>;
    fn get_description(&self) -> Option<Cow<'_, str>>;
}
