use crate::style::color::OnagreColor;
use crate::style::rows::RowContainerStyles;
use crate::style::scrollable::ScrollableStyles;
use crate::style::search::SearchContainerStyles;
use iced::{container, Color};
use iced_native::Background;

#[derive(Deserialize, Serialize, Debug)]
#[serde(default)]
pub struct Theme {
    pub background: OnagreColor,
    pub foreground: OnagreColor,
    pub border_color: OnagreColor,
    pub border_radius: u16,
    pub border_width: u16,
    pub rows: RowContainerStyles,
    pub scrollable: ScrollableStyles,
    pub search: SearchContainerStyles,
    pub menu: RowContainerStyles,
}

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
