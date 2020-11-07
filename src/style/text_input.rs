use crate::style::theme_settings::{TextInputStyles};
use crate::style::OnagreColor;
use iced_style::{text_input, Background, Color};

impl Default for TextInputStyles {
    fn default() -> Self {
        Self {
            border_radius: 0,
            border_width: 0,
            border_color: OnagreColor::BLACK,
            background: OnagreColor::BLACK,
            placeholder_color: OnagreColor::WHITE,
            value_color: OnagreColor::WHITE,
            selection_color: OnagreColor::BLACK
        }
    }
}

impl text_input::StyleSheet for &TextInputStyles {
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
