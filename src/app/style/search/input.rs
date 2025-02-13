use crate::app::style::Scale;
use crate::config::color::OnagreColor;
use crate::config::padding::OnagrePadding;
use iced::alignment::{Horizontal, Vertical};
use iced::border::Radius;
use iced::widget::{container, text_input};
use iced::{Background, Border, Vector};
use iced::{Length, Shadow};

#[derive(Debug, PartialEq)]
pub struct SearchInputStyles {
    // Style
    pub background: OnagreColor,
    pub border_radius: f32,
    pub border_width: f32,
    pub border_color: OnagreColor,
    pub placeholder_color: OnagreColor,
    pub value_color: OnagreColor,
    pub selection_color: OnagreColor,
    pub text_width: Length,

    // Layout
    pub font_size: u16,
    pub width: Length,
    pub height: Length,
    pub align_x: Horizontal,
    pub align_y: Vertical,
    pub padding: OnagrePadding,
}

impl Scale for SearchInputStyles {
    fn scale(mut self, scale: f32) -> Self {
        self.height = self.height.scale(scale);
        self.width = self.width.scale(scale);
        self.padding = self.padding.scale(scale);
        self.padding = self.padding.scale(scale);
        self.padding = self.padding.scale(scale);
        self
    }
}

impl From<&SearchInputStyles> for text_input::Style {
    fn from(val: &SearchInputStyles) -> Self {
        text_input::Style {
            background: Background::Color(val.background.into()),
            border: Border {
                color: val.border_color.into(),
                width: val.border_width,
                radius: Radius::from(val.border_radius),
            },
            icon: Default::default(),
            placeholder: val.placeholder_color.into(),
            value: val.value_color.into(),
            selection: val.selection_color.into(),
        }
    }
}

impl From<&SearchInputStyles> for container::Style {
    fn from(val: &SearchInputStyles) -> Self {
        container::Style {
            text_color: Some(val.placeholder_color.into()),
            background: Some(Background::Color(val.background.into())),
            border: Border {
                color: val.border_color.into(),
                width: val.border_width,
                radius: Radius::from(val.border_radius),
            },
            shadow: Shadow {
                color: iced::Color::TRANSPARENT,
                offset: Vector::ZERO,
                blur_radius: 0.,
            },
        }
    }
}

impl Default for SearchInputStyles {
    fn default() -> Self {
        SearchInputStyles {
            border_radius: 0.0,
            border_width: 0.0,
            border_color: OnagreColor::TRANSPARENT,
            background: OnagreColor::WHITE,
            placeholder_color: OnagreColor::DEFAULT_TEXT,
            value_color: OnagreColor::DEFAULT_TEXT,
            selection_color: OnagreColor::DEFAULT_BORDER,
            text_width: Length::Fill,
            font_size: 14,
            width: Length::Fill,
            height: Length::Fill,
            align_x: Horizontal::Left,
            align_y: Vertical::Center,
            padding: OnagrePadding {
                top: 0,
                right: 5,
                bottom: 0,
                left: 5,
            },
        }
    }
}
