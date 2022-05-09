use crate::ui::style::color::OnagreColor;
use iced_style::{container, Background};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct RowContainerStyles {
    pub background: OnagreColor,
    pub text_color: OnagreColor,
    pub border_color: OnagreColor,
    pub border_radius: f32,
    pub border_width: f32,
    pub padding: u16,
    pub lines: RowEntries,
}

impl Eq for RowContainerStyles {}

impl Default for RowContainerStyles {
    fn default() -> Self {
        Self {
            background: OnagreColor::DEFAULT_BACKGROUND,
            border_radius: 0.0,
            border_width: 0.0,
            text_color: OnagreColor::from("#ffffff").unwrap(),
            border_color: OnagreColor::from("#000000").unwrap(),
            lines: RowEntries::default(),
            padding: 0,
        }
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, default)]
pub struct RowEntries {
    pub default: RowStyles,
    pub selected: RowStyles,
}

impl Eq for RowEntries {}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, default)]
pub struct RowStyles {
    pub background: OnagreColor,
    pub text_color: OnagreColor,
    pub border_color: OnagreColor,
    pub border_radius: f32,
    pub border_width: f32,
    pub padding: u16,
}

impl Default for RowStyles {
    fn default() -> Self {
        RowStyles {
            background: OnagreColor::from("#00000000").unwrap(),
            border_radius: 0.0,
            border_width: 0.0,
            text_color: OnagreColor::from("#18405a").unwrap(),
            border_color: OnagreColor::from("#000000").unwrap(),
            padding: 5,
        }
    }
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

impl Default for RowEntries {
    fn default() -> Self {
        RowEntries {
            default: RowStyles::default(),
            selected: RowStyles {
                background: OnagreColor::from("#63b4fbCf").unwrap(),
                border_radius: 0.0,
                border_width: 0.0,
                text_color: OnagreColor::from("#0000ff").unwrap(),
                border_color: OnagreColor::from("#000000").unwrap(),
                padding: 5,
            },
        }
    }
}
