use iced::alignment::{Horizontal, Vertical};
use iced::{Background, Length};
use iced_style::container;
use iced_style::container::Style;
use crate::config::color::OnagreColor;
use crate::config::padding::OnagrePadding;

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
            width: Length::Fill,
            height: Length::Shrink,
            background: OnagreColor::GREEN,
            border_radius: 1.0,
            border_width: 1.0,
            color: OnagreColor::BLUE,
            padding: OnagrePadding::from(5),
            align_x: Horizontal::Left,
            border_color: OnagreColor::RED,
            align_y: Vertical::Center,
            font_size: 14
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
