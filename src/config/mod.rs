use crate::{Theme, THEME_PATH};
use config::{Config, File};

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
        let settings = OnagreSettings { modes };

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

impl Theme {
    /// Resolve onagre theme settings against its standard xdg path :
    /// `$XDG_CONFIG_DIR/onagre/style.toml`
    pub fn get() -> anyhow::Result<Self> {
        let theme_path = THEME_PATH.lock().unwrap();
        if theme_path.exists() {
            let mut s = Config::new();
            s.merge(File::from(theme_path.clone()))?;
            s.try_into()
                .map_err(|err| anyhow!("{} : {}", "Theme format error", err))
        } else {
            Err(anyhow!(
                "Unable to find theme settings file {}",
                theme_path.display()
            ))
        }
    }
}
