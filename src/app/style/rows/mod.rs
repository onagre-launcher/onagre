use crate::config::color::OnagreColor;
use crate::config::padding::OnagrePadding;
use generic::GenericContainerStyle;
use iced::alignment::{Horizontal, Vertical};
use iced::{Background, Length};
use iced_style::container;
use icon::IconStyle;
pub mod generic;
pub mod icon;

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
    pub hide_description: bool,
    pub hide_category_icon: bool,
    // Children
    pub title: GenericContainerStyle,
    pub description: GenericContainerStyle,
    pub icon: IconStyle,
    pub category_icon: IconStyle,
}

impl Default for RowStyles {
    fn default() -> Self {
        RowStyles {
            width: Length::Fill,
            height: Length::Shrink,
            background: OnagreColor::DEFAULT_BACKGROUND,
            color: OnagreColor::WHITE,
            border_radius: 0.0,
            border_width: 0.0,
            padding: OnagrePadding::from(5),
            align_x: Horizontal::Right,
            align_y: Vertical::Bottom,
            border_color: OnagreColor::RED,
            hide_description: false,
            hide_category_icon: false,
            title: GenericContainerStyle::default(),
            description: GenericContainerStyle::description_default(),
            icon: Default::default(),
            category_icon: IconStyle::category_default(),
        }
    }
}

impl RowStyles {
    pub fn default_selected() -> Self {
        Self {
            color: OnagreColor::WHITE,
            title: GenericContainerStyle {
                color: OnagreColor::WHITE,
                ..Default::default()
            },
            description: GenericContainerStyle {
                color: OnagreColor::WHITE,
                ..GenericContainerStyle::description_default()
            },
            ..Default::default()
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
