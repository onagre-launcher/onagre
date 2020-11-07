use crate::style::theme_settings::{ScrollableStyles, Scroller};
use iced_style::{scrollable, Background};
use crate::style::OnagreColor;

impl Default for ScrollableStyles {
    fn default() -> Self {
        Self {
            background: OnagreColor::BLACK,
            border_radius: 0,
            border_width: 0,
            border_color: OnagreColor::BLACK,
            scroller: Scroller {
                color: OnagreColor::BLACK,
                border_radius: 2,
                border_width: 2,
                border_color: OnagreColor::WHITE
            },
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

        scrollable::Scrollbar {
            ..active
        }
    }

    fn dragging(&self) -> scrollable::Scrollbar {
        let hovered = self.hovered();

        scrollable::Scrollbar {
            ..hovered
        }
    }
}
