use crate::config::color::OnagreColor;
use crate::config::padding::OnagrePadding;
use crate::ui::style::rows::RowStyles;
use iced::Background;
use iced_style::container;

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
            color: OnagreColor::DEFAULT_TEXT,
            background: OnagreColor::DEFAULT_BACKGROUND,
            border_color: OnagreColor::RED,
            border_radius: 0.0,
            border_width: 0.0,
            padding: OnagrePadding {
                top: 5,
                right: 5,
                bottom: 5,
                left: 0,
            },
            row: RowStyles::default(),
            row_selected: RowStyles::default_selected(),
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
