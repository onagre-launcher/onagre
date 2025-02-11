use iced::widget::button;
use iced::widget::container;
use iced::widget::text;
use iced::Border;
use iced::Color;

use crate::app::style::{self};

pub enum Class {
    Main,
    Description { selected: bool },
    Title { selected: bool },
    RowClickable,
    Row,
}

impl container::Catalog for style::Theme {
    type Class<'a> = Class;

    fn default<'a>() -> Self::Class<'a> {
        Class::Main
    }

    fn style(&self, class: &Self::Class<'_>) -> container::Style {
        match class {
            Class::Main => self.into(),
            Class::Description { selected } if *selected => {
                (&self.app().rows.row_selected.description).into()
            }
            Class::Title { selected } if *selected => (&self.app().rows.row_selected.title).into(),
            Class::Description { .. } => (&self.app().rows.row.description).into(),
            Class::Title { .. } => (&self.app().rows.row.title).into(),
            Class::Row { .. } => (&self.app().rows.row).into(),
            _ => unreachable!(),
        }
    }
}
impl button::Catalog for style::Theme {
    type Class<'a> = Class;

    fn default<'a>() -> Self::Class<'a> {
        Class::RowClickable
    }

    fn style(&self, _: &Self::Class<'_>, _: button::Status) -> button::Style {
        button::Style {
            background: None,
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 0.0.into(),
            },
            text_color: Color::BLACK,
            shadow: Default::default(),
        }
    }
}
impl text::Catalog for style::Theme {
    type Class<'a> = text::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(text_default)
    }

    fn style(&self, class: &Self::Class<'_>) -> text::Style {
        class(self)
    }
}

pub fn text_default(theme: &style::Theme) -> text::Style {
    text::Style {
        color: Some(theme.color.into()),
    }
}
