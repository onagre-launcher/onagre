use iced_style::{container};
use iced::Background;
use crate::config::color::OnagreColor;
use crate::config::padding::OnagrePadding;
use crate::ui::style::rows::RowStyles;

pub mod scroller;

#[derive(Debug, PartialEq)]
pub struct RowContainerStyle {
    // Iced Container
    pub color: OnagreColor,
    pub background: OnagreColor,
    pub border_color: OnagreColor,
    pub border_radius: f32,
    pub border_width: f32,

    // Layout
    pub padding: OnagrePadding,

    // Iced Scrollable
    pub row: RowStyles,
    pub row_selected: RowStyles,
}

impl Eq for RowContainerStyle {}

impl Default for RowContainerStyle {
    fn default() -> Self {
        Self {
            color: OnagreColor::RED,
            background: OnagreColor::YELLOW,
            border_color: OnagreColor::RED,
            border_radius: 0.0,
            border_width: 1.0,
            padding: OnagrePadding::from(0),
            row: Default::default(),
            row_selected: Default::default(),
        }
    }
}

impl container::StyleSheet for &RowContainerStyle {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(self.background.into())),
            text_color: Some(self.color.into()),
            border_radius: self.border_radius,
            border_width: self.border_radius,
            border_color: self.border_color.into(),
        }
    }
}