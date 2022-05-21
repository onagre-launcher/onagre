pub mod color;
mod error;
mod helpers;
mod inheritance;
pub mod padding;

use error::ConfigError;
use inheritance::Inherit;
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;
use std::convert::TryFrom;
use std::path::Path;

use crate::app::style::app::AppContainerStyles;
use crate::app::style::rows::generic::GenericContainerStyle;
use crate::app::style::rows::icon::IconStyle;
use crate::app::style::rows::RowStyles;
use crate::app::style::scrollable::scroller::ScrollerStyles;
use crate::app::style::scrollable::RowContainerStyle;
use crate::app::style::search::input::SearchInputStyles;
use crate::app::style::search::SearchContainerStyles;
use crate::app::style::Theme;
use padding::OnagrePadding;

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

trait ApplyConfig {
    fn apply(&mut self, pair: Pair<'_, Rule>) -> Result<(), ConfigError>;
}

impl TryFrom<Pair<'_, Rule>> for Theme {
    type Error = ConfigError;
    fn try_from(pair: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let mut theme = Theme::base();

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::exit_unfocused => theme.exit_unfocused = helpers::unwrap_attr_bool(pair),
                Rule::font_family => theme.font = Some(helpers::unwrap_attr_str(pair).to_string()),
                Rule::font_size => {
                    theme.font_size = helpers::unwrap_attr_u16(pair)?;
                    theme.propagate_font_size();
                }
                Rule::icon_theme => {
                    theme.icon_theme = Some(helpers::unwrap_attr_str(pair).to_string())
                }
                Rule::icon_size => {
                    theme.icon_size = helpers::unwrap_attr_u16(pair)?;
                    theme.propagate_icon_size();
                }
                Rule::window_height => theme.size.1 = helpers::unwrap_attr_32(pair)?,
                Rule::window_width => theme.size.0 = helpers::unwrap_attr_32(pair)?,
                Rule::background => {
                    theme.background = helpers::unwrap_hex_color(pair)?;
                    theme.propagate_background();
                }
                Rule::spacing => theme.background = helpers::unwrap_hex_color(pair)?,
                Rule::color => {
                    theme.color = helpers::unwrap_hex_color(pair)?;
                    theme.propagate_color();
                }
                Rule::border_color => theme.border_color = helpers::unwrap_hex_color(pair)?,
                Rule::border_width => theme.border_width = helpers::unwrap_attr_f32(pair)?,
                Rule::border_radius => theme.border_radius = helpers::unwrap_attr_f32(pair)?,
                Rule::padding => {
                    theme.padding = OnagrePadding::from(helpers::unwrap_attr_u16(pair)?)
                }
                Rule::padding_top => theme.padding.top = helpers::unwrap_attr_u16(pair)?,
                Rule::padding_bottom => theme.padding.bottom = helpers::unwrap_attr_u16(pair)?,
                Rule::padding_right => theme.padding.right = helpers::unwrap_attr_u16(pair)?,
                Rule::padding_left => theme.padding.left = helpers::unwrap_attr_u16(pair)?,
                Rule::container => theme.app_container.apply(pair)?,
                Rule::EOI => break,
                _ => unreachable!(),
            }
        }

        Ok(theme)
    }
}

impl ApplyConfig for AppContainerStyles {
    fn apply(&mut self, pair: Pair<'_, Rule>) -> Result<(), ConfigError> {
        for pair in pair.into_inner() {
            match pair.as_rule() {
                // Style
                Rule::background => {
                    self.background = helpers::unwrap_hex_color(pair)?;
                    self.propagate_background();
                }
                Rule::color => {
                    self.color = helpers::unwrap_hex_color(pair)?;
                    self.propagate_color();
                }
                Rule::border_color => self.border_color = helpers::unwrap_hex_color(pair)?,
                Rule::border_radius => self.border_radius = helpers::unwrap_attr_f32(pair)?,
                Rule::border_width => self.border_width = helpers::unwrap_attr_f32(pair)?,
                // Padding
                Rule::padding => {
                    self.padding = OnagrePadding::from(helpers::unwrap_attr_u16(pair)?)
                }
                Rule::padding_top => self.padding.top = helpers::unwrap_attr_u16(pair)?,
                Rule::padding_bottom => self.padding.bottom = helpers::unwrap_attr_u16(pair)?,
                Rule::padding_right => self.padding.right = helpers::unwrap_attr_u16(pair)?,
                Rule::padding_left => self.padding.left = helpers::unwrap_attr_u16(pair)?,
                // Children
                Rule::search => self.search.apply(pair)?,
                Rule::rows => self.rows.apply(pair)?,
                Rule::scrollable => self.scrollable.apply(pair)?,
                _ => unreachable!(),
            }
        }

        Ok(())
    }
}

