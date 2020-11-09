use crate::style::color::OnagreColor;
use crate::style::layout::Length;
use iced_style::{container, scrollable, Background};

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(default)]
pub struct ScrollableStyles {
    pub background: OnagreColor,
    pub border_radius: u16,
    pub border_width: u16,
    pub border_color: OnagreColor,
    pub scroller: Scroller,
    pub height: Length,
    pub width: Length,
    pub scrollbar_width: u16,
    pub scroller_width: u16,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Scroller {
    pub color: OnagreColor,
    pub border_radius: u16,
    pub border_width: u16,
    pub border_color: OnagreColor,
}

impl Default for ScrollableStyles {
    fn default() -> Self {
        Self {
            background: OnagreColor::BLUE,
            border_color: OnagreColor::BLUE,
            border_radius: 0,
            border_width: 20,
            scroller: Scroller {
                color: OnagreColor::RED,
                border_radius: 2,
                border_width: 2,
                border_color: OnagreColor::GREEN,
            },
            height: Length::fill(),
            width: Length::fill(),
            scrollbar_width: 10,
            scroller_width: 10,
        }
    }
}

impl container::StyleSheet for &ScrollableStyles {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(self.background.into())),
            text_color: None,
            border_radius: self.border_radius,
            border_width: self.border_radius,
            border_color: self.border_color.into(),
        }
    }
}

impl scrollable::StyleSheet for &ScrollableStyles {
    fn active(&self) -> scrollable::Scrollbar {
        scrollable::Scrollbar {
            background: Some(Background::Color(self.background.into())),
            border_radius: self.border_radius,
            border_width: self.border_width,
            border_color: self.border_color.into(),
            scroller: scrollable::Scroller {
                color: self.scroller.color.into(),
                border_radius: self.scroller.border_radius,
                border_width: self.scroller.border_width,
                border_color: self.scroller.border_color.into(),
            },
        }
    }

    fn hovered(&self) -> scrollable::Scrollbar {
        let active = self.active();
        scrollable::Scrollbar { ..active }
    }

    fn dragging(&self) -> scrollable::Scrollbar {
        let hovered = self.active();
        scrollable::Scrollbar { ..hovered }
    }
}
