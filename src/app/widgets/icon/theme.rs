use iced::widget::svg;

use crate::app::{widgets::entries::theme::Class, OnagreTheme};

impl svg::Catalog for OnagreTheme {
    type Class<'a> = Class;

    fn default<'a>() -> Self::Class<'a> {
        Class::Icon { selected: false }
    }

    fn style(&self, class: &Self::Class<'_>, _status: svg::Status) -> svg::Style {
        let Class::Icon { .. } = class else {
            panic!("attempted to set an invalid icon class")
        };

        svg::Style {
            // TODO: a style dedicated to symbolic icons
            color: None,
            // color: Some(self.0.row(*selected).icon.color.into()),
        }
    }
}