impl ApplyConfig for ScrollerStyles {
    fn apply(&mut self, pair: Pair<'_, Rule>) -> Result<(), ConfigError> {
        for pair in pair.into_inner() {
            match pair.as_rule() {
                // Style
                Rule::background => self.background = helpers::unwrap_hex_color(pair)?,
                Rule::border_color => self.border_color = helpers::unwrap_hex_color(pair)?,
                Rule::border_radius => self.border_radius = helpers::unwrap_attr_f32(pair)?,
                Rule::border_width => self.border_width = helpers::unwrap_attr_f32(pair)?,
                Rule::scrollbar_margin => self.scrollbar_margin = helpers::unwrap_attr_u16(pair)?,
                Rule::scrollbar_width => self.scrollbar_width = helpers::unwrap_attr_u16(pair)?,
                Rule::scroller => {
                    for pair in pair.into_inner() {
                        match pair.as_rule() {
                            Rule::color => self.scroller_color = helpers::unwrap_hex_color(pair)?,
                            Rule::border_color => {
                                self.scroller_border_color = helpers::unwrap_hex_color(pair)?
                            }
                            Rule::border_width => {
                                self.scroller_border_width = helpers::unwrap_attr_f32(pair)?
                            }
                            Rule::border_radius => {
                                self.scroller_border_radius = helpers::unwrap_attr_f32(pair)?
                            }
                            Rule::scroller_width => {
                                self.scroller_width = helpers::unwrap_attr_u16(pair)?
                            }
                            _ => unreachable!(),
                        }
                    }
                }
                _ => unreachable!(),
            }
        }

        Ok(())
    }
}

impl ApplyConfig for SearchContainerStyles {
    fn apply(&mut self, pair: Pair<'_, Rule>) -> Result<(), ConfigError> {
        for pair in pair.into_inner() {
            match pair.as_rule() {
                // Style
                Rule::background => {
                    self.background = helpers::unwrap_hex_color(pair)?;
                    self.propagate_background();
                }
                Rule::color => {
                    self.color = helpers::unwrap_hex_color(pair)?;
                    self.propagate_color();
                }
                Rule::border_color => self.border_color = helpers::unwrap_hex_color(pair)?,
                Rule::border_radius => self.border_radius = helpers::unwrap_attr_f32(pair)?,
                Rule::border_width => self.border_width = helpers::unwrap_attr_f32(pair)?,

                // Layout
                Rule::padding => {
                    self.padding = OnagrePadding::from(helpers::unwrap_attr_u16(pair)?)
                }
                Rule::padding_top => self.padding.top = helpers::unwrap_attr_u16(pair)?,
                Rule::padding_bottom => self.padding.bottom = helpers::unwrap_attr_u16(pair)?,
                Rule::padding_right => self.padding.right = helpers::unwrap_attr_u16(pair)?,
                Rule::padding_left => self.padding.left = helpers::unwrap_attr_u16(pair)?,
                Rule::width => self.width = helpers::unwrap_length(pair)?,
                Rule::height => self.height = helpers::unwrap_length(pair)?,
                Rule::align_x => self.align_x = helpers::unwrap_x(pair)?,
                Rule::align_y => self.align_y = helpers::unwrap_y(pair)?,

                // Children
                Rule::bar => self.input.apply(pair)?,
                Rule::plugin_hint => {
                    self.plugin_hint = {
                        let mut hint = GenericContainerStyle::default();
                        hint.apply(pair)?;
                        Some(hint)
                    }
                }

                _ => unreachable!(),
            }
        }

        Ok(())
    }
}

