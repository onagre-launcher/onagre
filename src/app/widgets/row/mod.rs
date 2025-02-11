use iced::widget::{container, row, text, Button, Container};
use theme::{text_default, Class};

use crate::{
    app::{
        entries::entry2::Entry2,
        style::{self, rows::RowStyles},
        Message,
    },
    icons::IconPath,
    THEME,
};

use super::icon::Named;

mod theme;

pub struct LauncherEntriesContainer {
    pub category_icon: IconPath,
    pub selected: usize,
    pub children: Vec<LauncherEntry>,
}

pub struct LauncherEntry {
    pub index: usize,
    pub selected: bool,
    pub layout: &'static RowStyles,
    pub entry: Box<dyn Entry2>,
    pub show_icon: bool,
}

impl LauncherEntry {
    pub fn new(entry: Box<dyn Entry2>, index: usize, selected: bool) -> Self {
        LauncherEntry {
            entry,
            index,
            selected,
            layout: &THEME.app().rows.row,
            show_icon: false,
        }
    }

    fn title(&self) -> Container<Message, style::Theme> {
        container(iced::widget::row(vec![text(self.entry.get_display_name())
            .style(text_default)
            .size(self.layout.title.font_size)
            .into()]))
        .class(Class::Main)
        .padding(self.layout.title.padding.to_iced_padding())
        .width(self.layout.title.width)
        .height(self.layout.title.height)
        .align_x(self.layout.title.align_x)
        .align_y(self.layout.title.align_y)
    }

    fn description(&self) -> Option<Container<Message, crate::Theme>> {
        let class = Class::Description {
            selected: self.selected,
        };

        self.entry.get_description().map(|description| {
            container(row!(text(description)
                .style(text_default)
                .size(self.layout.description.font_size)))
            .class(class)
            .padding(self.layout.description.padding.to_iced_padding())
            .width(self.layout.description.width)
            .height(self.layout.description.height)
            .align_x(self.layout.description.align_x)
            .align_y(self.layout.description.align_y)
        })
    }

    pub fn to_container(&self, category_icon: Option<&str>) -> Container<Message, style::Theme> {
        let column = iced::widget::column(vec![self.title().into()]);
        let column = match self.description() {
            Some(description) if !self.layout.hide_description => column.push(description),
            _ => column,
        };

        let row = if let Some(category_icon) = category_icon {
            let named = Named::new(category_icon);
            let icon = named.icon();
            row![icon, column]
        } else {
            row![column]
        };

        let button = Button::new(row).on_press(Message::Click(self.index));

        Container::new(button)
            .class(Class::Row)
            .padding(self.layout.padding.to_iced_padding())
            .width(self.layout.width)
            .height(self.layout.height)
            .align_x(self.layout.align_x)
            .align_y(self.layout.align_y)
    }
}
