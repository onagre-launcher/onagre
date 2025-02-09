use iced::widget::{button, column, container, row, scrollable, text, Container, Scrollable};
use theme::{text_default, Class};

use crate::app::{entries::Entry, style::rows::RowStyles, Message, OnagreTheme};

use super::icon::{IconFallback, Named};

pub mod theme;

pub fn to_scrollable<'a>(
    layout: &'a RowStyles,
    entries: &'a [Box<dyn Entry>],
    selected: usize,
    icon_theme: Option<&'a str>,
) -> Scrollable<'a, Message, OnagreTheme> {
    let selected = |idx| idx == selected;
    let children: Vec<Container<Message, OnagreTheme>> = entries
        .iter()
        .enumerate()
        .map(|(idx, entry)| to_container(layout, idx, selected(idx), entry.as_ref(), icon_theme))
        .collect();

    scrollable(column(children.into_iter().map(Into::into))).class(Class::RowClickable)
}

fn title<'a>(
    layout: &'a RowStyles,
    entry: &'a dyn Entry,
    selected: bool,
) -> Container<'a, Message, OnagreTheme> {
    container(iced::widget::row(vec![text(
        entry.get_display_name().to_string(),
    )
    .style(text_default)
    .size(layout.title.font_size)
    .into()]))
    .class(Class::Title { selected })
    .padding(layout.title.padding.to_iced_padding())
    .width(layout.title.width)
    .height(layout.title.height)
    .align_x(layout.title.align_x)
    .align_y(layout.title.align_y)
}

fn description<'a>(
    layout: &'a RowStyles,
    selected: bool,
    entry: &'a dyn Entry,
) -> Option<Container<'a, Message, OnagreTheme>> {
    entry.get_description().map(|description| {
        container(row!(text(description).style(text_default)))
            .class(Class::Description { selected })
            .padding(layout.description.padding.to_iced_padding())
            .width(layout.description.width)
            .height(layout.description.height)
            .align_x(layout.description.align_x)
            .align_y(layout.description.align_y)
    })
}

pub fn to_container<'a>(
    layout: &'a RowStyles,
    index: usize,
    selected: bool,
    entry: &'a dyn Entry,
    icon_theme: Option<&str>,
) -> Container<'a, Message, OnagreTheme> {
    let column = iced::widget::column(vec![title(layout, entry, selected).into()]);
    let column = match description(layout, selected, entry) {
        Some(description) if !layout.hide_description => column.push(description),
        _ => column,
    };

    let icon = entry
        .get_icon()
        .map(|source| Named::from_icon_source(source, icon_theme).fallback(IconFallback::Default))
        .map(|i| Named::icon(i, selected).size(layout.icon.icon_size))
        .map(|icon| {
            container(icon)
                .class(Class::Icon { selected })
                .align_y(layout.icon.align_y)
                .align_x(layout.icon.align_x)
                .height(layout.icon.height)
                .width(layout.icon.width)
                .padding(layout.icon.padding.to_iced_padding())
        });
    let category_icon = entry
        .get_category_icon()
        .map(|source| Named::from_icon_source(source, icon_theme).fallback(IconFallback::Default))
        .map(|i| Named::icon(i, selected).size(layout.category_icon.icon_size))
        .map(|icon| {
            container(icon)
                .class(Class::CategoryIcon { selected })
                .align_y(layout.category_icon.align_y)
                .align_x(layout.category_icon.align_x)
                .height(layout.category_icon.height)
                .width(layout.category_icon.width)
                .padding(layout.category_icon.padding.to_iced_padding())
        });

    let row = match (category_icon, icon) {
        (Some(category_icon), Some(icon)) => row![category_icon, icon, column],
        (None, Some(icon)) => row![icon, column],
        (Some(category_icon), None) => row![category_icon, column],
        _ => row![column],
    };

    let button = button(row).on_press(Message::Click(index));

    container(button)
        .class(Class::Row { selected })
        .padding(layout.padding.to_iced_padding())
        .width(layout.width)
        .height(layout.height)
        .align_x(layout.align_x)
        .align_y(layout.align_y)
}