impl ApplyConfig for SearchInputStyles {
    fn apply(&mut self, pair: Pair<'_, Rule>) -> Result<(), ConfigError> {
        for pair in pair.into_inner() {
            match pair.as_rule() {
                // Style
                Rule::background => self.background = helpers::unwrap_hex_color(pair)?,
                Rule::color => self.value_color = helpers::unwrap_hex_color(pair)?,
                Rule::border_color => self.border_color = helpers::unwrap_hex_color(pair)?,
                Rule::border_radius => self.border_radius = helpers::unwrap_attr_f32(pair)?,
                Rule::border_width => self.border_width = helpers::unwrap_attr_f32(pair)?,
                Rule::text_width => self.text_width = helpers::unwrap_length(pair)?,
                Rule::selection_color => self.selection_color = helpers::unwrap_hex_color(pair)?,
                Rule::placeholder_color => {
                    self.placeholder_color = helpers::unwrap_hex_color(pair)?
                }
                Rule::font_size => self.size = helpers::unwrap_attr_u16(pair)?,

                // Layout
                Rule::padding => {
                    self.padding = OnagrePadding::from(helpers::unwrap_attr_u16(pair)?)
                }
                Rule::padding_top => self.padding.top = helpers::unwrap_attr_u16(pair)?,
                Rule::padding_bottom => self.padding.bottom = helpers::unwrap_attr_u16(pair)?,
                Rule::padding_right => self.padding.right = helpers::unwrap_attr_u16(pair)?,
                Rule::padding_left => self.padding.left = helpers::unwrap_attr_u16(pair)?,
                Rule::width => self.width = helpers::unwrap_length(pair)?,
                Rule::height => self.height = helpers::unwrap_length(pair)?,
                Rule::align_x => self.align_x = helpers::unwrap_x(pair)?,
                Rule::align_y => self.align_y = helpers::unwrap_y(pair)?,
                _ => unreachable!(),
            }
        }

        Ok(())
    }
}

impl ApplyConfig for RowContainerStyle {
    fn apply(&mut self, pair: Pair<'_, Rule>) -> Result<(), ConfigError> {
        for pair in pair.into_inner() {
            match pair.as_rule() {
                // Style
                Rule::color => {
                    self.color = helpers::unwrap_hex_color(pair)?;
                    self.propagate_color();
                }
                Rule::background => {
                    self.background = helpers::unwrap_hex_color(pair)?;
                    self.propagate_color();
                }
                Rule::border_color => self.border_color = helpers::unwrap_hex_color(pair)?,
                Rule::border_radius => self.border_radius = helpers::unwrap_attr_f32(pair)?,
                Rule::border_width => self.border_width = helpers::unwrap_attr_f32(pair)?,

                // Padding
                Rule::padding => {
                    self.padding = OnagrePadding::from(helpers::unwrap_attr_u16(pair)?)
                }
                Rule::padding_top => self.padding.top = helpers::unwrap_attr_u16(pair)?,
                Rule::padding_bottom => self.padding.bottom = helpers::unwrap_attr_u16(pair)?,
                Rule::padding_right => self.padding.right = helpers::unwrap_attr_u16(pair)?,
                Rule::padding_left => self.padding.left = helpers::unwrap_attr_u16(pair)?,
                Rule::width => self.width = helpers::unwrap_length(pair)?,
                Rule::height => self.height = helpers::unwrap_length(pair)?,

                // Children
                Rule::default_row => self.row.apply(pair)?,
                Rule::selected_row => self.row_selected.apply(pair)?,
                _ => unreachable!(),
            }
        }

        Ok(())
    }
}

impl ApplyConfig for RowStyles {
    // We need this to inherit row default attribute
    fn apply(&mut self, pair: Pair<'_, Rule>) -> Result<(), ConfigError> {
        for pair in pair.into_inner() {
            match pair.as_rule() {
                // Iced style
                Rule::background => {
                    self.background = helpers::unwrap_hex_color(pair)?;
                    self.propagate_background();
                }
                Rule::border_color => {
                    self.border_color = helpers::unwrap_hex_color(pair)?;
                    self.propagate_color();
                }
                Rule::border_radius => self.border_radius = helpers::unwrap_attr_f32(pair)?,
                Rule::border_width => self.border_width = helpers::unwrap_attr_f32(pair)?,
                Rule::color => self.color = helpers::unwrap_hex_color(pair)?,

                // Iced Layout
                Rule::padding => {
                    self.padding = OnagrePadding::from(helpers::unwrap_attr_u16(pair)?)
                }
                Rule::padding_top => self.padding.top = helpers::unwrap_attr_u16(pair)?,
                Rule::padding_bottom => self.padding.bottom = helpers::unwrap_attr_u16(pair)?,
                Rule::padding_right => self.padding.right = helpers::unwrap_attr_u16(pair)?,
                Rule::padding_left => self.padding.left = helpers::unwrap_attr_u16(pair)?,
                Rule::width => self.width = helpers::unwrap_length(pair)?,
                Rule::height => self.height = helpers::unwrap_length(pair)?,
                Rule::align_x => self.align_x = helpers::unwrap_x(pair)?,
                Rule::align_y => self.align_y = helpers::unwrap_y(pair)?,

                // Children
                Rule::description_row => {
                    self.hide_description = false;
                    self.description.apply(pair)?
                }
                Rule::title_row => self.title.apply(pair)?,
                Rule::icon => self.icon.apply(pair)?,
                _ => unreachable!(),
            }
        }

        Ok(())
    }
}

