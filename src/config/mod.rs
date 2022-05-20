pub mod color;
pub mod padding;
mod error;
mod helpers;

use error::ConfigError;
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;
use std::convert::TryFrom;
use std::path::Path;

use crate::ui::style::rows::icon::IconStyle;
use crate::ui::style::Theme;
use padding::OnagrePadding;
use crate::ui::style::rows::generic::GenericContainerStyle;
use crate::ui::style::rows::RowStyles;
use crate::ui::style::scrollable::RowContainerStyle;
use crate::ui::style::search::SearchContainerStyles;
use crate::ui::style::search::input::SearchInputStyles;
use crate::ui::style::app::AppContainerStyles;
use crate::ui::style::scrollable::scroller::ScrollerStyles;

#[derive(Parser)]
#[grammar = "config/grammar.pest"]
struct ThemeParser;

pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<Theme, ConfigError> {
    let content = std::fs::read_to_string(path)?;
    let pairs = ThemeParser::parse(Rule::stylesheet, &content)?
        .next()
        .unwrap();

    for pair in pairs.into_inner() {
        match pair.as_rule() {
            Rule::onagre_style => {
                return Theme::try_from(pair);
            }
            _ => unreachable!(),
        }
    }

    unreachable!()
}

impl TryFrom<Pair<'_, Rule>> for Theme {
    type Error = ConfigError;
    fn try_from(pair: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let mut theme = Theme::default();

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::exit_unfocused => theme.exit_unfocused = helpers::unwrap_attr_bool(pair),
                Rule::font_family => theme.font = Some(helpers::unwrap_attr_str(pair).to_string()),
                Rule::font_size => theme.font_size = helpers::unwrap_attr_u16(pair)?,
                Rule::icon_theme => theme.icon_theme = Some(helpers::unwrap_attr_str(pair).to_string()),
                Rule::icon_size => theme.icon_size = helpers::unwrap_attr_u16(pair)?,
                Rule::window_height => theme.size.1 = helpers::unwrap_attr_32(pair)?,
                Rule::window_width => theme.size.0 = helpers::unwrap_attr_32(pair)?,
                Rule::background => theme.background = helpers::unwrap_hex_color(pair)?,
                Rule::spacing => theme.background = helpers::unwrap_hex_color(pair)?,
                Rule::color => theme.color = helpers::unwrap_hex_color(pair)?,
                Rule::border_color => theme.border_color = helpers::unwrap_hex_color(pair)?,
                Rule::border_width => theme.border_width = helpers::unwrap_attr_f32(pair)?,
                Rule::border_radius => theme.border_radius = helpers::unwrap_attr_f32(pair)?,
                Rule::padding => theme.padding = OnagrePadding::from(helpers::unwrap_attr_u16(pair)?),
                Rule::padding_top => theme.padding.top = helpers::unwrap_attr_u16(pair)?,
                Rule::padding_bottom => theme.padding.bottom = helpers::unwrap_attr_u16(pair)?,
                Rule::padding_right => theme.padding.right = helpers::unwrap_attr_u16(pair)?,
                Rule::padding_left => theme.padding.left = helpers::unwrap_attr_u16(pair)?,
                Rule::container => theme.app_container = AppContainerStyles::try_from(pair)?,
                Rule::EOI => break,
                _ => unreachable!(),
            }
        }

        Ok(theme)
    }
}

impl TryFrom<Pair<'_, Rule>> for AppContainerStyles {
    type Error = ConfigError;

    fn try_from(pair: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let mut container = AppContainerStyles::default();

        for pair in pair.into_inner() {
            match pair.as_rule() {
                // Style
                Rule::background => container.background = helpers::unwrap_hex_color(pair)?,
                Rule::color => container.color = helpers::unwrap_hex_color(pair)?,
                Rule::border_color => container.border_color = helpers::unwrap_hex_color(pair)?,
                Rule::border_radius => container.border_radius = helpers::unwrap_attr_f32(pair)?,
                Rule::border_width => container.border_width = helpers::unwrap_attr_f32(pair)?,
                // Padding
                Rule::padding => container.padding = OnagrePadding::from(helpers::unwrap_attr_u16(pair)?),
                Rule::padding_top => container.padding.top = helpers::unwrap_attr_u16(pair)?,
                Rule::padding_bottom => container.padding.bottom = helpers::unwrap_attr_u16(pair)?,
                Rule::padding_right => container.padding.right = helpers::unwrap_attr_u16(pair)?,
                Rule::padding_left => container.padding.left = helpers::unwrap_attr_u16(pair)?,
                // Children
                Rule::search => container.search = SearchContainerStyles::try_from(pair)?,
                Rule::rows => container.rows = RowContainerStyle::try_from(pair)?,
                Rule::scrollable => container.scrollable = ScrollerStyles::try_from(pair)?,
                _ => unreachable!(),
            }
        }

        Ok(container)
    }
}

