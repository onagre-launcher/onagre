use crate::config::color::OnagreColor;
use crate::config::padding::OnagrePadding;
use crate::ui::style::rows::generic::GenericContainerStyle;
use iced::alignment::{Horizontal, Vertical};
use iced::{Background, Length};
use iced_style::container;
use iced_style::container::Style;
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
    pub width: Length,
    pub height: Length,
    pub align_x: Horizontal,
    pub align_y: Vertical,

    // Children
    pub input: SearchInputStyles,
    pub plugin_hint: Option<GenericContainerStyle>,
}

impl Eq for SearchContainerStyles {}

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
