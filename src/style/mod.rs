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
        let r = if let Some(red) = hex_color.get(1..3) {
            OnagreColor::f32_from_str_hex(&red)?
        } else {
            0.0
        };

        let g = if let Some(green) = hex_color.get(3..5) {
            OnagreColor::f32_from_str_hex(green)?
        } else {
            0.0
        };

        let b = if let Some(blue) = hex_color.get(5..7) {
            OnagreColor::f32_from_str_hex(blue)?
        } else {
            0.0
        };

        let a = if let Some(opacity) = hex_color.get(7..9) {
            OnagreColor::f32_from_str_hex(opacity)?
        } else {
            1.0
        };

        Ok(OnagreColor {
            color: Color { r, g, b, a },
        })
    }

    fn f32_from_str_hex(hex_color: &str) -> Result<f32> {
        u32::from_str_radix(hex_color, 16)
            .map_err(|err| anyhow!("{} Is not a valid hex color : {}", hex_color, err))
            .map(|value| value as f32 / 255.0)
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
    fn should_get_color_from_with_opacity() {
        let hex_color = "#CCFF00CC";

        let color = OnagreColor::from(hex_color);

        assert_eq!(
            OnagreColor {
                color: Color {
                    r: 0.8,
                    g: 1.0,
                    b: 0.0,
                    a: 0.8,
                }
            },
            color.unwrap()
        );
    }

    #[test]
    fn should_get_red_value_and_default() {
        let hex_color = "#CC";

        let color = OnagreColor::from(hex_color);

        assert_eq!(
            OnagreColor {
                color: Color {
                    r: 0.8,
                    g: 0.0,
                    b: 0.0,
                    a: 1.0,
                }
            },
            color.unwrap()
        );
    }

    #[test]
    fn parse_error() {
        let hex_color = "#II";

        let color = OnagreColor::from(hex_color);

        assert!(color.is_err());
    }
}
