use crate::app::style::rows::RowStyles;
use crate::app::Message;
use crate::icons::{fallback_icon, Extension, IconPath};
use crate::THEME;
use iced::{Container, Image, Length, Row, Svg, Text};
use iced_native::widget::Column;
use iced_native::Alignment;
use std::borrow::Cow;

pub(crate) mod db_entry;
pub(crate) mod pop_entry;

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
