use crate::ui::style::color::OnagreColor;
use crate::ui::style::layout::Length;
use crate::ui::style::theme::Theme;
use crate::THEME_PATH;
use anyhow::anyhow;
use config::{Config, File};
use serde::de::{Deserialize, Deserializer};
use serde::{Serialize, Serializer};
use std::convert::TryFrom;

impl Theme {
    /// Resolve onagre theme settings against its standard xdg path :
    /// `$XDG_CONFIG_DIR/onagre/style.toml`
    pub fn get() -> anyhow::Result<Self> {
        let theme_path = THEME_PATH.lock().unwrap();
        if theme_path.exists() {
            let mut s = Config::new();
            s.merge(File::from(theme_path.as_path()))?;
            s.try_into()
                .map_err(|err| anyhow!("{} : {}", "Theme format error", err))
        } else {
            Err(anyhow!(
                "Unable to find theme settings file {}",
                &theme_path.display()
            ))
        }
    }
}

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
