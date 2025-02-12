use crate::app::style::Scale;
use crate::config::color::OnagreColor;
use crate::config::padding::OnagrePadding;
use generic::GenericContainerStyle;
use iced::alignment::{Horizontal, Vertical};

use iced::widget::container::Style;
use iced::{Length, Vector};
use icon::IconStyle;

pub mod generic;
pub mod icon;

pub fn row_style(_: &iced::Theme) -> Style {
    todo!()
}

#[derive(Debug, PartialEq, Clone)]
pub struct RowStyles {
    // Layout
    pub padding: OnagrePadding,
    pub width: Length,
    pub height: Length,
    pub spacing: u16,
    pub align_x: Horizontal,
    pub align_y: Vertical,

    // Style
    pub background: OnagreColor,
    pub border_radius: f32,
    pub border_width: f32,
    pub color: OnagreColor,
    pub border_color: OnagreColor,
    pub hide_description: bool,
    pub hide_category_icon: bool,

    // Children
    pub title: GenericContainerStyle,
    pub description: GenericContainerStyle,
    pub icon: IconStyle,
    pub category_icon: IconStyle,
}

impl From<&RowStyles> for Style {
    fn from(val: &RowStyles) -> Self {
        Style {
            text_color: Some(val.color.into()),
            background: Some(iced::Background::Color(val.background.into())),
            border: iced::Border {
                color: val.border_color.into(),
                width: val.border_width,
                radius: iced::border::Radius::from(val.border_radius),
            },
            shadow: iced::Shadow {
                color: iced::Color::TRANSPARENT,
                offset: Vector::ZERO,
                blur_radius: 0.,
            },
        }
    }
}

impl Scale for RowStyles {
    fn scale(mut self, scale: f32) -> Self {
        self.height = self.height.scale(scale);
        self.width = self.width.scale(scale);
        self.spacing = self.spacing.scale(scale);
        self.border_width = self.border_width.scale(scale);
        self.title = self.title.scale(scale);
        self.description = self.description.scale(scale);
        self.icon = self.icon.scale(scale);
        self.category_icon = self.category_icon.scale(scale);
        self
    }
}

impl Default for RowStyles {
    fn default() -> Self {
        RowStyles {
            width: Length::Fill,
            height: Length::Shrink,
            background: OnagreColor::DEFAULT_BACKGROUND,
            color: OnagreColor::DEFAULT_TEXT,
            border_radius: 0.0,
            border_width: 0.0,
            padding: OnagrePadding::from(5),
            align_x: Horizontal::Right,
            align_y: Vertical::Bottom,
            border_color: OnagreColor::RED,
            hide_description: false,
            hide_category_icon: false,
            title: GenericContainerStyle::default(),
            description: GenericContainerStyle::description_default(),
            icon: Default::default(),
            category_icon: IconStyle::category_default(),
            spacing: 2,
        }
    }
}

impl RowStyles {
    pub fn default_selected() -> Self {
        Self {
            color: OnagreColor::WHITE,
            title: GenericContainerStyle {
                color: OnagreColor::WHITE,
                ..Default::default()
            },
            description: GenericContainerStyle {
                color: OnagreColor::WHITE,
                ..GenericContainerStyle::description_default()
            },
            ..Default::default()
        }
    }
}
impl Eq for RowStyles {}
