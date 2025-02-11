
use iced::widget::{column, container, row, scrollable, text, Button, Container, Scrollable};
use theme::{text_default, Class};

use crate::app::{
        entries::entry2::Entry2,
        style::{self, rows::RowStyles},
        Message,
    };

use super::icon::Named;

pub mod theme;

pub fn to_scrollable<'a>(
    category_icon: Option<&'a str>,
    layout: &'static RowStyles,
    entries: &'a [Box<dyn Entry2>],
    selected: usize,
) -> Scrollable<'a, Message, crate::Theme> {
    let selected = |idx| idx == selected;
    let children: Vec<Container<Message, crate::Theme>> = entries
        .iter()
        .enumerate()
        .map(|(idx, entry)| to_container(layout, idx, selected(idx), entry, category_icon))
        .collect();

    scrollable(column(children.into_iter().map(Into::into)))
}

fn title<'a>(
    layout: &'static RowStyles,
    entry: &'a Box<dyn Entry2>,
) -> Container<'a, Message, style::Theme> {
    container(iced::widget::row(vec![text(
        entry.get_display_name().to_string(),
    )
    .style(text_default)
    .size(layout.title.font_size)
    .into()]))
    .class(Class::Main)
    .padding(layout.title.padding.to_iced_padding())
    .width(layout.title.width)
    .height(layout.title.height)
    .align_x(layout.title.align_x)
    .align_y(layout.title.align_y)
}
fn description<'a>(
    layout: &'static RowStyles,
    selected: bool,
    entry: &'a Box<dyn Entry2>,
) -> Option<Container<'a, Message, crate::Theme>> {
    let class = Class::Description { selected };

    entry.get_description().map(|description| {
        container(row!(text(description).style(text_default)))
            .class(class)
            .padding(layout.description.padding.to_iced_padding())
            .width(layout.description.width)
            .height(layout.description.height)
            .align_x(layout.description.align_x)
            .align_y(layout.description.align_y)
    })
}

pub fn to_container<'a>(
    layout: &'static RowStyles,
    index: usize,
    selected: bool,
    entry: &'a Box<dyn Entry2>,
    category_icon: Option<&'a str>,
) -> Container<'a, Message, style::Theme> {
    let column = iced::widget::column(vec![title(layout, entry).into()]);
    let column = match description(layout, selected, entry) {
        Some(description) if !layout.hide_description => column.push(description),
        _ => column,
    };

    let row = if let Some(category_icon) = category_icon {
        let named = Named::new(category_icon);
        let icon = named.icon();
        row![icon, column]
    } else {
        row![column]
    };

    let button = Button::new(row).on_press(Message::Click(index));

    Container::new(button)
        .class(Class::Row)
        .padding(layout.padding.to_iced_padding())
        .width(layout.width)
        .height(layout.height)
        .align_x(layout.align_x)
        .align_y(layout.align_y)
}
