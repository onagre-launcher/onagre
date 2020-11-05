use iced::{
    container, rule, scrollable, text_input, Color,
};

use iced_native::Background;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Theme;

impl From<Theme> for Box<dyn container::StyleSheet> {
    fn from(_: Theme) -> Self {
        MainContainer.into()
    }
}

impl From<Theme> for Box<dyn text_input::StyleSheet> {
    fn from(_: Theme) -> Self {
        TextInput.into()
    }
}

impl From<Theme> for Box<dyn scrollable::StyleSheet> {
    fn from(_: Theme) -> Self {
        Scrollable.into()
    }
}

impl From<Theme> for Box<dyn rule::StyleSheet> {
    fn from(_: Theme) -> Self {
        Rule.into()
    }
}

const MAIN_BG: Color = Color::BLACK;
const MAIN_FG: Color = Color::WHITE;
const MAIN_BORDER_RADIUS: u16 = 15;
const MAIN_BORDER_WIDTH: u16 = 12;
const MAIN_BORDER_COLOR: Color = Color::from_rgb(1.0, 0.0, 0.0);

const ROW_BG: Color = MAIN_BG;
const ROW_FG: Color = MAIN_FG;
const ROW_BORDER_COLOR: Color = Color::from_rgb(0.1, 0.1, 0.3);
const ROW_BORDER_RADIUS: u16 = 1;
const ROW_BORDER_WIDTH: u16 = 1;

const ROW_SELECTED_BG: Color = MAIN_FG;
const ROW_SELECTED_FG: Color = MAIN_BG;
const ROW_SELECTED_BORDER_COLOR: Color = Color::from_rgb(0.1, 0.1, 0.3);
const ROW_SELECTED_BORDER_RADIUS: u16 = 1;
const ROW_SELECTED_BORDER_WIDTH: u16 = 1;

const INPUT_BG: Color = MAIN_FG;
const INPUT_FG: Color = MAIN_BG;
const INPUT_BORDER_RADIUS: u16 = 1;
const INPUT_BORDER_WIDTH: u16 = 1;
const INPUT_BORDER_COLOR: Color = MAIN_BG;

const INPUT_HOVERED_BG: Color = MAIN_FG;
const INPUT_HOVERED_FG: Color = MAIN_BG;
const INPUT_HOVERED_BORDER_RADIUS: u16 = 1;
const INPUT_HOVERED_BORDER_WIDTH: u16 = 1;
const INPUT_HOVERED_BORDER_COLOR: Color = MAIN_BG;

pub struct TransparentContainer;

impl container::StyleSheet for TransparentContainer {
    fn style(&self) -> container::Style {
        container::Style {
            background: Color::TRANSPARENT.into(),
            border_radius: 0,
            border_width: 2,
            text_color: Color::TRANSPARENT.into(),
            border_color: Color::from_rgb(1.0, 0.0, 0.0).into(),
        }
    }
}

pub struct MainContainer;

impl container::StyleSheet for MainContainer {
    fn style(&self) -> container::Style {
        container::Style {
            background: MAIN_BG.into(),
            border_radius: MAIN_BORDER_RADIUS,
            border_width: MAIN_BORDER_WIDTH,
            text_color: MAIN_FG.into(),
            border_color: MAIN_BORDER_COLOR,
        }
    }
}

pub struct RowContainer;

impl container::StyleSheet for RowContainer {
    fn style(&self) -> container::Style {
        container::Style {
            background: ROW_BG.into(),
            border_radius: ROW_BORDER_RADIUS,
            border_width: ROW_BORDER_WIDTH,
            text_color: ROW_FG.into(),
            border_color: ROW_BORDER_COLOR.into(),
        }
    }
}

pub struct ContainerSelected;

impl container::StyleSheet for ContainerSelected {
    fn style(&self) -> container::Style {
        container::Style {
            background: ROW_SELECTED_BG.into(),
            border_radius: ROW_SELECTED_BORDER_RADIUS,
            border_width: ROW_SELECTED_BORDER_WIDTH,
            text_color: ROW_SELECTED_FG.into(),
            border_color: ROW_SELECTED_BORDER_COLOR.into(),
        }
    }
}

pub struct TextInput;

impl text_input::StyleSheet for TextInput {
    fn active(&self) -> text_input::Style {
        text_input::Style {
            background: INPUT_FG.into(),
            border_radius: INPUT_BORDER_RADIUS,
            border_width: INPUT_BORDER_WIDTH,
            border_color: INPUT_BORDER_COLOR,
        }
    }

    fn focused(&self) -> text_input::Style {
        text_input::Style {
            background: INPUT_FG.into(),
            border_radius: INPUT_BORDER_RADIUS,
            border_width: INPUT_BORDER_WIDTH,
            border_color: INPUT_HOVERED_BORDER_COLOR,
        }
    }

    fn placeholder_color(&self) -> Color {
        Color::from_rgb(0.4, 0.4, 0.4)
    }

    fn value_color(&self) -> Color {
        Color::WHITE
    }

    fn selection_color(&self) -> Color {
        Color::from_rgb(1.0, 0.0, 0.0)
    }

    fn hovered(&self) -> text_input::Style {
        text_input::Style {
            border_width: 1,
            border_color: Color::BLACK,
            ..self.focused()
        }
    }
}

pub struct Scrollable;

impl scrollable::StyleSheet for Scrollable {
    fn active(&self) -> scrollable::Scrollbar {
        scrollable::Scrollbar {
            background: Some(Background::Color(Color::BLACK)),
            border_radius: 10,
            border_width: 20,
            border_color: Color::TRANSPARENT,
            scroller: scrollable::Scroller {
                color: Color::from_rgb(1.0, 0.0, 0.0),
                border_radius: 2,
                border_width: 0,
                border_color: Color::TRANSPARENT,
            },
        }
    }

    fn hovered(&self) -> scrollable::Scrollbar {
        let active = self.active();

        scrollable::Scrollbar {
            background: Some(Background::Color(Color::from_rgb(1.0, 1.0, 1.0))),
            scroller: scrollable::Scroller {
                color: Color::from_rgb(1.0, 0.0, 0.0),
                ..active.scroller
            },
            ..active
        }
    }

    fn dragging(&self) -> scrollable::Scrollbar {
        let hovered = self.hovered();

        scrollable::Scrollbar {
            scroller: scrollable::Scroller {
                color: Color::from_rgb(0.85, 0.85, 0.85),
                ..hovered.scroller
            },
            ..hovered
        }
    }
}

pub struct Rule;

impl rule::StyleSheet for Rule {
    fn style(&self) -> rule::Style {
        rule::Style {
            color: Color::BLACK,
            width: 2,
            radius: 1,
            fill_mode: rule::FillMode::Padded(15),
        }
    }
}
