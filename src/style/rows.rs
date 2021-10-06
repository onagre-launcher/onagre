use crate::style::color::OnagreColor;
use crate::style::layout::Length;
use iced_style::{container, Background};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(default)]
pub struct RowContainerStyles {
    pub background: OnagreColor,
    pub text_color: OnagreColor,
    pub border_color: OnagreColor,
    pub border_radius: f32,
    pub border_width: f32,
    pub width: Length,
    pub height: Length,
    pub padding: u16,
    pub lines: RowEntries,
}

impl Eq for RowContainerStyles {}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct RowEntries {
    pub default: RowStyles,
    pub selected: RowStyles,
}

impl Eq for RowEntries {}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct RowStyles {
    pub background: OnagreColor,
    pub text_color: OnagreColor,
    pub border_color: OnagreColor,
    pub border_radius: f32,
    pub border_width: f32,
    pub width: Length,
    pub height: Length,
    pub padding: u16,
}

impl Eq for RowStyles {}

impl container::StyleSheet for &RowContainerStyles {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(self.background.into())),
            border_radius: self.border_radius,
            border_width: self.border_width,
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
            border_width: self.border_width,
            text_color: Some(self.text_color.into()),
            border_color: self.border_color.into(),
        }
    }
}

impl Default for RowContainerStyles {
    fn default() -> Self {
        Self {
            background: OnagreColor::DEFAULT_BACKGROUND,
            border_radius: 0.0,
            border_width: 0.0,
            text_color: OnagreColor::from("#ffffff").unwrap(),
            border_color: OnagreColor::from("#000000").unwrap(),
            height: Length::fill(),
            width: Length::shrink(),
            lines: RowEntries {
                default: RowStyles {
                    background: OnagreColor::from("#00000000").unwrap(),
                    border_radius: 0.0,
                    border_width: 0.0,
                    text_color: OnagreColor::from("#18405a").unwrap(),
                    border_color: OnagreColor::from("#000000").unwrap(),
                    height: Length::shrink(),
                    width: Length::fill(),
                    padding: 4,
                },
                selected: RowStyles {
                    background: OnagreColor::from("#63b4fbCf").unwrap(),
                    border_radius: 0.0,
                    border_width: 0.0,
                    text_color: OnagreColor::from("#0000ff").unwrap(),
                    border_color: OnagreColor::from("#000000").unwrap(),
                    height: Length::shrink(),
                    width: Length::fill(),
                    padding: 5,
                },
            },
            padding: 0,
        }
    }
}

impl RowContainerStyles {
    pub fn mode_entries() -> Self {
        Self {
            background: OnagreColor::from("#63b4fbCf").unwrap(),
            border_radius: 0.0,
            border_width: 10.0,
            text_color: OnagreColor::from("#ffffff").unwrap(),
            border_color: OnagreColor::from("#63b4fbCf").unwrap(),
            height: Length::shrink(),
            width: Length::fill(),
            lines: RowEntries {
                default: RowStyles {
                    background: OnagreColor::from("#00000000").unwrap(),
                    border_radius: 0.0,
                    border_width: 0.0,
                    text_color: OnagreColor::from("#000000").unwrap(),
                    border_color: OnagreColor::from("#000000").unwrap(),
                    height: Length::raw(0),
                    width: Length::raw(0),
                    padding: 0,
                },
                selected: RowStyles {
                    background: OnagreColor::TRANSPARENT,
                    border_radius: 0.0,
                    border_width: 0.0,
                    text_color: OnagreColor::from("#000000").unwrap(),
                    border_color: OnagreColor::from("#314c84").unwrap(),
                    height: Length::shrink(),
                    width: Length::fill(),
                    padding: 0,
                },
            },
            padding: 10,
        }
    }
}
