use crate::app::style::Scale;
use crate::config::color::OnagreColor;
use crate::config::padding::OnagrePadding;
use iced::alignment::{Horizontal, Vertical};
use iced::border::Radius;
use iced::widget::text_input;
use iced::Length;
use iced::{Background, Border};

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

impl Into<text_input::Style> for SearchInputStyles {
    fn into(self) -> text_input::Style {
        text_input::Style {
            background: Background::Color(self.background.into()),
            border: Border {
                color: self.border_color.into(),
                width: self.border_width,
                radius: Radius::from(self.border_radius),
            },
            icon: Default::default(),
            placeholder: self.placeholder_color.into(),
            value: self.value_color.into(),
            selection: self.selection_color.into(),
        }
    }
}

/*     fn disabled(&self, _style: &Self::Style) -> Appearance {
        Appearance {
            background: Background::Color(self.background.into()),
            border: Border {
                color: self.border_color.into(),
                width: self.border_width,
                radius: Radius::from(self.border_radius),
            },
            icon_color: Default::default(),
        }
    }
*/

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
