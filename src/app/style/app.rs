use crate::app::style::scrollable::scroller::ScrollerStyles;
use crate::app::style::scrollable::RowContainerStyle;
use crate::app::style::search::SearchContainerStyles;
use crate::app::style::Scale;
use crate::config::color::OnagreColor;
use crate::config::padding::OnagrePadding;
use iced_core::border::Radius;
use iced_core::{Background, Border};
use iced_style::container::{Appearance, StyleSheet};

// The top level container wrapping the app
// We don't want to edit this style, it's here only to provide rounded
// Transparent corner and avoid weird behavior with the scrollable widget
pub struct AppWrapperStyle;

#[derive(Debug, PartialEq)]
pub struct AppContainerStyles {
    // Style
    pub background: OnagreColor,
    pub color: OnagreColor,
    pub border_color: OnagreColor,
    pub border_radius: f32,
    pub border_width: f32,

    // Layout
    pub padding: OnagrePadding,

    // Children
    pub search: SearchContainerStyles,
    pub rows: RowContainerStyle,
    pub scrollable: ScrollerStyles,
}

impl Scale for AppContainerStyles {
    fn scale(mut self, scale: f32) -> Self {
        self.search = self.search.scale(scale);
        self.padding = self.padding * scale;
        self.rows = self.rows.scale(scale);
        self.search = self.search.scale(scale);
        self.scrollable = self.scrollable.scale(scale);
        self.border_width = self.border_width.scale(scale);
        self
    }
}

impl StyleSheet for &AppContainerStyles {
    type Style = iced::Theme;

    fn appearance(&self, _: &Self::Style) -> Appearance {
        Appearance {
            text_color: Some(self.color.into()),
            background: Some(Background::Color(self.background.into())),
            border: Border {
                color: self.border_color.into(),
                width: self.border_width,
                radius: Radius::from(self.border_radius),
            },
            shadow: Default::default(),
        }
    }
}

impl Default for AppContainerStyles {
    fn default() -> Self {
        Self {
            background: OnagreColor::DEFAULT_BACKGROUND,
            color: OnagreColor::DEFAULT_TEXT,
            border_color: OnagreColor::RED,
            border_radius: 0.0,
            border_width: 0.0,
            padding: OnagrePadding::from(20),
            search: Default::default(),
            rows: Default::default(),
            scrollable: Default::default(),
        }
    }
}
