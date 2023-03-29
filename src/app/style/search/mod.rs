use crate::app::style::rows::generic::GenericContainerStyle;
use crate::config::color::OnagreColor;
use crate::config::padding::OnagrePadding;
use iced::alignment::{Horizontal, Vertical};
use iced::Length;
use iced_core::Background;
use iced_style::container::{Appearance, StyleSheet};
use input::SearchInputStyles;

pub mod hint;
pub mod input;

#[derive(Debug, PartialEq)]
pub struct SearchContainerStyles {
    // Style
    pub background: OnagreColor,
    pub color: OnagreColor,
    pub border_color: OnagreColor,
    pub border_radius: f32,
    pub border_width: f32,

    // Layout
    pub padding: OnagrePadding,
    pub spacing: u16,
    pub width: Length,
    pub height: Length,
    pub align_x: Horizontal,
    pub align_y: Vertical,

    // Children
    pub input: SearchInputStyles,
    pub plugin_hint: Option<GenericContainerStyle>,
}

impl Eq for SearchContainerStyles {}

impl StyleSheet for &SearchContainerStyles {
    type Style = iced::Theme;

    fn appearance(&self, _: &Self::Style) -> Appearance {
        Appearance {
            text_color: Some(self.color.into()),
            background: Some(Background::Color(self.background.into())),
            border_radius: self.border_radius,
            border_width: self.border_width,
            border_color: self.border_color.into(),
        }
    }
}

impl Default for SearchContainerStyles {
    fn default() -> Self {
        Self {
            background: OnagreColor::WHITE,
            color: OnagreColor::DEFAULT_TEXT,
            border_radius: 4.0,
            border_width: 0.0,
            border_color: OnagreColor::TRANSPARENT,
            align_x: Horizontal::Left,
            align_y: Vertical::Center,
            height: Length::FillPortion(1),
            width: Length::Fill,
            padding: OnagrePadding::ZERO,
            input: Default::default(),
            plugin_hint: None,
            spacing: 2,
        }
    }
}
