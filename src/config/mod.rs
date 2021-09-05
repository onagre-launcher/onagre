use crate::SETTINGS_PATH;
use anyhow::Result;
use config::Config;
use config::File;
use std::collections::HashMap;

pub mod theme_settings;

#[derive(Deserialize, Serialize, Debug)]
pub struct ModeSettings {
    pub source: Option<String>,
    pub target: String,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct OnagreSettings {
    pub icons: Option<String>,
    #[serde(default)]
    pub modes: HashMap<String, ModeSettings>,
}

impl OnagreSettings {
    /// Resolve onagre theme settings against its standard xdg path :
    /// `$XDG_CONFIG_DIR/onagre/style.toml`
    pub fn get() -> Result<Self> {
        let settings_path = SETTINGS_PATH.lock().unwrap();
        if settings_path.exists() {
            let mut s = Config::new();
            s.merge(File::from(settings_path.clone()))?;
            s.try_into()
                .map_err(|err| anyhow!("{} : {}", "Config format error", err))
        } else {
            Err(anyhow!(
                "Unable to find settings file {}",
                settings_path.display()
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ModeSettings;
    use super::OnagreSettings;
    use anyhow::Result;
    use std::collections::HashMap;

    #[test]
    fn generate_default_conf() -> Result<()> {
        use crate::style::theme::Theme;

        let settings = Theme::default();
        let settings = toml::to_string(&settings)?;
        std::fs::write(
            dirs::config_dir()
                .unwrap()
                .join("onagre")
                .join("theme.toml"),
            settings,
        )?;

        let mut modes = HashMap::new();
        let mode_xdg = ModeSettings {
            source: Some("fd . /home/okno/".to_string()),
            target: "xdg-open %".to_string(),
        };
        modes.insert("xdg".to_string(), mode_xdg);
        let settings = OnagreSettings { icons: None, modes };

        let settings = toml::to_string(&settings)?;
        std::fs::write(
            dirs::config_dir()
                .unwrap()
                .join("onagre")
                .join("config.toml"),
            settings,
        )?;

        Ok(())
    }
}
