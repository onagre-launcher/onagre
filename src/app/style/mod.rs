use std::path::PathBuf;

use crate::app::style::app::AppContainerStyles;
use crate::app::style::rows::generic::GenericContainerStyle;
use crate::app::style::scrollable::scroller::ScrollerStyles;
use crate::app::style::search::input::SearchInputStyles;
use crate::app::style::search::SearchContainerStyles;
use crate::config::color::OnagreColor;
use crate::config::padding::OnagrePadding;
use iced::daemon::DefaultStyle;
use iced::widget::container;
use iced::widget::container::Style;
use iced::widget::text;
use iced::Length;
use iced::Vector;
use rows::icon::IconStyle;
use rows::RowStyles;
use scrollable::RowContainerStyle;
use tracing::{error, warn};

use super::state::Onagre;
use super::OnagreTheme;

pub mod app;
pub mod rows;
pub mod scrollable;
pub mod search;

impl Onagre {
    pub fn load_theme(&self) -> OnagreTheme {
        self.theme.clone()
    }
}

impl Theme {
    pub fn load(path: PathBuf) -> Self {
        let theme = crate::config::parse_file(&path);
        if let Err(err) = &theme {
            error!("Failed to parse theme {path:?}: {err}");
            warn!("Failing back to default theme");
        };

        theme.unwrap_or_default()
    }
}

pub(crate) trait Scale {
    fn scale(self, scale: f32) -> Self;
}

impl AsRef<Theme> for Theme {
    fn as_ref(&self) -> &Theme {
        self
    }
}

#[derive(Debug, PartialEq)]
pub struct Theme {
    // Layout
    pub exit_unfocused: bool,
    pub size: (u32, u32),
    pub font: Option<String>,
    pub font_size: u16,
    pub icon_theme: Option<String>,
    pub icon_size: u16,
    pub padding: OnagrePadding,

    // Style
    pub background: OnagreColor,
    pub color: OnagreColor,
    pub border_color: OnagreColor,
    pub border_radius: f32,
    pub border_width: f32,

    // Children
    pub app_container: AppContainerStyles,
}

impl DefaultStyle for OnagreTheme {
    fn default_style(&self) -> iced::daemon::Appearance {
        iced::daemon::Appearance {
            background_color: self.0.background.into(),
            text_color: self.0.color.into(),
        }
    }
}

impl Scale for Theme {
    fn scale(mut self, scale: f32) -> Self {
        self.app_container = self.app_container.scale(scale);
        self.icon_size = (self.icon_size as f32 * scale) as u16;
        self.size.0 = (self.size.0 as f32 * scale) as u32;
        self.size.1 = (self.size.1 as f32 * scale) as u32;
        self.padding = self.padding * scale;
        self.font_size = (self.font_size as f32 * scale) as u16;
        self
    }
}

impl Scale for Length {
    fn scale(self, scale: f32) -> Self {
        match self {
            Length::Fixed(size) => Length::Fixed(size * scale),
            _ => self,
        }
    }
}

impl Scale for u16 {
    fn scale(self, scale: f32) -> Self {
        (self as f32 * scale) as u16
    }
}

impl Scale for f32 {
    fn scale(self, scale: f32) -> Self {
        self * scale
    }
}

impl Theme {
    pub fn search(&self) -> &SearchContainerStyles {
        &self.app_container.search
    }

    pub fn search_input(&self) -> &SearchInputStyles {
        &self.app_container.search.input
    }

    pub fn plugin_hint(&self) -> Option<&GenericContainerStyle> {
        self.app_container.search.plugin_hint.as_ref()
    }

    pub fn scrollable(&self) -> &ScrollerStyles {
        &self.app_container.scrollable
    }

    pub fn app(&self) -> &AppContainerStyles {
        &self.app_container
    }

    pub fn rows(&self) -> &RowContainerStyle {
        &self.app_container.rows
    }

    pub fn row(&self, selected: bool) -> &RowStyles {
        if selected {
            &self.app_container.rows.row_selected
        } else {
            &self.app_container.rows.row
        }
    }

    pub fn title(&self, selected: bool) -> &GenericContainerStyle {
        &self.row(selected).title
    }

    pub fn description(&self, selected: bool) -> &GenericContainerStyle {
        &self.row(selected).description
    }

    pub fn icon(&self, selected: bool) -> &IconStyle {
        &self.row(selected).icon
    }

    pub fn category_icon(&self, selected: bool) -> &IconStyle {
        &self.row(selected).category_icon
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            exit_unfocused: false,
            size: (450, 300),
            font: None,
            font_size: 18,
            icon_theme: freedesktop_icons::default_theme_gtk(),
            icon_size: 24,
            background: OnagreColor::DEFAULT_BACKGROUND,
            color: OnagreColor::DEFAULT_TEXT,
            border_color: OnagreColor::TRANSPARENT,
            border_radius: 0.0,
            border_width: 0.0,
            padding: OnagrePadding::ZERO,
            app_container: AppContainerStyles::default(),
        }
    }
}

impl From<&Theme> for container::Style {
    fn from(val: &Theme) -> Self {
        Style {
            text_color: Some(val.color.into()),
            background: Some(iced::Background::Color(val.background.into())),
            border: iced::Border {
                color: val.border_color.into(),
                width: val.border_width,
                radius: iced::border::Radius::from(val.border_radius),
            },
            shadow: iced::Shadow {
                color: iced::Color::TRANSPARENT,
                offset: Vector::ZERO,
                blur_radius: 0.,
            },
        }
    }
}

impl From<&Theme> for text::Style {
    fn from(val: &Theme) -> Self {
        text::Style {
            color: Some(val.color.into()),
        }
    }
}
