use crate::style::layout::Length;
use crate::style::color::OnagreColor;
use serde::de::{Deserialize, Deserializer};
use serde::{Serialize, Serializer};
use std::convert::TryFrom;

impl<'de> Deserialize<'de> for OnagreColor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let str_hex_color = String::deserialize(deserializer)?;
        OnagreColor::from(&str_hex_color).map_err(serde::de::Error::custom)
    }
}

impl Serialize for OnagreColor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Length {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Length::try_from(value).map_err(serde::de::Error::custom)
    }
}

impl Serialize for Length {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