impl TryFrom<Pair<'_, Rule>> for ScrollerStyles {
    type Error = ConfigError;

    fn try_from(pair: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let mut scroller = ScrollerStyles::default();
        for pair in pair.into_inner() {
            match pair.as_rule() {
                // Style
                Rule::background => scroller.background = helpers::unwrap_hex_color(pair)?,
                Rule::border_color => scroller.border_color = helpers::unwrap_hex_color(pair)?,
                Rule::border_radius => scroller.border_radius = helpers::unwrap_attr_f32(pair)?,
                Rule::border_width => scroller.border_width = helpers::unwrap_attr_f32(pair)?,
                Rule::scrollbar_margin => scroller.scrollbar_margin = helpers::unwrap_attr_u16(pair)?,
                Rule::scrollbar_width => scroller.scrollbar_width = helpers::unwrap_attr_u16(pair)?,
                Rule::scroller => {
                    for pair in pair.into_inner() {
                        match pair.as_rule() {
                            Rule::color => scroller.scroller_color = helpers::unwrap_hex_color(pair)?,
                            Rule::border_color => scroller.scroller_border_color = helpers::unwrap_hex_color(pair)?,
                            Rule::border_width => scroller.scroller_border_width = helpers::unwrap_attr_f32(pair)?,
                            Rule::border_radius => scroller.scroller_border_radius = helpers::unwrap_attr_f32(pair)?,
                            Rule::scroller_width => scroller.scroller_width = helpers::unwrap_attr_u16(pair)?,
                            _ => unreachable!()
                        }
                    }
                }
                _ => unreachable!(),
            }
        }

        Ok(scroller)
    }
}

impl TryFrom<Pair<'_, Rule>> for SearchContainerStyles {
    type Error = ConfigError;

    fn try_from(pair: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let mut search_bar = SearchContainerStyles::default();
        for pair in pair.into_inner() {
            match pair.as_rule() {
                // Style
                Rule::background => search_bar.background = helpers::unwrap_hex_color(pair)?,
                Rule::color => search_bar.color = helpers::unwrap_hex_color(pair)?,
                Rule::border_color => search_bar.border_color = helpers::unwrap_hex_color(pair)?,
                Rule::border_radius => search_bar.border_radius = helpers::unwrap_attr_f32(pair)?,
                Rule::border_width => search_bar.border_width = helpers::unwrap_attr_f32(pair)?,

                // Layout
                Rule::padding => search_bar.padding = OnagrePadding::from(helpers::unwrap_attr_u16(pair)?),
                Rule::padding_top => search_bar.padding.top = helpers::unwrap_attr_u16(pair)?,
                Rule::padding_bottom => search_bar.padding.bottom = helpers::unwrap_attr_u16(pair)?,
                Rule::padding_right => search_bar.padding.right = helpers::unwrap_attr_u16(pair)?,
                Rule::padding_left => search_bar.padding.left = helpers::unwrap_attr_u16(pair)?,
                Rule::width => search_bar.width = helpers::unwrap_length(pair)?,
                Rule::height => search_bar.height = helpers::unwrap_length(pair)?,
                Rule::align_x => search_bar.align_x = helpers::unwrap_x(pair)?,
                Rule::align_y => search_bar.align_y = helpers::unwrap_y(pair)?,

                // Children
                Rule::bar => search_bar.input = SearchInputStyles::try_from(pair)?,
                Rule::plugin_hint => search_bar.plugin_hint = Some(GenericContainerStyle::try_from(pair)?),

                _ => unreachable!(),
            }
        }

        Ok(search_bar)
    }
}

