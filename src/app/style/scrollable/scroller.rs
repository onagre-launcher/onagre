use crate::app::style::Scale;
use crate::config::color::OnagreColor;
use iced::widget::scrollable::Scrollbar;
use iced::Background;
use iced_core::BorderRadius;
use iced_style::scrollable::StyleSheet;
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

    fn active(&self, _: &Self::Style) -> Scrollbar {
        Scrollbar {
            background: Some(Background::Color(self.background.into())),
            border_radius: BorderRadius::from(self.border_radius),
            border_width: self.border_width,
            border_color: self.border_color.into(),
            scroller: iced::widget::scrollable::Scroller {
                color: self.scroller_color.into(),
                border_radius: BorderRadius::from(self.scroller_border_radius),
                border_width: self.scroller_border_width,
                border_color: self.scroller_border_color.into(),
            },
        }
    }

    fn hovered(&self, style: &Self::Style, _is_mouse_over_scrollbar: bool) -> Scrollbar {
        self.active(style)
    }
}
