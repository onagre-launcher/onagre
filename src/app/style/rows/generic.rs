use crate::app::style::Scale;
use crate::config::color::OnagreColor;
use crate::config::padding::OnagrePadding;
use iced::alignment::{Horizontal, Vertical};
use iced::Length;
use iced_core::{Background, BorderRadius};
use iced_style::container::{Appearance, StyleSheet};

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

impl Scale for GenericContainerStyle {
    fn scale(mut self, scale: f32) -> Self {
        self.height = self.height.scale(scale);
        self.width = self.width.scale(scale);
        self.font_size = self.font_size.scale(scale);
        self.border_width = self.border_width.scale(scale);
        self.padding = self.padding.scale(scale);
        self
    }
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

impl StyleSheet for &GenericContainerStyle {
    type Style = iced::Theme;

    fn appearance(&self, _: &Self::Style) -> Appearance {
        Appearance {
            text_color: Some(self.color.into()),
            background: Some(Background::Color(self.background.into())),
            border_radius: BorderRadius::from(self.border_radius),
            border_width: self.border_width,
            border_color: self.border_color.into(),
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
