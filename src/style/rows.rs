use crate::style::color::OnagreColor;
use crate::style::layout::Length;
use iced_style::{container, Background};

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(default)]
pub struct RowContainerStyles {
    pub background: OnagreColor,
    pub border_radius: u16,
    pub border_width: u16,
    pub text_color: OnagreColor,
    pub border_color: OnagreColor,
    pub width: Length,
    pub height: Length,
    pub lines: RowEntries,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct RowEntries {
    pub default: RowStyles,
    pub selected: RowStyles,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct RowStyles {
    pub background: OnagreColor,
    pub text_color: OnagreColor,
    pub border_color: OnagreColor,
    pub border_radius: u16,
    pub border_width: u16,
    pub width: Length,
    pub height: Length,
}

impl container::StyleSheet for &RowContainerStyles {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(self.background.into())),
            border_radius: self.border_radius,
            border_width: self.border_radius,
            text_color: Some(self.text_color.into()),
            border_color: self.border_color.into(),
        }
    }
}

impl container::StyleSheet for &RowStyles {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(self.background.into())),
            border_radius: self.border_radius,
            border_width: self.border_radius,
            text_color: Some(self.text_color.into()),
            border_color: self.border_color.into(),
        }
    }
}

impl Default for RowContainerStyles {
    fn default() -> Self {
        Self {
            background: OnagreColor::BLUE,
            border_radius: 0,
            border_width: 2,
            text_color: OnagreColor::WHITE,
            border_color: OnagreColor::GREEN,
            height: Length::fill(),
            width: Length::fill(),
            lines: RowEntries {
                default: RowStyles {
                    background: OnagreColor::BLACK,
                    border_radius: 0,
                    border_width: 2,
                    text_color: OnagreColor::RED,
                    border_color: OnagreColor::RED,
                    height: Length::shrink(),
                    width: Length::fill(),
                },
                selected: RowStyles {
                    background: OnagreColor::WHITE,
                    border_radius: 0,
                    border_width: 2,
                    text_color: OnagreColor::BLUE,
                    border_color: OnagreColor::RED,
                    height: Length::shrink(),
                    width: Length::fill(),
                },
            },
        }
    }
}

impl RowContainerStyles {
    pub fn mode_entries() -> Self {
        Self {
            background: OnagreColor::BLACK,
            border_radius: 0,
            border_width: 3,
            text_color: OnagreColor::WHITE,
            border_color: OnagreColor::GREEN,
            height: Length::flex(1),
            width: Length::flex(1),
            lines: RowEntries {
                default: RowStyles {
                    background: OnagreColor::BLACK,
                    border_radius: 0,
                    border_width: 2,
                    text_color: OnagreColor::RED,
                    border_color: OnagreColor::RED,
                    height: Length::shrink(),
                    width: Length::fill(),
                },
                selected: RowStyles {
                    background: OnagreColor::WHITE,
                    border_radius: 0,
                    border_width: 2,
                    text_color: OnagreColor::BLUE,
                    border_color: OnagreColor::RED,
                    height: Length::shrink(),
                    width: Length::fill(),
                },
            },
        }
    }
}
