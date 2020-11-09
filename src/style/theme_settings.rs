use crate::style::color::OnagreColor;
use anyhow::Result;
use config::{Config, File};
use std::path::PathBuf;
use crate::style::scrollable::ScrollableStyles;
use crate::style::search::SearchContainerStyles;
use crate::style::rows::RowContainerStyles;

#[derive(Deserialize, Serialize, Debug)]
#[serde(default)]
pub struct Theme {
    pub background: OnagreColor,
    pub foreground: OnagreColor,
    pub border_color: OnagreColor,
    pub border_radius: u16,
    pub border_width: u16,
    pub rows: RowContainerStyles,
    pub scrollable: ScrollableStyles,
    pub search: SearchContainerStyles,
    pub menu: RowContainerStyles,
}



impl Theme {
    /// Resolve onagre theme settings against its standard xdg path :
    /// `$XDG_CONFIG_DIR/onagre/style.toml`
    pub fn get() -> Result<Self> {
        match Self::path() {
            Ok(path) => {
                if path.exists() {
                    let mut s = Config::new();
                    s.merge(File::from(path))?;
                    s.try_into()
                        .map_err(|err| anyhow!("{} : {}", "Config format error", err))
                } else {
                    Err(anyhow!(
                        "Unable to find theme settings file {}",
                        path.display()
                    ))
                }
            }
            Err(err) => Err(anyhow!("Config error : {}", err)),
        }
    }

    fn path() -> Result<PathBuf> {
        dirs::config_dir()
            .ok_or_else(|| anyhow!("Theme config not found"))
            .map(|path| path.join("onagre").join("theme.toml"))
    }
}

#[test]
fn generate_default_conf() -> Result<()> {
    let settings = Theme::default();
    let settings = toml::to_string(&settings)?;
    std::fs::write(
        dirs::config_dir()
            .unwrap()
            .join("onagre")
            .join("theme.toml"),
        settings,
    )?;
    Ok(())
}
