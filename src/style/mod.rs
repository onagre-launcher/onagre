pub mod color_de;
pub mod rows;
pub mod scrollable;
pub mod text_input;
pub mod theme;
pub mod theme_settings;

use anyhow::Result;
use iced::Color;

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct OnagreColor {
    color: Color,
}

impl Eq for OnagreColor {}

impl OnagreColor {
    const BLACK: OnagreColor = OnagreColor {
        color: Color::BLACK,
    };

    const WHITE: OnagreColor = OnagreColor {
        color: Color::WHITE,
    };

    fn from(hex_color: &str) -> Result<Self> {
        if hex_color.len() != 7 {
            return Err(anyhow!("expected valid hex color, got `{}`", hex_color));
        }

        let r = u32::from_str_radix(&hex_color[1..3], 16)? as f32 / 255.0;
        let g = u32::from_str_radix(&hex_color[3..5], 16)? as f32 / 255.0;
        let b = u32::from_str_radix(&hex_color[5..7], 16)? as f32 / 255.0;

        Ok(OnagreColor {
            color: Color { r, g, b, a: 1.0 },
        })
    }
}

impl ToString for OnagreColor {
    fn to_string(&self) -> String {
        let r = (self.color.r * 255.0) as u32;
        let g = (self.color.g * 255.0) as u32;
        let b = (self.color.b * 255.0) as u32;

        let r = to_lower_gex_with_leading_zero(r);
        let g = to_lower_gex_with_leading_zero(g);
        let b = to_lower_gex_with_leading_zero(b);
        format!("#{}{}{}", r, g, b)
    }
}

fn to_lower_gex_with_leading_zero(value: u32) -> String {
    let val = format!("{:x}", value);
    if val.len() == 1 {
        format!("0{}", val)
    } else {
        val
    }
}

impl Into<Color> for OnagreColor {
    fn into(self) -> Color {
        self.color
    }
}

impl Default for OnagreColor {
    fn default() -> Self {
        Self {
            color: Default::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::style::OnagreColor;
    use iced_style::Color;

    #[test]
    fn should_get_color_from_str() {
        let hex_color = "#ff00ff";

        let color = OnagreColor::from(hex_color);

        assert_eq!(
            OnagreColor {
                color: Color {
                    r: 1.0,
                    g: 0.0,
                    b: 1.0,
                    a: 1.0,
                }
            },
            color.unwrap()
        );
    }

    #[test]
    fn should_get_color_from_str2() {
        let hex_color = "#CCFF00";

        let color = OnagreColor::from(hex_color);

        assert_eq!(
            OnagreColor {
                color: Color {
                    r: 0.8,
                    g: 1.0,
                    b: 0.0,
                    a: 1.0,
                }
            },
            color.unwrap()
        );
    }
}
