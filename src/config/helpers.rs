use crate::config::color::OnagreColor;
use crate::config::error::ConfigError;
use crate::config::Rule;
use iced::alignment::{Horizontal, Vertical};
use iced::Length;
use pest::iterators::Pair;

// Helper functions to get values from the css like theme config file.
// We don't mind unwrapping here since pest

pub fn unwrap_attr_u16(pair: Pair<'_, Rule>) -> Result<u16, ConfigError> {
    pair.into_inner()
        .last()
        .unwrap()
        .into_inner()
        .as_str()
        .parse::<u16>()
        .map_err(ConfigError::from)
}

pub fn unwrap_attr_32(pair: Pair<'_, Rule>) -> Result<u32, ConfigError> {
    pair.into_inner()
        .last()
        .unwrap()
        .into_inner()
        .as_str()
        .parse::<u32>()
        .map_err(ConfigError::from)
}

pub fn unwrap_attr_f32(pair: Pair<'_, Rule>) -> Result<f32, ConfigError> {
    pair.into_inner()
        .last()
        .unwrap()
        .into_inner()
        .as_str()
        .parse::<f32>()
        .map_err(ConfigError::from)
}

pub fn unwrap_attr_str(pair: Pair<'_, Rule>) -> &str {
    pair.into_inner().last().unwrap().into_inner().as_str()
}

pub fn unwrap_attr_bool(pair: Pair<'_, Rule>) -> bool {
    let value = pair.into_inner().last().unwrap().as_str();

    value == "true"
}

pub fn unwrap_hex_color(pair: Pair<'_, Rule>) -> Result<OnagreColor, ConfigError> {
    let color = pair.into_inner().last().unwrap().as_str();

    OnagreColor::from(color)
}

pub fn unwrap_x(pair: Pair<'_, Rule>) -> Result<Horizontal, ConfigError> {
    let alignment = pair.into_inner().last().unwrap();
    let pair = alignment.into_inner().next().unwrap();

    match pair.as_rule() {
        Rule::left => Ok(Horizontal::Left),
        Rule::center => Ok(Horizontal::Center),
        Rule::right => Ok(Horizontal::Right),
        _ => unreachable!(),
    }
}

pub fn unwrap_y(pair: Pair<'_, Rule>) -> Result<Vertical, ConfigError> {
    let alignment = pair.into_inner().last().unwrap();
    let pair = alignment.into_inner().next().unwrap();

    match pair.as_rule() {
        Rule::top => Ok(Vertical::Top),
        Rule::center => Ok(Vertical::Center),
        Rule::bottom => Ok(Vertical::Bottom),
        _ => unreachable!(),
    }
}

pub fn unwrap_length(pair: Pair<'_, Rule>) -> Result<Length, ConfigError> {
    let lenght = pair.into_inner().last().unwrap();
    let mut lenght = lenght.into_inner();
    let pair = lenght.next().unwrap();
    Ok(match pair.as_rule() {
        Rule::fill_portion => {
            let value = lenght.next().unwrap().as_str();
            let value = value.parse::<u16>()?;
            Length::FillPortion(value)
        }
        Rule::fill => Length::Fill,
        Rule::shrink => Length::Shrink,
        Rule::number => {
            let value = pair.as_str();
            let value = value.parse::<u16>()?;
            Length::Fixed(value as f32)
        }
        _ => unreachable!(),
    })
}

#[cfg(test)]
mod test {
    use crate::config::color::OnagreColor;
    use crate::config::helpers::{
        unwrap_attr_bool, unwrap_attr_f32, unwrap_attr_str, unwrap_attr_u16, unwrap_hex_color,
        unwrap_length, unwrap_x, unwrap_y,
    };
    use crate::config::{Rule, ThemeParser};
    use iced::alignment::{Horizontal, Vertical};
    use iced::Length;
    use pest::Parser;
    use speculoos::prelude::*;

    #[test]
    fn should_parse_align_x() {
        let pair = ThemeParser::parse(Rule::align_x, "--align-x: center;")
            .unwrap()
            .next()
            .unwrap();

        let align = unwrap_x(pair);

        asserting!("Should parse '--align-x' attribute")
            .that(&align)
            .is_ok()
            .is_equal_to(Horizontal::Center);
    }

    #[test]
    fn should_parse_align_y() {
        let pair = ThemeParser::parse(Rule::align_y, "--align-y: center;")
            .unwrap()
            .next()
            .unwrap();

        let align = unwrap_y(pair);

        asserting!("Should parse '--align-y' attribute")
            .that(&align)
            .is_ok()
            .is_equal_to(Vertical::Center);
    }

    #[test]
    fn should_parse_spacing() {
        let pair = ThemeParser::parse(Rule::spacing, "--spacing: 1px;")
            .unwrap()
            .next()
            .unwrap();

        let spacing = unwrap_attr_u16(pair);

        asserting!("Should parse 'spacing' attribute")
            .that(&spacing)
            .is_ok()
            .is_equal_to(1);
    }