impl TryFrom<Pair<'_, Rule>> for SearchInputStyles {
    type Error = ConfigError;

    fn try_from(pair: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let mut search_bar = SearchInputStyles::default();
        for pair in pair.into_inner() {
            match pair.as_rule() {
                // Style
                Rule::background => search_bar.background = helpers::unwrap_hex_color(pair)?,
                Rule::color => search_bar.value_color = helpers::unwrap_hex_color(pair)?,
                Rule::border_color => search_bar.border_color = helpers::unwrap_hex_color(pair)?,
                Rule::border_radius => search_bar.border_radius = helpers::unwrap_attr_f32(pair)?,
                Rule::border_width => search_bar.border_width = helpers::unwrap_attr_f32(pair)?,
                Rule::text_width => search_bar.text_width = helpers::unwrap_length(pair)?,
                Rule::selection_color => search_bar.selection_color = helpers::unwrap_hex_color(pair)?,
                Rule::placeholder_color => search_bar.placeholder_color = helpers::unwrap_hex_color(pair)?,
                Rule::font_size => search_bar.size = helpers::unwrap_attr_u16(pair)?,

                // Layout
                Rule::padding => search_bar.padding = OnagrePadding::from(helpers::unwrap_attr_u16(pair)?),
                Rule::padding_top => search_bar.padding.top = helpers::unwrap_attr_u16(pair)?,
                Rule::padding_bottom => search_bar.padding.bottom = helpers::unwrap_attr_u16(pair)?,
                Rule::padding_right => search_bar.padding.right = helpers::unwrap_attr_u16(pair)?,
                Rule::padding_left => search_bar.padding.left = helpers::unwrap_attr_u16(pair)?,
                Rule::width => search_bar.width = helpers::unwrap_length(pair)?,
                Rule::height => search_bar.height = helpers::unwrap_length(pair)?,
                Rule::align_x => search_bar.align_x = helpers::unwrap_x(pair)?,
                Rule::align_y => search_bar.align_y = helpers::unwrap_y(pair)?,
                _ => unreachable!(),
            }
        }

        Ok(search_bar)
    }
}


impl TryFrom<Pair<'_, Rule>> for RowContainerStyle {
    type Error = ConfigError;

    fn try_from(pair: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let mut rows = RowContainerStyle::default();
        for pair in pair.into_inner() {
            match pair.as_rule() {
                // Style
                Rule::color => rows.color = helpers::unwrap_hex_color(pair)?,
                Rule::background => rows.background = helpers::unwrap_hex_color(pair)?,
                Rule::border_color => rows.border_color = helpers::unwrap_hex_color(pair)?,
                Rule::border_radius => rows.border_radius = helpers::unwrap_attr_f32(pair)?,
                Rule::border_width => rows.border_width = helpers::unwrap_attr_f32(pair)?,

                // Padding
                Rule::padding => rows.padding = OnagrePadding::from(helpers::unwrap_attr_u16(pair)?),
                Rule::padding_top => rows.padding.top = helpers::unwrap_attr_u16(pair)?,
                Rule::padding_bottom => rows.padding.bottom = helpers::unwrap_attr_u16(pair)?,
                Rule::padding_right => rows.padding.right = helpers::unwrap_attr_u16(pair)?,
                Rule::padding_left => rows.padding.left = helpers::unwrap_attr_u16(pair)?,

                // Children
                Rule::default_row => rows.row = RowStyles::try_from(pair)?,
                Rule::selected_row => rows.row_selected = rows.row.try_from_cloned(pair)?,
                _ => unreachable!(),
            }
        }

        Ok(rows)
    }
}

