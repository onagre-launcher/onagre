use crate::style::theme::Theme;
use crate::THEME_PATH;
use anyhow::Result;
use config::{Config, File};

impl Theme {
    /// Resolve onagre theme settings against its standard xdg path :
    /// `$XDG_CONFIG_DIR/onagre/style.toml`
    pub fn get() -> Result<Self> {
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
