use iced::{container, rule, Color};

use crate::style::theme_settings::Theme;
use iced_native::Background;

impl Theme {
    pub fn load() -> Self {
        if let Ok(theme) = Theme::get() {
            theme
        } else {
            Default::default()
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            background: Default::default(),
            foreground: Default::default(),
            border_color: Default::default(),
            border_radius: 0,
            border_width: 0,
            rows: Default::default(),
            scrollable: Default::default(),
            search: Default::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TransparentContainer;

impl container::StyleSheet for TransparentContainer {
    fn style(&self) -> container::Style {
        container::Style {
            background: Color::TRANSPARENT.into(),
            border_radius: 0,
            border_width: 0,
            text_color: Color::TRANSPARENT.into(),
            border_color: Color::TRANSPARENT,
        }
    }
}

impl container::StyleSheet for &Theme {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(self.background.into())),
            border_radius: self.border_radius,
            border_width: self.border_width,
            text_color: Some(self.foreground.into()),
            border_color: self.border_color.into(),
        }
    }
}

impl AsRef<Theme> for Theme {
    fn as_ref(&self) -> &Theme {
        &self
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rule;

impl rule::StyleSheet for Rule {
    fn style(&self) -> rule::Style {
        rule::Style {
            color: Color::BLACK,
            width: 2,
            radius: 1,
            fill_mode: rule::FillMode::Padded(15),
        }
    }
}
