use crate::app::style::Scale;
use crate::config::color::OnagreColor;
use crate::config::padding::OnagrePadding;
use iced::alignment::{Horizontal, Vertical};
use iced::widget::container::Style;
use iced::{Length, Vector};

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

impl From<&GenericContainerStyle> for Style {
    fn from(val: &GenericContainerStyle) -> Self {
        Style {
            text_color: Some(val.color.into()),
            background: Some(iced::Background::Color(val.background.into())),
            border: iced::Border {
                color: val.border_color.into(),
                width: val.border_width,
                radius: iced::border::Radius::from(val.border_radius),
            },
            shadow: iced::Shadow {
                color: iced::Color::TRANSPARENT,
                offset: Vector::ZERO,
                blur_radius: 0.,
            },
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
