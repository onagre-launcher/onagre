use crate::app::style::rows::button::ButtonStyle;
use crate::app::style::rows::icon::IconStyle;
use crate::app::style::rows::RowStyles;
use crate::app::Message;
use crate::icons::{fallback_icon, Extension, IconPath};
use crate::THEME;
use iced::widget::{column, container, row, text, Button, Container, Image, Row};
use iced::{Alignment, Length};
use std::borrow::Cow;

pub(crate) mod db_entry;
pub(crate) mod pop_entry;

pub(crate) trait AsEntry<'a> {
    fn to_row<'b>(
        &'a self,
        selected: Option<usize>,
        idx: usize,
        category_icon: Option<&'a IconPath>,
    ) -> Container<'b, Message>
    where
        'b: 'a,
    {
        let theme = self.get_style(selected, idx);

        let row = if THEME.icon_theme.is_some() {
            self.get_icon_layout(category_icon, theme)
        } else {
            Row::new()
        };

        let row = row
            .height(Length::Shrink)
            .width(Length::Fill)
            .spacing(theme.spacing)
            // See : https://github.com/iced-rs/iced/pull/1044
            .align_items(Alignment::Start);

        self.as_row(row, theme, idx)
    }

    fn as_row<'b>(
        &self,
        row: Row<'b, Message>,
        theme: &'static RowStyles,
        idx: usize,
    ) -> Container<'b, Message>
    where
        'b: 'a,
    {
        let title_row: Container<'_, Message> =
            container(iced::widget::row(vec![text(self.get_display_name())
                .size(theme.title.font_size)
                .into()]))
            .style(iced::theme::Container::Custom(Box::new(&theme.title)))
            .padding(theme.title.padding.to_iced_padding())
            .width(theme.title.width)
            .height(theme.title.height)
            .align_x(theme.title.align_x)
            .align_y(theme.title.align_y);

        let description_row: Option<Container<'_, Message>> =
            self.get_description().map(|description| {
                container(row!(
                    text(description.as_ref()).size(theme.description.font_size)
                ))
                .style(iced::theme::Container::Custom(Box::new(&theme.description)))
                .padding(theme.description.padding.to_iced_padding())
                .width(theme.description.width)
                .height(theme.description.height)
                .align_x(theme.description.align_x)
                .align_y(theme.description.align_y)
            });

        let column = column(vec![title_row.into()]);

        let column = match description_row {
            Some(description) if !theme.hide_description => column.push(description),
            _ => column,
        };

        let button = Button::new(row.push(column))
            .style(iced::theme::Button::Custom(Box::new(&ButtonStyle)))
            .on_press(Message::Click(idx));

        Container::new(button)
            .style(iced::theme::Container::Custom(Box::new(theme)))
            .padding(theme.padding.to_iced_padding())
            .width(theme.width)
            .height(theme.height)
            .align_x(theme.align_x)
            .align_y(theme.align_y)
    }

    fn get_style(&self, selected: Option<usize>, idx: usize) -> &'static RowStyles {
        let selected = selected.map(|selected| selected == idx).unwrap_or(false);
        if selected {
            &THEME.app_container.rows.row_selected
        } else {
            &THEME.app_container.rows.row
        }
    }

    fn get_icon_layout<'b>(
        &'a self,
        category_icon: Option<&'a IconPath>,
        style: &'static RowStyles,
    ) -> Row<'b, Message>
    where
        'b: 'a,
    {
        let icon = self.get_icon();
        let icon = Self::build_icon(&style.icon, icon);
        let row = if !style.hide_category_icon {
            let category_icon = Self::build_icon(&style.category_icon, category_icon);
            Row::new().push(category_icon)
        } else {
            Row::new()
        };

        row.push(icon)
    }

    fn build_icon<'b, I: AsRef<IconPath>>(
        theme: &'static IconStyle,
        icon: Option<I>,
    ) -> Container<'b, Message>
    where
        'b: 'a,
    {
        let icon = match icon.as_ref() {
            Some(icon) => match icon.as_ref().extension {
                Extension::Svg => Container::new(
                    icon.as_ref()
                        .to_svg(&theme.color)
                        .height(Length::Fixed(theme.icon_size as f32))
                        .width(Length::Fixed(theme.icon_size as f32)),
                ),
                Extension::Png => Container::new(
                    Image::new(&icon.as_ref().path)
                        .height(Length::Fixed(theme.icon_size as f32))
                        .width(Length::Fixed(theme.icon_size as f32)),
                ),
            },
            None => Container::new(
                fallback_icon(&theme.color)
                    .height(Length::Fixed(theme.icon_size as f32))
                    .width(Length::Fixed(theme.icon_size as f32)),
            ),
        };

        icon
            //.style(theme)
            .align_y(theme.align_y)
            .align_x(theme.align_x)
            .width(theme.width)
            .height(theme.height)
            .padding(theme.padding.to_iced_padding())
    }

    fn get_display_name(&self) -> &str;

    fn get_icon(&self) -> Option<IconPath> {
        unreachable!()
    }
    fn get_description(&self) -> Option<Cow<'_, str>>;
}
