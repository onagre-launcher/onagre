use iced::widget::button;
use iced::widget::container;
use iced::widget::scrollable;
use iced::widget::text;
use iced::widget::text_input;
use iced::Border;
use iced::Color;

use crate::app::OnagreTheme;

#[derive(Debug, Clone)]
pub enum Class {
    Main,
    AppContainer,
    Description { selected: bool },
    Title { selected: bool },
    Row { selected: bool },
    Rows,
    RowClickable,
    PluginHint,
    SearchInput,
    Icon { selected: bool },
    CategoryIcon { selected: bool },
    SearchBar,
}

impl container::Catalog for OnagreTheme {
    type Class<'a> = Class;

    fn default<'a>() -> Self::Class<'a> {
        Class::Main
    }

    fn style(&self, class: &Self::Class<'_>) -> container::Style {
        match class {
            Class::Main => self.0.as_ref().into(),
            Class::AppContainer => self.0.app().into(),
            Class::Description { selected } => self.0.description(*selected).into(),
            Class::Title { selected } => self.0.title(*selected).into(),
            Class::Rows => self.0.rows().into(),
            Class::Row { selected } => self.0.row(*selected).into(),
            Class::Icon { selected } => self.0.icon(*selected).into(),
            Class::CategoryIcon { selected } => self.0.category_icon(*selected).into(),
            Class::PluginHint => match self.0.app().search.plugin_hint.as_ref() {
                Some(style) => style.into(),
                None => self.0.search().into(),
            },
            Class::SearchInput => self.0.search_input().into(),
            Class::SearchBar => self.0.search().into(),
            _ => unreachable!(),
        }
    }
}

impl button::Catalog for OnagreTheme {
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

impl scrollable::Catalog for OnagreTheme {
    type Class<'a> = Class;

    fn default<'a>() -> Self::Class<'a> {
        Class::RowClickable
    }

    fn style(&self, _class: &Self::Class<'_>, _status: scrollable::Status) -> scrollable::Style {
        (&self.0.app().scrollable).into()
    }
}

impl text_input::Catalog for OnagreTheme {
    type Class<'a> = Class;

    fn default<'a>() -> Self::Class<'a> {
        Class::SearchInput
    }

    fn style(&self, _: &Self::Class<'_>, _: text_input::Status) -> text_input::Style {
        (&self.0.app().search.input).into()
    }
}

impl text::Catalog for OnagreTheme {
    type Class<'a> = text::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(text_default)
    }

    fn style(&self, class: &Self::Class<'_>) -> text::Style {
        class(self)
    }
}

pub fn text_default(theme: &OnagreTheme) -> text::Style {
    text::Style {
        color: Some(theme.0.color.into()),
    }
}
