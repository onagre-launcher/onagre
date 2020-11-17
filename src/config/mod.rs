use anyhow::Result;
use config::Config;
use config::File;
use std::collections::HashMap;
use std::path::PathBuf;

pub mod theme_settings;

#[derive(Deserialize, Serialize, Debug)]
pub struct ModeSettings {
    pub source: String,
    pub target: String,
}

#[derive(Deserialize, Serialize, Default)]
pub struct OnagreSettings {
    pub(crate) icons: Option<String>,
    pub modes: HashMap<String, ModeSettings>,
}

impl OnagreSettings {
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
                    Err(anyhow!("Unable to find settings file {}", path.display()))
                }
            }
            Err(err) => Err(anyhow!("Config error : {}", err)),
        }
    }

    fn path() -> Result<PathBuf> {
        dirs::config_dir()
            .ok_or_else(|| anyhow!("Config not found"))
            .map(|path| path.join("onagre").join("config.toml"))
    }
}

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
        source: "fd . /home/okno/".to_string(),
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
