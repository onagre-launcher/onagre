use crate::style::color::OnagreColor;
use crate::style::layout::Length;
use iced_style::{container, text_input, Background, Color};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct SearchContainerStyles {
    pub background: OnagreColor,
    pub text_color: OnagreColor,
    pub width: Length,
    pub height: Length,
    pub padding: u16,
    pub border_color: OnagreColor,
    pub border_radius: f32,
    pub border_width: f32,
    pub bar: SearchBarStyles,
}

impl Eq for SearchContainerStyles {}

impl Default for SearchContainerStyles {
    fn default() -> Self {
        Self {
            border_radius: 0.0,
            border_width: 0.0,
            text_color: OnagreColor::BLACK,
            border_color: OnagreColor::TRANSPARENT,
            background: OnagreColor::TRANSPARENT,
            height: Length::raw(40),
            bar: Default::default(),
            width: Length::fill(),
            padding: 4,
        }
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct SearchBarStyles {
    pub background: OnagreColor,
    pub value_color: OnagreColor,
    pub placeholder_color: OnagreColor,
    pub border_color: OnagreColor,
    pub selection_color: OnagreColor,
    pub border_radius: f32,
    pub border_width: f32,
    pub text_width: Length,
}

impl Eq for SearchBarStyles {}
impl Default for SearchBarStyles {
    fn default() -> Self {
        SearchBarStyles {
            border_radius: 0.0,
            border_width: 0.0,
            border_color: OnagreColor::TRANSPARENT,
            background: OnagreColor::TRANSPARENT,
            placeholder_color: OnagreColor::BLUE,
            value_color: OnagreColor::RED,
            selection_color: OnagreColor::BLACK,
            text_width: Length::fill(),
        }
    }
}

impl container::StyleSheet for &SearchContainerStyles {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(self.background.into())),
            border_radius: self.border_radius,
            border_width: self.border_radius,
            text_color: Some(self.text_color.into()),
            border_color: self.border_color.into(),
        }
    }
}

impl text_input::StyleSheet for &SearchBarStyles {
    fn active(&self) -> text_input::Style {
        text_input::Style {
            background: Background::Color(self.background.into()),
            border_radius: self.border_radius,
            border_width: self.border_width,
            border_color: self.border_color.into(),
        }
    }

    fn focused(&self) -> text_input::Style {
        text_input::Style {
            background: Background::Color(self.background.into()),
            border_radius: self.border_radius,
            border_width: self.border_width,
            border_color: self.border_color.into(),
        }
    }

    fn placeholder_color(&self) -> Color {
        self.placeholder_color.into()
    }

    fn value_color(&self) -> Color {
        self.value_color.into()
    }

    fn selection_color(&self) -> Color {
        self.selection_color.into()
    }

    fn hovered(&self) -> text_input::Style {
        text_input::Style {
            background: Background::Color(self.background.into()),
            border_radius: self.border_radius,
            border_width: self.border_width,
            border_color: self.border_color.into(),
        }
    }
}
