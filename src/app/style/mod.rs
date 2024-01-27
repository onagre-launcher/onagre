use crate::app::style::app::AppContainerStyles;
use crate::app::style::rows::generic::GenericContainerStyle;
use crate::app::style::scrollable::scroller::ScrollerStyles;
use crate::app::style::search::input::SearchInputStyles;
use crate::app::style::search::SearchContainerStyles;
use crate::config::color::OnagreColor;
use crate::config::padding::OnagrePadding;
use crate::THEME_PATH;
use crate::THEME_SCALE;
use iced::widget::container::Appearance;
use iced::Background;
use iced_core::{BorderRadius, Length};
use tracing::{error, warn};

pub mod app;
pub mod rows;
pub mod scrollable;
pub mod search;

impl Theme {
    pub fn load() -> Self {
        let buf = THEME_PATH.lock().unwrap().clone();
        let theme = crate::config::parse_file(&buf);
        if let Err(err) = &theme {
            error!("Failed to parse theme {buf:?}: {err}");
            warn!("Failing back to default theme");
        };

        let mut theme = theme.unwrap_or_default();
        if let Some(scale) = THEME_SCALE.get() {
            theme = theme.scale(*scale)
        }

        theme
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
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            exit_unfocused: false,
            size: (450, 300),
            font: None,
            font_size: 18,
            // TODO: default icon theme ?
            icon_theme: Some("Papirus".to_string()),
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

impl iced::widget::container::StyleSheet for &Theme {
    type Style = iced::Theme;

    fn appearance(&self, _: &Self::Style) -> Appearance {
        Appearance {
            background: Some(Background::Color(self.background.into())),
            border_radius: BorderRadius::from(self.border_radius),
            border_width: self.border_width,
            text_color: Some(self.color.into()),
            border_color: self.border_color.into(),
        }
    }
}
