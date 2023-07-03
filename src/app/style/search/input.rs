use crate::config::color::OnagreColor;
use crate::config::padding::OnagrePadding;
use iced::alignment::{Horizontal, Vertical};
use iced::Length;
use iced_core::{Background, Color};
use iced_style::text_input::{Appearance, StyleSheet};

#[derive(Debug, PartialEq)]
pub struct SearchInputStyles {
    // Style
    pub background: OnagreColor,
    pub border_radius: f32,
    pub border_width: f32,
    pub border_color: OnagreColor,
    pub placeholder_color: OnagreColor,
    pub value_color: OnagreColor,
    pub selection_color: OnagreColor,
    pub text_width: Length,

    // Layout
    pub font_size: u16,
    pub width: Length,
    pub height: Length,
    pub align_x: Horizontal,
    pub align_y: Vertical,
    pub padding: OnagrePadding,
}

impl Eq for SearchInputStyles {}

impl StyleSheet for &SearchInputStyles {
    type Style = iced::Theme;

    fn active(&self, _: &Self::Style) -> Appearance {
        Appearance {
            background: Background::Color(self.background.into()),
            border_radius: self.border_radius,
            border_width: self.border_width,
            border_color: self.border_color.into(),
            icon_color: Default::default(),
        }
    }

    fn focused(&self, style: &Self::Style) -> Appearance {
        self.active(style)
    }

    fn placeholder_color(&self, _: &Self::Style) -> Color {
        self.placeholder_color.into()
    }

    fn value_color(&self, _: &Self::Style) -> Color {
        self.value_color.into()
    }

    fn disabled_color(&self, _style: &Self::Style) -> Color {
        Color::TRANSPARENT
    }

    fn selection_color(&self, _: &Self::Style) -> Color {
        self.selection_color.into()
    }

    fn disabled(&self, _style: &Self::Style) -> Appearance {
        Appearance {
            background: Background::Color(self.background.into()),
            border_radius: self.border_radius,
            border_width: self.border_width,
            border_color: self.border_color.into(),
            icon_color: Default::default(),
        }
    }
}

impl Default for SearchInputStyles {
    fn default() -> Self {
        SearchInputStyles {
            border_radius: 0.0,
            border_width: 0.0,
            border_color: OnagreColor::TRANSPARENT,
            background: OnagreColor::WHITE,
            placeholder_color: OnagreColor::DEFAULT_TEXT,
            value_color: OnagreColor::DEFAULT_TEXT,
            selection_color: OnagreColor::DEFAULT_BORDER,
            text_width: Length::Fill,
            font_size: 14,
            width: Length::Fill,
            height: Length::Fill,
            align_x: Horizontal::Left,
            align_y: Vertical::Center,
            padding: OnagrePadding {
                top: 0,
                right: 5,
                bottom: 0,
                left: 5,
            },
        }
    }
}
