use crate::app::style::rows::RowStyles;
use crate::config::color::OnagreColor;
use crate::config::padding::OnagrePadding;
use iced::Length;
use iced_core::Background;
use iced_style::container::{Appearance, StyleSheet};

pub mod scroller;

#[derive(Debug, PartialEq, Clone)]
pub struct RowContainerStyle {
    // Iced Container
    pub color: OnagreColor,
    pub background: OnagreColor,
    pub border_color: OnagreColor,
    pub border_radius: f32,
    pub border_width: f32,

    // Layout
    pub padding: OnagrePadding,
    pub width: Length,
    pub height: Length,

    // Iced Scrollable
    pub row: RowStyles,
    pub row_selected: RowStyles,
}

impl Eq for RowContainerStyle {}

impl StyleSheet for &RowContainerStyle {
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

impl Default for RowContainerStyle {
    fn default() -> Self {
        Self {
            color: OnagreColor::DEFAULT_TEXT,
            background: OnagreColor::DEFAULT_BACKGROUND,
            border_color: OnagreColor::RED,
            border_radius: 0.0,
            border_width: 0.0,
            padding: OnagrePadding {
                top: 10,
                right: 0,
                bottom: 0,
                left: 0,
            },
            width: Length::Fill,
            height: Length::FillPortion(8),
            row: RowStyles::default(),
            row_selected: RowStyles::default_selected(),
        }
    }
}
