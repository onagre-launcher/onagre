use crate::config::color::OnagreColor;
use crate::config::padding::OnagrePadding;
use iced::alignment::{Horizontal, Vertical};
use iced::{Background, Color, Length};
use iced_style::text_input;

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

impl Eq for SearchInputStyles {}

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

impl text_input::StyleSheet for &SearchInputStyles {
    fn active(&self) -> text_input::Style {
        text_input::Style {
            background: Background::Color(self.background.into()),
            border_radius: self.border_radius,
            border_width: self.border_width,
            border_color: self.border_color.into(),
        }
    }

    fn focused(&self) -> text_input::Style {
        text_input::Style {
            background: Background::Color(self.background.into()),
            border_radius: self.border_radius,
            border_width: self.border_width,
            border_color: self.border_color.into(),
        }
    }

    fn placeholder_color(&self) -> Color {
        self.placeholder_color.into()
    }

    fn value_color(&self) -> Color {
        self.value_color.into()
    }

    fn selection_color(&self) -> Color {
        self.selection_color.into()
    }

    fn hovered(&self) -> text_input::Style {
        text_input::Style {
            background: Background::Color(self.background.into()),
            border_radius: self.border_radius,
            border_width: self.border_width,
            border_color: self.border_color.into(),
        }
    }
}
