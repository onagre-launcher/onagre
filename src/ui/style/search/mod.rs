use iced::{Background, Length};
use iced::alignment::{Horizontal, Vertical};
use iced_style::container;
use iced_style::container::Style;
use crate::config::color::OnagreColor;
use crate::config::padding::OnagrePadding;
use crate::ui::style::rows::generic::GenericContainerStyle;
use input::SearchInputStyles;

pub mod input;
pub mod hint;

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
    pub width: Length,
    pub height: Length,
    pub align_x: Horizontal,
    pub align_y: Vertical,

    // Children
    pub input: SearchInputStyles,
    pub plugin_hint: GenericContainerStyle,
}

impl Eq for SearchContainerStyles {}

impl Default for SearchContainerStyles {
    fn default() -> Self {
        Self {
            border_radius: 0.0,
            border_width: 0.0,
            color: OnagreColor::BLACK,
            border_color: OnagreColor::TRANSPARENT,
            background: OnagreColor::TRANSPARENT,
            height: Length::Units(40),
            align_x: Horizontal::Left,
            align_y: Vertical::Center,
            input: Default::default(),
            width: Length::Fill,
            padding: OnagrePadding::from(0),
            plugin_hint: Default::default()
        }
    }
}

impl container::StyleSheet for &SearchContainerStyles {
    fn style(&self) -> Style {
        Style {
            background: Some(Background::Color(self.background.into())),
            border_radius: self.border_radius,
            border_width: self.border_width,
            text_color: Some(self.color.into()),
            border_color: self.border_color.into(),
        }
    }
}
