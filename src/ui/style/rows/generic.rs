use crate::config::color::OnagreColor;
use crate::config::padding::OnagrePadding;
use iced::alignment::{Horizontal, Vertical};
use iced::{Background, Length};
use iced_style::container;
use iced_style::container::Style;

#[derive(Debug, PartialEq, Clone)]
pub struct GenericContainerStyle {
    // Style
    pub background: OnagreColor,
    pub border_radius: f32,
    pub border_width: f32,
    pub color: OnagreColor,
    pub border_color: OnagreColor,

    // Layout
    pub font_size: u16,
    pub padding: OnagrePadding,
    pub align_x: Horizontal,
    pub align_y: Vertical,
    pub width: Length,
    pub height: Length,
}

impl Default for GenericContainerStyle {
    fn default() -> Self {
        GenericContainerStyle {
            background: OnagreColor::DEFAULT_BACKGROUND,
            color: OnagreColor::DEFAULT_TEXT,
            width: Length::Fill,
            height: Length::Shrink,
            border_color: OnagreColor::RED,
            border_radius: 0.0,
            border_width: 0.0,
            padding: OnagrePadding::ZERO,
            align_x: Horizontal::Left,
            align_y: Vertical::Center,
            font_size: 14,
        }
    }
}

impl GenericContainerStyle {
    pub fn description_default() -> Self {
        Self {
            font_size: 11,
            ..Default::default()
        }
    }
}

impl Eq for GenericContainerStyle {}

impl container::StyleSheet for &GenericContainerStyle {
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
