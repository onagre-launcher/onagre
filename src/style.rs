use iced::{
    button, checkbox, container, progress_bar, radio, rule, scrollable, slider, text_input, Color,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Theme;

impl From<Theme> for Box<dyn container::StyleSheet> {
    fn from(_: Theme) -> Self {
        Container.into()
    }
}

impl From<Theme> for Box<dyn radio::StyleSheet> {
    fn from(_: Theme) -> Self {
        Radio.into()
    }
}

impl From<Theme> for Box<dyn text_input::StyleSheet> {
    fn from(_: Theme) -> Self {
        TextInput.into()
    }
}

impl From<Theme> for Box<dyn button::StyleSheet> {
    fn from(_: Theme) -> Self {
        Button.into()
    }
}

impl From<Theme> for Box<dyn scrollable::StyleSheet> {
    fn from(_: Theme) -> Self {
        Scrollable.into()
    }
}

impl From<Theme> for Box<dyn slider::StyleSheet> {
    fn from(_: Theme) -> Self {
        Slider.into()
    }
}

impl From<Theme> for Box<dyn progress_bar::StyleSheet> {
    fn from(_: Theme) -> Self {
        ProgressBar.into()
    }
}

impl From<Theme> for Box<dyn checkbox::StyleSheet> {
    fn from(_: Theme) -> Self {
        Checkbox.into()
    }
}

impl From<Theme> for Box<dyn rule::StyleSheet> {
    fn from(_: Theme) -> Self {
        Rule.into()
    }
}

const SURFACE: Color = Color::from_rgb(
    0x40 as f32 / 255.0,
    0x44 as f32 / 255.0,
    0x4B as f32 / 255.0,
);

const ACCENT: Color = Color::from_rgb(
    0x6F as f32 / 255.0,
    0xFF as f32 / 255.0,
    0xE9 as f32 / 255.0,
);

const ACTIVE: Color = Color::from_rgb(
    0x72 as f32 / 255.0,
    0x89 as f32 / 255.0,
    0xDA as f32 / 255.0,
);

const HOVERED: Color = Color::from_rgb(
    0x67 as f32 / 255.0,
    0x7B as f32 / 255.0,
    0xC4 as f32 / 255.0,
);

pub struct Container;

impl container::StyleSheet for Container {
    fn style(&self) -> container::Style {
        container::Style {
            background: Color::from_rgb8(0x36, 0x39, 0x3F).into(),
            text_color: Color::WHITE.into(),
            ..container::Style::default()
        }
    }
}

pub struct ContainerSelected;

impl container::StyleSheet for ContainerSelected {
    fn style(&self) -> container::Style {
        container::Style {
            background: Color::from_rgb(1.0, 0.0, 0.0).into(),
            text_color: Color::BLACK.into(),
            ..container::Style::default()
        }
    }
}

pub struct Radio;

impl radio::StyleSheet for Radio {
    fn active(&self) -> radio::Style {
        radio::Style {
            background: SURFACE.into(),
            dot_color: ACTIVE,
            border_width: 1,
            border_color: ACTIVE,
        }
    }

    fn hovered(&self) -> radio::Style {
        radio::Style {
            background: Color { a: 0.5, ..SURFACE }.into(),
            ..self.active()
        }
    }
}

pub struct TextInput;

impl text_input::StyleSheet for TextInput {
    fn active(&self) -> text_input::Style {
        text_input::Style {
            background: SURFACE.into(),
            border_radius: 2,
            border_width: 0,
            border_color: Color::TRANSPARENT,
        }
    }

    fn focused(&self) -> text_input::Style {
        text_input::Style {
            border_width: 1,
            border_color: ACCENT,
            ..self.active()
        }
    }

    fn hovered(&self) -> text_input::Style {
        text_input::Style {
            border_width: 1,
            border_color: Color { a: 0.3, ..ACCENT },
            ..self.focused()
        }
    }

    fn placeholder_color(&self) -> Color {
        Color::from_rgb(0.4, 0.4, 0.4)
    }

    fn value_color(&self) -> Color {
        Color::WHITE
    }

    fn selection_color(&self) -> Color {
        ACTIVE
    }
}

pub struct Button;

impl button::StyleSheet for Button {
    fn active(&self) -> button::Style {
        button::Style {
            background: ACTIVE.into(),
            border_radius: 3,
            text_color: Color::WHITE,
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            background: HOVERED.into(),
            text_color: Color::WHITE,
            ..self.active()
        }
    }

    fn pressed(&self) -> button::Style {
        button::Style {
            border_width: 1,
            border_color: Color::WHITE,
            ..self.hovered()
        }
    }
}

pub struct Scrollable;

impl scrollable::StyleSheet for Scrollable {
    fn active(&self) -> scrollable::Scrollbar {
        scrollable::Scrollbar {
            background: SURFACE.into(),
            border_radius: 2,
            border_width: 0,
            border_color: Color::TRANSPARENT,
            scroller: scrollable::Scroller {
                color: ACTIVE,
                border_radius: 2,
                border_width: 0,
                border_color: Color::TRANSPARENT,
            },
        }
    }

    fn hovered(&self) -> scrollable::Scrollbar {
        let active = self.active();

        scrollable::Scrollbar {
            background: Color { a: 0.5, ..SURFACE }.into(),
            scroller: scrollable::Scroller {
                color: HOVERED,
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

pub struct Slider;

impl slider::StyleSheet for Slider {
    fn active(&self) -> slider::Style {
        slider::Style {
            rail_colors: (ACTIVE, Color { a: 0.1, ..ACTIVE }),
            handle: slider::Handle {
                shape: slider::HandleShape::Circle { radius: 9 },
                color: ACTIVE,
                border_width: 0,
                border_color: Color::TRANSPARENT,
            },
        }
    }

    fn hovered(&self) -> slider::Style {
        let active = self.active();

        slider::Style {
            handle: slider::Handle {
                color: HOVERED,
                ..active.handle
            },
            ..active
        }
    }

    fn dragging(&self) -> slider::Style {
        let active = self.active();

        slider::Style {
            handle: slider::Handle {
                color: Color::from_rgb(0.85, 0.85, 0.85),
                ..active.handle
            },
            ..active
        }
    }
}

pub struct ProgressBar;

impl progress_bar::StyleSheet for ProgressBar {
    fn style(&self) -> progress_bar::Style {
        progress_bar::Style {
            background: SURFACE.into(),
            bar: ACTIVE.into(),
            border_radius: 10,
        }
    }
}

pub struct Checkbox;

impl checkbox::StyleSheet for Checkbox {
    fn active(&self, is_checked: bool) -> checkbox::Style {
        checkbox::Style {
            background: if is_checked { ACTIVE } else { SURFACE }.into(),
            checkmark_color: Color::WHITE,
            border_radius: 2,
            border_width: 1,
            border_color: ACTIVE,
        }
    }

    fn hovered(&self, is_checked: bool) -> checkbox::Style {
        checkbox::Style {
            background: Color {
                a: 0.8,
                ..if is_checked { ACTIVE } else { SURFACE }
            }
            .into(),
            ..self.active(is_checked)
        }
    }
}

pub struct Rule;

impl rule::StyleSheet for Rule {
    fn style(&self) -> rule::Style {
        rule::Style {
            color: SURFACE,
            width: 2,
            radius: 1,
            fill_mode: rule::FillMode::Padded(15),
        }
    }
}