    #[test]
    fn should_parse_border_width() {
        let pair = ThemeParser::parse(Rule::border_width, "border-width: 1px;")
            .unwrap()
            .next()
            .unwrap();

        let border_width = unwrap_attr_u16(pair);

        asserting!("Should parse 'border-width' attribute")
            .that(&border_width)
            .is_ok()
            .is_equal_to(1);
    }

    #[test]
    fn should_parse_border_radius() {
        let pair = ThemeParser::parse(Rule::border_radius, "border-radius: 1%;")
            .unwrap()
            .next()
            .unwrap();

        let border_radius = unwrap_attr_f32(pair);

        asserting!("Should parse 'border-radius' attribute")
            .that(&border_radius)
            .is_ok()
            .is_equal_to(1.0);
    }

    #[test]
    fn should_parse_border_color() {
        let pair = ThemeParser::parse(Rule::border_color, "border-color: #ffffff;")
            .unwrap()
            .next()
            .unwrap();

        let border_color = unwrap_hex_color(pair);

        asserting!("Should parse 'border-color' attribute")
            .that(&border_color)
            .is_ok()
            .is_equal_to(&OnagreColor::from("#ffffff").unwrap());
    }

    #[test]
    fn should_parse_color() {
        let pair = ThemeParser::parse(Rule::color, "color: #ffffff;")
            .unwrap()
            .next()
            .unwrap();

        let color = unwrap_hex_color(pair);

        asserting!("Should parse 'color' attribute")
            .that(&color)
            .is_ok()
            .is_equal_to(&OnagreColor::from("#ffffff").unwrap());
    }

    #[test]
    fn should_parse_background() {
        let pair = ThemeParser::parse(Rule::background, "background: #ffffff;")
            .unwrap()
            .next()
            .unwrap();

        let background = unwrap_hex_color(pair);

        asserting!("Should parse 'background' attribute")
            .that(&background)
            .is_ok()
            .is_equal_to(&OnagreColor::from("#ffffff").unwrap());
    }

    #[test]
    fn should_parse_window_width() {
        let pair = ThemeParser::parse(Rule::window_width, "width: 100px;")
            .unwrap()
            .next()
            .unwrap();

        let window_width = unwrap_attr_u16(pair);

        asserting!("Should parse '--window-width' attribute")
            .that(&window_width)
            .is_ok()
            .is_equal_to(100);
    }

    #[test]
    fn should_parse_window_height() {
        let pair = ThemeParser::parse(Rule::window_height, "height: 100px;")
            .unwrap()
            .next()
            .unwrap();

        let window_height = unwrap_attr_u16(pair);

        asserting!("Should parse '--window-height' attribute")
            .that(&window_height)
            .is_ok()
            .is_equal_to(100);
    }

    #[test]
    fn should_parse_padding_top() {
        let pair = ThemeParser::parse(Rule::padding_top, "padding-top: 10px;")
            .unwrap()
            .next()
            .unwrap();

        let padding_top = unwrap_attr_u16(pair);

        asserting!("Should parse 'padding-top' attribute")
            .that(&padding_top)
            .is_ok()
            .is_equal_to(10);
    }

    #[test]
    fn should_parse_padding_bottom() {
        let pair = ThemeParser::parse(Rule::padding_bottom, "padding-bottom: 10px;")
            .unwrap()
            .next()
            .unwrap();

        let padding_bottom = unwrap_attr_u16(pair);

        asserting!("Should parse 'padding-bottom' attribute")
            .that(&padding_bottom)
            .is_ok()
            .is_equal_to(10);
    }

    #[test]
    fn should_parse_padding_right() {
        let pair = ThemeParser::parse(Rule::padding_right, "padding-right: 10px;")
            .unwrap()
            .next()
            .unwrap();

        let padding_right = unwrap_attr_u16(pair);

        asserting!("Should parse 'padding-right' attribute")
            .that(&padding_right)
            .is_ok()
            .is_equal_to(10);
    }

    #[test]
    fn should_parse_padding_left() {
        let pair = ThemeParser::parse(Rule::padding_left, "padding-left: 10px;")
            .unwrap()
            .next()
            .unwrap();

        let padding_left = unwrap_attr_u16(pair);

        asserting!("Should parse 'padding-left' attribute")
            .that(&padding_left)
            .is_ok()
            .is_equal_to(10);
    }

    #[test]
    fn should_parse_padding() {
        let pair = ThemeParser::parse(Rule::padding, "padding: 10px;")
            .unwrap()
            .next()
            .unwrap();

        let padding = unwrap_attr_u16(pair);

        asserting!("Should parse 'padding' attribute")
            .that(&padding)
            .is_ok()
            .is_equal_to(10);
    }