impl ApplyConfig for GenericContainerStyle {
    fn apply(&mut self, pair: Pair<Rule>) -> Result<(), ConfigError> {
        for pair in pair.into_inner() {
            match pair.as_rule() {
                // Iced style
                Rule::background => self.background = helpers::unwrap_hex_color(pair)?,
                Rule::border_radius => self.border_radius = helpers::unwrap_attr_f32(pair)?,
                Rule::border_width => self.border_width = helpers::unwrap_attr_f32(pair)?,
                Rule::border_color => self.border_color = helpers::unwrap_hex_color(pair)?,
                Rule::color => self.color = helpers::unwrap_hex_color(pair)?,
                // Iced Layout
                Rule::padding => {
                    self.padding = OnagrePadding::from(helpers::unwrap_attr_u16(pair)?)
                }
                Rule::padding_top => self.padding.top = helpers::unwrap_attr_u16(pair)?,
                Rule::padding_bottom => self.padding.bottom = helpers::unwrap_attr_u16(pair)?,
                Rule::padding_right => self.padding.right = helpers::unwrap_attr_u16(pair)?,
                Rule::padding_left => self.padding.left = helpers::unwrap_attr_u16(pair)?,
                Rule::align_x => self.align_x = helpers::unwrap_x(pair)?,
                Rule::align_y => self.align_y = helpers::unwrap_y(pair)?,
                Rule::width => self.width = helpers::unwrap_length(pair)?,
                Rule::height => self.height = helpers::unwrap_length(pair)?,
                Rule::font_size => self.font_size = helpers::unwrap_attr_u16(pair)?,
                _ => unreachable!(),
            }
        }

        Ok(())
    }
}

impl ApplyConfig for IconStyle {
    fn apply(&mut self, pair: Pair<Rule>) -> Result<(), ConfigError> {
        for pair in pair.into_inner() {
            match pair.as_rule() {
                // Iced style
                Rule::background => self.background = helpers::unwrap_hex_color(pair)?,
                Rule::border_radius => self.border_radius = helpers::unwrap_attr_f32(pair)?,
                Rule::border_width => self.border_width = helpers::unwrap_attr_f32(pair)?,
                Rule::border_color => self.border_color = helpers::unwrap_hex_color(pair)?,
                Rule::color => self.color = helpers::unwrap_hex_color(pair)?,

                // Iced Layout
                Rule::padding => {
                    self.padding = OnagrePadding::from(helpers::unwrap_attr_u16(pair)?)
                }
                Rule::padding_top => self.padding.top = helpers::unwrap_attr_u16(pair)?,
                Rule::padding_bottom => self.padding.bottom = helpers::unwrap_attr_u16(pair)?,
                Rule::padding_right => self.padding.right = helpers::unwrap_attr_u16(pair)?,
                Rule::padding_left => self.padding.left = helpers::unwrap_attr_u16(pair)?,
                Rule::width => self.width = helpers::unwrap_length(pair)?,
                Rule::height => self.height = helpers::unwrap_length(pair)?,
                Rule::align_x => self.align_x = helpers::unwrap_x(pair)?,
                Rule::align_y => self.align_y = helpers::unwrap_y(pair)?,
                Rule::icon_size => self.size = helpers::unwrap_attr_u16(pair)?,
                _ => unreachable!(),
            }
        }

        Ok(())
    }
}

impl Theme {
    fn base() -> Self {
        Theme {
            icon_theme: None,
            app_container: AppContainerStyles {
                rows: RowContainerStyle {
                    row: RowStyles {
                        hide_description: true,
                        ..Default::default()
                    },
                    row_selected: RowStyles {
                        hide_description: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod test {
    use crate::app::style::Theme;
    use crate::config::parse_file;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_parse_theme_config() {
        let theme = parse_file("tests/theme.scss");

        if let Err(err) = &theme {
            println!("{err}")
        }

        let theme = theme.unwrap();

        assert_eq!(theme, Theme::default());
    }
}
