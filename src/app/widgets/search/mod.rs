use iced::{
    alignment::{Horizontal, Vertical},
    widget::{container, text_input, text_input::Id, Container, Row, Text},
    Length,
};

use crate::app::{style::search::SearchContainerStyles, Message, OnagreTheme};

use super::entries::theme::Class;

pub fn search_bar<'a>(
    input_id: Id,
    input_display: &'a str,
    modifier_display: Option<&'a str>,
    layout: &'a SearchContainerStyles,
) -> Container<'a, Message, OnagreTheme> {
    let search_input_layout = &layout.input;

    let text_input = text_input("Search", input_display)
        .on_input(Message::InputChanged)
        .id(input_id)
        .class(Class::SearchInput)
        .padding(search_input_layout.padding.to_iced_padding())
        .width(search_input_layout.text_width)
        .size(search_input_layout.font_size);

    let search_input = container(text_input)
        .width(search_input_layout.width)
        .height(search_input_layout.height)
        .align_x(search_input_layout.align_x)
        .align_y(search_input_layout.align_y)
        .class(Class::SearchInput);

    let search_bar = Row::new().width(Length::Fill).height(Length::Fill);
    // Either plugin_hint is enabled and we try to display it
    // Or we display the normal search input
    let search_bar = match &layout.plugin_hint {
        None => search_bar.push(search_input),
        Some(plugin_hint_style) => if modifier_display.is_some() {
            let plugin_hint = container(
                Text::new(modifier_display.unwrap())
                    .align_y(Vertical::Center)
                    .align_x(Horizontal::Center)
                    .size(plugin_hint_style.font_size),
            )
            .class(Class::PluginHint)
            .width(plugin_hint_style.width)
            .height(plugin_hint_style.height)
            .align_y(plugin_hint_style.align_y)
            .align_x(plugin_hint_style.align_x)
            .padding(plugin_hint_style.padding.to_iced_padding());

            search_bar.push(plugin_hint).push(search_input)
        } else {
            search_bar.push(search_input)
        }
        .spacing(layout.spacing),
    };

    container(search_bar)
        .class(Class::SearchBar)
        .align_x(layout.align_x)
        .align_y(layout.align_y)
        .padding(layout.padding.to_iced_padding())
        .width(layout.width)
        .height(layout.height)
}