    #[test]
    fn should_parse_width_fill() {
        let pair = ThemeParser::parse(Rule::width, r#"--width: fill;"#)
            .unwrap()
            .next()
            .unwrap();

        let width = unwrap_length(pair);

        asserting!("Should parse 'width' 'fill' attribute")
            .that(&width)
            .is_ok()
            .is_equal_to(Length::Fill);
    }

    #[test]
    fn should_parse_width_fill_portion() {
        let pair = ThemeParser::parse(Rule::width, r#"--width: fill-portion 2;"#)
            .unwrap()
            .next()
            .unwrap();

        let width = unwrap_length(pair);

        asserting!("Should parse 'width' 'fill-portion' attribute")
            .that(&width)
            .is_ok()
            .is_equal_to(Length::FillPortion(2));
    }

    #[test]
    fn should_parse_width_units() {
        let pair = ThemeParser::parse(Rule::width, r#"--width: 24px;"#)
            .unwrap()
            .next()
            .unwrap();

        let width = unwrap_length(pair);

        asserting!("Should parse 'width' 'fixed' attribute")
            .that(&width)
            .is_ok()
            .is_equal_to(Length::Fixed(24.0));
    }

    #[test]
    fn should_parse_width_shrink() {
        let pair = ThemeParser::parse(Rule::width, r#"--width: shrink;"#)
            .unwrap()
            .next()
            .unwrap();

        let width = unwrap_length(pair);

        asserting!("Should parse 'width' 'shrink' attribute")
            .that(&width)
            .is_ok()
            .is_equal_to(Length::Shrink);
    }

    #[test]
    fn should_parse_height_fill() {
        let pair = ThemeParser::parse(Rule::height, r#"--height: fill;"#)
            .unwrap()
            .next()
            .unwrap();

        let height = unwrap_length(pair);

        asserting!("Should parse 'height' 'fill' attribute")
            .that(&height)
            .is_ok()
            .is_equal_to(Length::Fill);
    }

    #[test]
    fn should_parse_height_fill_portion() {
        let pair = ThemeParser::parse(Rule::height, r#"--height: fill-portion 2;"#)
            .unwrap()
            .next()
            .unwrap();

        let height = unwrap_length(pair);

        asserting!("Should parse 'height' 'fill-portion' attribute")
            .that(&height)
            .is_ok()
            .is_equal_to(Length::FillPortion(2));
    }

    #[test]
    fn should_parse_height_units() {
        let pair = ThemeParser::parse(Rule::height, r#"--height: 24px;"#)
            .unwrap()
            .next()
            .unwrap();

        let height = unwrap_length(pair);

        asserting!("Should parse 'height' 'fixed' attribute")
            .that(&height)
            .is_ok()
            .is_equal_to(Length::Fixed(24.0));
    }

    #[test]
    fn should_parse_height_skrink() {
        let pair = ThemeParser::parse(Rule::height, r#"--height: shrink;"#)
            .unwrap()
            .next()
            .unwrap();

        let height = unwrap_length(pair);

        asserting!("Should parse 'height' 'shrink' attribute")
            .that(&height)
            .is_ok()
            .is_equal_to(Length::Shrink);
    }

    #[test]
    fn should_parse_icon_theme() {
        let pair = ThemeParser::parse(Rule::icon_theme, r#"--icon-theme: "Arc";"#)
            .unwrap()
            .next()
            .unwrap();

        let icon_theme = unwrap_attr_str(pair);

        asserting!("Should parse 'icon_theme' attribute")
            .that(&icon_theme)
            .is_equal_to("Arc");
    }

    #[test]
    fn should_parse_font_size() {
        let pair = ThemeParser::parse(Rule::font_size, r#"font-size: 24px;"#)
            .unwrap()
            .next()
            .unwrap();

        let font_size = unwrap_attr_u16(pair);

        asserting!("Should parse 'font_size' attribute")
            .that(&font_size)
            .is_ok()
            .is_equal_to(24);
    }

    #[test]
    fn should_parse_font_family() {
        let pair = ThemeParser::parse(Rule::font_family, r#"--font-family: "Monospace";"#)
            .unwrap()
            .next()
            .unwrap();

        let font_family = unwrap_attr_str(pair);

        asserting!("Should parse 'font_family' attribute")
            .that(&font_family)
            .is_equal_to("Monospace");
    }

    #[test]
    fn should_parse_exit_unfocused() {
        let pair = ThemeParser::parse(Rule::exit_unfocused, "--exit-unfocused: true;")
            .unwrap()
            .next()
            .unwrap();

        let exit_unfocused = unwrap_attr_bool(pair);

        asserting!("Should parse 'exit_unfocused' attribute")
            .that(&exit_unfocused)
            .is_true();
    }
}
