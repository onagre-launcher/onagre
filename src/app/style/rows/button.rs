use iced_core::{Color, Vector};
use iced_style::button::{Appearance, StyleSheet};

// Button is just used as a wrapper to get access to the click event.
// For now all theming option is disabled, we might want to make
// on hovered theming options available in the config later.
pub struct ButtonStyle;

impl StyleSheet for &ButtonStyle {
    type Style = iced::Theme;

    fn active(&self, _: &Self::Style) -> Appearance {
        no_style()
    }

    fn hovered(&self, _: &Self::Style) -> Appearance {
        no_style()
    }

    fn pressed(&self, _: &Self::Style) -> Appearance {
        no_style()
    }

    fn disabled(&self, _: &Self::Style) -> Appearance {
        no_style()
    }
}

fn no_style() -> Appearance {
    Appearance {
        shadow_offset: Vector { x: 0.0, y: 0.0 },
        background: None,
        border_radius: 0.0.into(),
        border_width: 0.0,
        border_color: Color::TRANSPARENT,
        text_color: Color::BLACK,
    }
}
