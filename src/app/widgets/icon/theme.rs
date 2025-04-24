use iced::widget::svg;
use tracing::error;

use crate::app::{widgets::entries::theme::Class, OnagreTheme};

impl svg::Catalog for OnagreTheme {
    type Class<'a> = Class;

    fn default<'a>() -> Self::Class<'a> {
        Class::Icon { selected: false }
    }

    fn style(&self, class: &Self::Class<'_>, _status: svg::Status) -> svg::Style {
        match class {
            Class::Icon { .. } => svg::Style { color: None },
            Class::CategoryIcon { selected } => {
                let color = Some(self.0.row(*selected).category_icon.color.into());
                error!("{color:?}");
                svg::Style { color }
            }
            _ => unreachable!("attempted to set an invalid icon class"),
        }
    }
}
