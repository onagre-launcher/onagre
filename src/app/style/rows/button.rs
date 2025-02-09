use iced::{
    widget::button::{self, Style},
    Border, Color,
};

// Button is just used as a wrapper to get access to the click event.
// For now all theming option is disabled, we might want to make
// on hovered theming options available in the config later.
pub fn no_style() -> button::Style {
    Style {
        background: None,
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 0.0.into(),
        },
        text_color: Color::BLACK,
        shadow: Default::default(),
    }
}
