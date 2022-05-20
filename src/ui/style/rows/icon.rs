use iced::{Background, Length};
use iced::alignment::{Horizontal, Vertical};
use iced_style::container;
use iced_style::container::Style;
use crate::config::color::OnagreColor;
use crate::config::padding::OnagrePadding;

#[derive(Debug, PartialEq, Clone)]
pub struct IconStyle {
    // Style
    pub background: OnagreColor,
    pub border_radius: f32,
    pub border_width: f32,
    pub color: OnagreColor,
    pub border_color: OnagreColor,

    // Layout
    pub padding: OnagrePadding,
    pub width: Length,
    pub height: Length,
    pub align_x: Horizontal,
    pub align_y: Vertical,
    pub size: u16,
}

impl Eq for IconStyle {}

impl Default for IconStyle {
    fn default() -> Self {
        IconStyle {
            // Style
            background: OnagreColor::GREEN,
            border_radius: 1.0,
            border_width: 1.0,
            border_color: OnagreColor::RED,
            color: OnagreColor::BLUE,

            // Layout
            padding: OnagrePadding::from(0),
            width: Length::Shrink,
            height: Length::Shrink,
            align_x: Horizontal::Center,
            align_y: Vertical::Center,
            size: 24
        }
    }
}

impl container::StyleSheet for &IconStyle {
    fn style(&self) -> Style {
        Style {
            text_color: Some(self.color.into()),
            background: Some(Background::Color(self.background.into())),
            border_radius: self.border_radius,
            border_width: self.border_width,
            border_color: self.border_color.into()
        }
    }
}