impl RowStyles {
    // We need this to inherit row default attribute
    fn try_from_cloned(&self, pair: Pair<'_, Rule>) -> Result<Self, ConfigError> {
        let mut row_style = self.clone();

        for pair in pair.into_inner() {
            match pair.as_rule() {
                // Iced style
                Rule::background => row_style.background = helpers::unwrap_hex_color(pair)?,
                Rule::border_radius => row_style.border_radius = helpers::unwrap_attr_f32(pair)?,
                Rule::border_width => row_style.border_width = helpers::unwrap_attr_f32(pair)?,
                Rule::border_color => row_style.border_color = helpers::unwrap_hex_color(pair)?,
                Rule::color => row_style.color = helpers::unwrap_hex_color(pair)?,

                // Iced Layout
                Rule::padding => row_style.padding = OnagrePadding::from(helpers::unwrap_attr_u16(pair)?),
                Rule::padding_top => row_style.padding.top = helpers::unwrap_attr_u16(pair)?,
                Rule::padding_bottom => row_style.padding.bottom = helpers::unwrap_attr_u16(pair)?,
                Rule::padding_right => row_style.padding.right = helpers::unwrap_attr_u16(pair)?,
                Rule::padding_left => row_style.padding.left = helpers::unwrap_attr_u16(pair)?,
                Rule::width => row_style.width = helpers::unwrap_length(pair)?,
                Rule::height => row_style.height = helpers::unwrap_length(pair)?,
                Rule::align_x => row_style.align_x = helpers::unwrap_x(pair)?,
                Rule::align_y => row_style.align_y = helpers::unwrap_y(pair)?,

                // Children
                Rule::description_row => row_style.description = GenericContainerStyle::try_from(pair)?,
                Rule::title_row => row_style.title = GenericContainerStyle::try_from(pair)?,
                Rule::icon => row_style.icon = IconStyle::try_from(pair)?,
                _ => unreachable!(),
            }
        }

        Ok(row_style)

    }
}


impl TryFrom<Pair<'_, Rule>> for RowStyles {
    type Error = ConfigError;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        let mut row_style = RowStyles::default();

        for pair in pair.into_inner() {
            match pair.as_rule() {
                // Iced style
                Rule::background => row_style.background = helpers::unwrap_hex_color(pair)?,
                Rule::border_radius => row_style.border_radius = helpers::unwrap_attr_f32(pair)?,
                Rule::border_width => row_style.border_width = helpers::unwrap_attr_f32(pair)?,
                Rule::border_color => row_style.border_color = helpers::unwrap_hex_color(pair)?,
                Rule::color => row_style.color = helpers::unwrap_hex_color(pair)?,

                // Iced Layout
                Rule::padding => row_style.padding = OnagrePadding::from(helpers::unwrap_attr_u16(pair)?),
                Rule::padding_top => row_style.padding.top = helpers::unwrap_attr_u16(pair)?,
                Rule::padding_bottom => row_style.padding.bottom = helpers::unwrap_attr_u16(pair)?,
                Rule::padding_right => row_style.padding.right = helpers::unwrap_attr_u16(pair)?,
                Rule::padding_left => row_style.padding.left = helpers::unwrap_attr_u16(pair)?,
                Rule::width => row_style.width = helpers::unwrap_length(pair)?,
                Rule::height => row_style.height = helpers::unwrap_length(pair)?,
                Rule::align_x => row_style.align_x = helpers::unwrap_x(pair)?,
                Rule::align_y => row_style.align_y = helpers::unwrap_y(pair)?,

                // Children
                Rule::description_row => row_style.description = GenericContainerStyle::try_from(pair)?,
                Rule::title_row => row_style.title = GenericContainerStyle::try_from(pair)?,
                Rule::icon => row_style.icon = IconStyle::try_from(pair)?,
                _ => unreachable!(),
            }
        }

        Ok(row_style)
    }
}

