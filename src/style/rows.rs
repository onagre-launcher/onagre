use iced_style::{container, Background};
use crate::style::theme_settings::{RowStyles, RowStylesSelected};
use crate::style::OnagreColor;

impl container::StyleSheet for &RowStyles {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(self.background.into())),
            border_radius: self.border_radius,
            border_width: self.border_radius,
            text_color: Some(self.text_color.into()),
            border_color: self.border_color.into(),
        }
    }
}

impl container::StyleSheet for &RowStylesSelected {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(self.background.into())),
            border_radius: self.border_radius,
            border_width: self.border_radius,
            text_color: Some(self.text_color.into()),
            border_color: self.border_color.into(),
        }
    }
}

impl Default for RowStyles {
    fn default() -> Self {
        Self {
            selected: RowStylesSelected {
                background: OnagreColor::WHITE,
                border_radius: 0,
                border_width: 0,
                text_color: OnagreColor::BLACK,
                border_color: OnagreColor::BLACK
            },
            background: OnagreColor::BLACK,
            border_radius: 0,
            border_width: 0,
            text_color: OnagreColor::WHITE,
            border_color: OnagreColor::BLACK
        }
    }
}
