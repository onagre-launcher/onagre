use crate::app::style::rows::RowStyles;
use crate::app::style::Scale;
use crate::config::color::OnagreColor;
use crate::config::padding::OnagrePadding;
use crate::THEME;
use iced::widget::container::Style;
use iced::{Length, Vector};

pub mod scroller;

pub fn row_container_style(_: &iced::Theme) -> Style {
    let theme = &THEME.app().rows;
    theme.into()
}

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
    pub width: iced::Length,
    pub height: Length,

    // Iced Scrollable
    pub row: RowStyles,
    pub row_selected: RowStyles,
}

impl Scale for RowContainerStyle {
    fn scale(mut self, scale: f32) -> Self {
        self.padding = self.padding.scale(scale);
        self.border_width = self.border_width.scale(scale);
        self.width = self.width.scale(scale);
        self.height = self.height.scale(scale);
        self.row = self.row.scale(scale);
        self.row_selected = self.row_selected.scale(scale);
        self
    }
}

impl Eq for RowContainerStyle {}

impl Into<Style> for &RowContainerStyle {
    fn into(self) -> Style {
        Style {
            text_color: Some(self.color.into()),
            background: Some(iced::Background::Color(self.background.into())),
            border: iced::Border {
                color: self.border_color.into(),
                width: self.border_width,
                radius: iced::border::Radius::from(self.border_radius),
            },
            shadow: iced::Shadow {
                color: iced::Color::TRANSPARENT,
                offset: Vector::ZERO,
                blur_radius: 0.,
            },
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
