use crate::ui::style::color::OnagreColor;
use crate::ui::style::rows::RowContainerStyles;
use crate::ui::style::scrollable::ScrollableStyles;
use crate::ui::style::search::SearchContainerStyles;
use iced::container;
use iced_native::Background;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(default, deny_unknown_fields)]
pub struct Theme {
    pub exit_unfocused: bool,
    pub size: (u32, u32),
    pub font: Option<String>,
    pub font_size: u16,
    pub icon_theme: Option<String>,
    pub icon_size: u16,
    pub background: OnagreColor,
    pub foreground: OnagreColor,
    pub border_color: OnagreColor,
    pub border_radius: f32,
    pub border_width: f32,
    pub rows: RowContainerStyles,
    pub scrollable: ScrollableStyles,
    pub search: SearchContainerStyles,
}

impl Theme {
    pub fn load() -> Self {
        match Theme::get() {
            Ok(theme) => theme,
            Err(err) => {
                eprintln!("Unable to load user theme: {}", err);
                Theme::default()
            }
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            exit_unfocused: false,
            size: (800, 300),
            font: None,
            font_size: 22,
            icon_size: 24,
            icon_theme: None,
            background: OnagreColor::DEFAULT_BACKGROUND,
            foreground: OnagreColor::DEFAULT_BACKGROUND,
            border_color: OnagreColor::from("#00000000").unwrap(),
            border_radius: 0.0,
            border_width: 0.0,
            rows: RowContainerStyles::default(),
            scrollable: ScrollableStyles::default(),
            search: SearchContainerStyles::default(),
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
        self
    }
}
