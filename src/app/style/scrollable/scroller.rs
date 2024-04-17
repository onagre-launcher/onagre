use crate::app::style::Scale;
use crate::config::color::OnagreColor;
use iced::widget::scrollable::Scrollbar;
use iced::Background;
use iced_core::Border;
use iced_core::border::Radius;
use iced_style::scrollable::{Appearance, StyleSheet};
use iced_style::theme::Scrollable;

#[derive(Debug, PartialEq)]
pub struct ScrollerStyles {
    pub background: OnagreColor,
    pub border_color: OnagreColor,
    pub border_radius: f32,
    pub border_width: f32,
    pub scroller_color: OnagreColor,
    pub scroller_border_radius: f32,
    pub scroller_border_width: f32,
    pub scroller_border_color: OnagreColor,
    pub scrollbar_margin: u16,
    pub scrollbar_width: u16,
    pub scroller_width: u16,
}

impl Scale for ScrollerStyles {
    fn scale(mut self, scale: f32) -> Self {
        self.border_width = self.border_width.scale(scale);
        self.scroller_border_width = self.scroller_border_width.scale(scale);
        self.scrollbar_margin = self.scrollbar_margin.scale(scale);
        self.scrollbar_width = self.scrollbar_width.scale(scale);
        self.scroller_width = self.scroller_width.scale(scale);
        self
    }
}

impl Eq for ScrollerStyles {}

impl Default for ScrollerStyles {
    fn default() -> Self {
        ScrollerStyles {
            background: OnagreColor::DEFAULT_SCROLL,
            border_radius: 0.3,
            border_width: 0.0,
            border_color: OnagreColor::TRANSPARENT,
            scroller_color: OnagreColor::DEFAULT_SCROLLER,
            scroller_border_radius: 3.0,
            scroller_border_width: 0.0,
            scroller_border_color: OnagreColor::DEFAULT_BORDER,
            scrollbar_margin: 0,
            scrollbar_width: 4,
            scroller_width: 6,
        }
    }
}

impl From<&ScrollerStyles> for Scrollable {
    fn from(_: &ScrollerStyles) -> Self {
        Scrollable::Default
    }
}

impl StyleSheet for &ScrollerStyles {
    type Style = iced::Theme;

    fn active(&self, _: &Self::Style) -> Appearance {
        Appearance {
            container: Default::default(),
            scrollbar: Scrollbar {
                background: Some(Background::Color(self.background.into())),
                border: Border {
                    color: self.border_color.into(),
                    width: self.border_width,
                    radius: Radius::from(self.border_radius),
                },
                scroller: iced::widget::scrollable::Scroller {
                    color: self.scroller_color.into(),
                    border: Border {
                        color: self.scroller_border_color.into(),
                        width: self.scroller_border_width,
                        radius: Radius::from(self.scroller_border_radius),
                    }
                },
            },
            gap: None,
        }
    }

    fn hovered(&self, style: &Self::Style, _is_mouse_over_scrollbar: bool) -> Appearance {
        self.active(style)
    }
}
