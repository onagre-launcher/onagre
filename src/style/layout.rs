use iced::Length as IcedLenght;
use std::convert::TryFrom;

#[derive(Debug, PartialEq)]
pub struct Lenght(IcedLenght);

impl TryFrom<String> for Lenght {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = value.trim();

        match value {
            "fill" => Ok(Lenght(IcedLenght::Fill)),
            "shrink" => Ok(Lenght(IcedLenght::Shrink)),
            portion if portion.starts_with("flex") => {
                let value = portion[4..].trim().parse::<u16>().map_err(|err| {
                    anyhow!(
                        "Expected 'fill', 'shrink', 'flex-{{value}}', or {{value}}, got {} :\n{}",
                        portion,
                        err
                    )
                })?;
                Ok(Lenght(IcedLenght::FillPortion(value)))
            }
            raw => {
                let value = raw.parse::<u16>().map_err(|err| {
                    anyhow!(
                        "Expected 'fill', 'shrink', 'flex-{{value}}', or {{value}}, got {} :\n{}",
                        raw,
                        err
                    )
                })?;

                Ok(Lenght(IcedLenght::Units(value)))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::style::layout::Lenght;
    use iced::Length as IcedLenght;
    use std::convert::TryFrom;

    #[test]
    fn lenght_fill() {
        let string_value = "fill".to_string();

        let lenght = Lenght::try_from(string_value);

        assert_eq!(lenght.unwrap(), Lenght(IcedLenght::Fill));
    }

    #[test]
    fn lenght_shrink() {
        let string_value = "shrink".to_string();

        let lenght = Lenght::try_from(string_value);

        assert_eq!(lenght.unwrap(), Lenght(IcedLenght::Shrink));
    }

    #[test]
    fn lenght_flex() {
        let string_value = "flex  3 ".to_string();

        let lenght = Lenght::try_from(string_value);

        assert_eq!(lenght.unwrap(), Lenght(IcedLenght::FillPortion(3)));
    }

    #[test]
    fn lenght_raw() {
        let string_value = " 2 ".to_string();

        let lenght = Lenght::try_from(string_value);

        assert_eq!(lenght.unwrap(), Lenght(IcedLenght::Units(2)));
    }

    #[test]
    fn lenght_flex_err() {
        let string_value = "flex err".to_string();

        let lenght = Lenght::try_from(string_value);

        assert!(lenght.is_err());
    }

    #[test]
    fn lenght_raw_err() {
        let string_value = " invalid ".to_string();

        let lenght = Lenght::try_from(string_value);

        assert!(lenght.is_err());
    }
}
