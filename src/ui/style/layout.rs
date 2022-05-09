use anyhow::anyhow;
use iced::Length as Icedlength;
use std::convert::TryFrom;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Length(Icedlength);

impl Eq for Length {}

impl TryFrom<String> for Length {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = value.trim();

        match value {
            "fill" => Ok(Length(Icedlength::Fill)),
            "shrink" => Ok(Length(Icedlength::Shrink)),
            portion if portion.starts_with("flex") => {
                let value = portion[4..].trim().parse::<u16>().map_err(|err| {
                    anyhow!(
                        "Expected 'fill', 'shrink', 'flex {{value}}', or {{value}}, got {} :\n{}",
                        portion,
                        err
                    )
                })?;
                Ok(Length(Icedlength::FillPortion(value)))
            }
            raw => {
                let value = raw.parse::<u16>().map_err(|err| {
                    anyhow!(
                        "Expected 'fill', 'shrink', 'flex-{{value}}', or {{value}}, got {} :\n{}",
                        raw,
                        err
                    )
                })?;

                Ok(Length(Icedlength::Units(value)))
            }
        }
    }
}

impl Length {
    pub fn fill() -> Self {
        Self(Icedlength::Fill)
    }

    pub fn shrink() -> Self {
        Self(Icedlength::Shrink)
    }

    pub fn flex(value: u16) -> Self {
        Self(Icedlength::FillPortion(value))
    }

    pub fn raw(value: u16) -> Self {
        Self(Icedlength::Units(value))
    }
}

impl ToString for Length {
    fn to_string(&self) -> String {
        match self.0 {
            Icedlength::Fill => "fill".into(),
            Icedlength::Shrink => "shrink".into(),
            Icedlength::Units(raw) => raw.to_string(),
            Icedlength::FillPortion(portion) => format!("flex {}", portion),
        }
    }
}

impl From<Length> for Icedlength {
    fn from(length: Length) -> Self {
        length.0
    }
}

#[cfg(test)]
mod test {
    use crate::ui::style::layout::Length;
    use iced::Length as Icedlength;
    use std::convert::TryFrom;

    #[test]
    fn length_fill() {
        let string_value = "fill".to_string();

        let length = Length::try_from(string_value);

        assert_eq!(length.unwrap(), Length(Icedlength::Fill));
    }

    #[test]
    fn length_shrink() {
        let string_value = "shrink".to_string();

        let length = Length::try_from(string_value);

        assert_eq!(length.unwrap(), Length(Icedlength::Shrink));
    }

    #[test]
    fn length_flex() {
        let string_value = "flex  3 ".to_string();

        let length = Length::try_from(string_value);

        assert_eq!(length.unwrap(), Length(Icedlength::FillPortion(3)));
    }

    #[test]
    fn length_raw() {
        let string_value = " 2 ".to_string();

        let length = Length::try_from(string_value);

        assert_eq!(length.unwrap(), Length(Icedlength::Units(2)));
    }

    #[test]
    fn length_flex_err() {
        let string_value = "flex err".to_string();

        let length = Length::try_from(string_value);

        assert!(length.is_err());
    }

    #[test]
    fn length_raw_err() {
        let string_value = " invalid ".to_string();

        let length = Length::try_from(string_value);

        assert!(length.is_err());
    }
}
