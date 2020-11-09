use crate::style::color::OnagreColor;
use crate::style::rows::RowContainerStyles;
use crate::style::scrollable::ScrollableStyles;
use crate::style::search::SearchContainerStyles;
use crate::style::theme_settings::Theme;
use iced::{container, Color};
use iced_native::Background;

impl Theme {
    pub fn load() -> Self {
        if let Ok(theme) = Theme::get() {
            theme
        } else {
            Theme::default()
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            background: OnagreColor::GREEN,
            foreground: OnagreColor::BLUE,
            border_color: OnagreColor::RED,
            border_radius: 0,
            border_width: 2,
            rows: RowContainerStyles::default(),
            scrollable: ScrollableStyles::default(),
            search: SearchContainerStyles::default(),
            menu: RowContainerStyles::mode_entries(),
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
