use crate::app::style::scrollable::scroller::ScrollerStyles;
use crate::app::style::scrollable::RowContainerStyle;
use crate::app::style::search::SearchContainerStyles;
use crate::config::color::OnagreColor;
use crate::config::padding::OnagrePadding;
use iced::{container, Color};
use iced_native::Background;

// The top level container wrapping the app
// We don't want to edit this style, it's here only to provide rounded
// Transparent corner and avoid weird behavior with the scrollable widget
pub struct AppWrapperStyle;

impl container::StyleSheet for AppWrapperStyle {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(Color::TRANSPARENT)),
            border_radius: 0.0,
            border_width: 0.0,
            text_color: None,
            border_color: Color::TRANSPARENT,
        }
    }
}

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

impl container::StyleSheet for &AppContainerStyles {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(self.background.into())),
            border_radius: self.border_radius,
            border_width: self.border_width,
            text_color: Some(self.color.into()),
            border_color: self.border_color.into(),
        }
    }
}
