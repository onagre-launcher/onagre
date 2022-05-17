use iced::{Background, Length};
use iced::alignment::{Horizontal, Vertical};
use iced_style::container;
use generic::GenericContainerStyle;
use crate::config::color::OnagreColor;
use crate::config::padding::OnagrePadding;
use icon::IconStyle;

pub mod icon;
pub mod generic;

#[derive(Debug, PartialEq, Clone)]
pub struct RowStyles {
    // Layout
    pub padding: OnagrePadding,
    pub width: Length,
    pub height: Length,
    pub align_x: Horizontal,
    pub align_y: Vertical,


    // Style
    pub background: OnagreColor,
    pub border_radius: f32,
    pub border_width: f32,
    pub color: OnagreColor,
    pub border_color: OnagreColor,

    // Children
    pub title: GenericContainerStyle,
    pub description: GenericContainerStyle,
    pub icon: IconStyle,
}

impl Default for RowStyles {
    fn default() -> Self {
        RowStyles {
            width: Length::Fill,
            height: Length::Shrink,
            background: OnagreColor::GREEN,
            border_radius: 1.0,
            border_width: 1.0,
            color: OnagreColor::BLUE,
            padding: OnagrePadding::from(5),
            align_x: Horizontal::Right,
            align_y: Vertical::Bottom,
            border_color: OnagreColor::RED,
            title: GenericContainerStyle::default(),
            description: GenericContainerStyle::default(),
            icon: Default::default()
        }
    }
}

impl Eq for RowStyles {}

impl container::StyleSheet for &RowStyles {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(self.background.into())),
            border_radius: self.border_radius,
            border_width: self.border_width,
            text_color: Some(self.color.into()),
            border_color: self.border_color.into(),
        }
    }
}