impl TryFrom<Pair<'_, Rule>> for GenericContainerStyle {
    type Error = ConfigError;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        let mut inner_row_style = GenericContainerStyle::default();

        for pair in pair.into_inner() {
            match pair.as_rule() {
                // Iced style
                Rule::background => inner_row_style.background = helpers::unwrap_hex_color(pair)?,
                Rule::border_radius => inner_row_style.border_radius = helpers::unwrap_attr_f32(pair)?,
                Rule::border_width => inner_row_style.border_width = helpers::unwrap_attr_f32(pair)?,
                Rule::border_color => inner_row_style.border_color = helpers::unwrap_hex_color(pair)?,
                Rule::color => inner_row_style.color = helpers::unwrap_hex_color(pair)?,
                // Iced Layout
                Rule::padding => inner_row_style.padding = OnagrePadding::from(helpers::unwrap_attr_u16(pair)?),
                Rule::padding_top => inner_row_style.padding.top = helpers::unwrap_attr_u16(pair)?,
                Rule::padding_bottom => inner_row_style.padding.bottom = helpers::unwrap_attr_u16(pair)?,
                Rule::padding_right => inner_row_style.padding.right = helpers::unwrap_attr_u16(pair)?,
                Rule::padding_left => inner_row_style.padding.left = helpers::unwrap_attr_u16(pair)?,
                Rule::align_x => inner_row_style.align_x = helpers::unwrap_x(pair)?,
                Rule::align_y => inner_row_style.align_y = helpers::unwrap_y(pair)?,
                Rule::width => inner_row_style.width = helpers::unwrap_length(pair)?,
                Rule::height => inner_row_style.height = helpers::unwrap_length(pair)?,
                Rule::font_size => inner_row_style.font_size = helpers::unwrap_attr_u16(pair)?,
                _ => unreachable!(),
            }
        }

        Ok(inner_row_style)
    }
}


impl TryFrom<Pair<'_, Rule>> for IconStyle {
    type Error = ConfigError;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        let mut icon_style = IconStyle::default();

        for pair in pair.into_inner() {
            match pair.as_rule() {
                // Iced style
                Rule::background => icon_style.background = helpers::unwrap_hex_color(pair)?,
                Rule::border_radius => icon_style.border_radius = helpers::unwrap_attr_f32(pair)?,
                Rule::border_width => icon_style.border_width = helpers::unwrap_attr_f32(pair)?,
                Rule::border_color => icon_style.border_color = helpers::unwrap_hex_color(pair)?,
                Rule::color => icon_style.color = helpers::unwrap_hex_color(pair)?,

                // Iced Layout
                Rule::padding => icon_style.padding = OnagrePadding::from(helpers::unwrap_attr_u16(pair)?),
                Rule::padding_top => icon_style.padding.top = helpers::unwrap_attr_u16(pair)?,
                Rule::padding_bottom => icon_style.padding.bottom = helpers::unwrap_attr_u16(pair)?,
                Rule::padding_right => icon_style.padding.right = helpers::unwrap_attr_u16(pair)?,
                Rule::padding_left => icon_style.padding.left = helpers::unwrap_attr_u16(pair)?,
                Rule::width => icon_style.width = helpers::unwrap_length(pair)?,
                Rule::height => icon_style.height = helpers::unwrap_length(pair)?,
                Rule::align_x => icon_style.align_x = helpers::unwrap_x(pair)?,
                Rule::align_y => icon_style.align_y = helpers::unwrap_y(pair)?,
                _ => unreachable!(),
            }
        }

        Ok(icon_style)
    }
}

#[cfg(test)]
mod test {
    use iced::{Alignment, Length};
    use crate::config::color::OnagreColor;
    use crate::config::parse_file;
    use crate::ui::style::scrollable::RowContainerStyle;
    use crate::ui::style::Theme;
    use crate::config::padding::OnagrePadding;
    use pretty_assertions::assert_eq;
    use crate::ui::style::rows::generic::GenericContainerStyle;
    use crate::ui::style::rows::RowStyles;
    use crate::ui::style::rows::icon::IconStyle;
    use crate::ui::style::app::AppContainerStyles;

    #[test]
    fn should_parse_theme_config() {
        let theme = parse_file("tests/theme.scss");

        if let Err(err) = &theme {
            println!("{err}")
        }

        let theme = theme.unwrap();

        assert_eq!(theme, Theme {
            exit_unfocused: true,
            size: (800, 300),
            font: Some("Fira Code".to_string()),
            font_size: 30,
            icon_theme: Some("Arc".to_string()),
            padding: OnagrePadding::from(1),
            background: OnagreColor::from("#e30b0b").unwrap(),
            color: OnagreColor::from("#efe6e6").unwrap(),
            border_color: OnagreColor::from("#efe6e600").unwrap(),
            border_radius: 1.3,
            border_width: 2.0,
            app_container: AppContainerStyles::default(),
        });
    }
}
