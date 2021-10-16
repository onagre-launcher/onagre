use iced::Length as IcedLenght;
use std::convert::TryFrom;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Length(IcedLenght);

impl Eq for Length {}

impl TryFrom<String> for Length {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = value.trim();

        match value {
            "fill" => Ok(Length(IcedLenght::Fill)),
            "shrink" => Ok(Length(IcedLenght::Shrink)),
            portion if portion.starts_with("flex") => {
                let value = portion[4..].trim().parse::<u16>().map_err(|err| {
                    anyhow!(
                        "Expected 'fill', 'shrink', 'flex {{value}}', or {{value}}, got {} :\n{}",
                        portion,
                        err
                    )
                })?;
                Ok(Length(IcedLenght::FillPortion(value)))
            }
            raw => {
                let value = raw.parse::<u16>().map_err(|err| {
                    anyhow!(
                        "Expected 'fill', 'shrink', 'flex-{{value}}', or {{value}}, got {} :\n{}",
                        raw,
                        err
                    )
                })?;

                Ok(Length(IcedLenght::Units(value)))
            }
        }
    }
}

impl Length {
    pub fn fill() -> Self {
        Self(IcedLenght::Fill)
    }

    pub fn shrink() -> Self {
        Self(IcedLenght::Shrink)
    }

    pub fn flex(value: u16) -> Self {
        Self(IcedLenght::FillPortion(value))
    }

    pub fn raw(value: u16) -> Self {
        Self(IcedLenght::Units(value))
    }
}

impl ToString for Length {
    fn to_string(&self) -> String {
        match self.0 {
            IcedLenght::Fill => "fill".into(),
            IcedLenght::Shrink => "shrink".into(),
            IcedLenght::Units(raw) => raw.to_string(),
            IcedLenght::FillPortion(portion) => format!("flex {}", portion),
        }
    }
}

impl From<Length> for IcedLenght {
    fn from(length: Length) -> Self {
        length.0
    }
}
#[cfg(test)]
mod test {
    use crate::style::layout::Length;
    use iced::Length as IcedLenght;
    use std::convert::TryFrom;

    #[test]
    fn lenght_fill() {
        let string_value = "fill".to_string();

        let lenght = Length::try_from(string_value);

        assert_eq!(lenght.unwrap(), Length(IcedLenght::Fill));
    }

    #[test]
    fn lenght_shrink() {
        let string_value = "shrink".to_string();

        let lenght = Length::try_from(string_value);

        assert_eq!(lenght.unwrap(), Length(IcedLenght::Shrink));
    }

    #[test]
    fn lenght_flex() {
        let string_value = "flex  3 ".to_string();

        let lenght = Length::try_from(string_value);

        assert_eq!(lenght.unwrap(), Length(IcedLenght::FillPortion(3)));
    }

    #[test]
    fn lenght_raw() {
        let string_value = " 2 ".to_string();

        let lenght = Length::try_from(string_value);

        assert_eq!(lenght.unwrap(), Length(IcedLenght::Units(2)));
    }

    #[test]
    fn lenght_flex_err() {
        let string_value = "flex err".to_string();

        let lenght = Length::try_from(string_value);

        assert!(lenght.is_err());
    }

    #[test]
    fn lenght_raw_err() {
        let string_value = " invalid ".to_string();

        let lenght = Length::try_from(string_value);

        assert!(lenght.is_err());
    }
}
