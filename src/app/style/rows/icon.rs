use crate::config::color::OnagreColor;
use crate::config::padding::OnagrePadding;
use iced::alignment::{Horizontal, Vertical};
use iced::Length;

#[derive(Debug, PartialEq, Clone)]
pub struct IconStyle {
    // Style
    pub background: OnagreColor,
    pub border_radius: f32,
    pub border_width: f32,
    pub color: OnagreColor,
    pub border_color: OnagreColor,

    // Layout
    pub padding: OnagrePadding,
    pub width: Length,
    pub height: Length,
    pub align_x: Horizontal,
    pub align_y: Vertical,
    pub icon_size: u16,
}

impl Eq for IconStyle {}

impl Default for IconStyle {
    fn default() -> Self {
        IconStyle {
            // Style
            background: OnagreColor::DEFAULT_BACKGROUND,
            border_radius: 0.0,
            border_width: 0.0,
            border_color: OnagreColor::TRANSPARENT,
            color: OnagreColor::DEFAULT_TEXT,

            // Layout
            padding: OnagrePadding {
                top: 3,
                right: 6,
                bottom: 3,
                left: 0,
            },
            width: Length::Shrink,
            height: Length::Shrink,
            align_x: Horizontal::Center,
            align_y: Vertical::Center,
            icon_size: 22,
        }
    }
}

impl IconStyle {
    pub(crate) fn category_default() -> Self {
        Self {
            icon_size: 12,
            padding: OnagrePadding {
                top: 10,
                right: 6,
                bottom: 0,
                left: 0,
            },
            ..Default::default()
        }
    }
}
