use iced_style::container;
use iced::Background;
use crate::config::color::OnagreColor;
use crate::config::padding::OnagrePadding;
use crate::ui::style::rows::generic::GenericContainerStyle;
use crate::ui::style::search::input::SearchInputStyles;
use crate::ui::style::search::SearchContainerStyles;
use crate::ui::style::app::AppContainerStyles;
use crate::ui::style::scrollable::scroller::ScrollerStyles;

pub mod font;
pub mod scrollable;
pub mod search;
pub mod app;
pub mod rows;

impl Theme {
    pub fn load() -> Self {
        let buf = dirs::config_dir()
            .expect("Failed to open `$XDG_CONFIG_HOME`")
            .join("onagre")
            .join("theme.scss");

        let theme = crate::config::parse_file(buf);
        if let Err(err) = &theme {
            eprintln!("Failed to parse theme:");
            eprintln!("{err}");
            eprintln!("Failing back to default theme");
        };

        theme.unwrap_or(Theme::default())
    }
}

impl AsRef<Theme> for Theme {
    fn as_ref(&self) -> &Theme {
        self
    }
}

#[derive(Debug, PartialEq)]
pub struct Theme {
    // Layout
    pub exit_unfocused: bool,
    pub size: (u32, u32),
    pub font: Option<String>,
    pub font_size: u16,
    pub icon_theme: Option<String>,
    pub icon_size: u16,
    pub padding: OnagrePadding,

    // Style
    pub background: OnagreColor,
    pub color: OnagreColor,
    pub border_color: OnagreColor,
    pub border_radius: f32,
    pub border_width: f32,

    // Children
    pub app_container: AppContainerStyles
}

impl Theme {
    pub fn search(&self) -> &SearchContainerStyles {
        &self.app_container.search
    }

    pub fn search_input(&self) -> &SearchInputStyles {
        &self.app_container.search.input
    }

    pub fn plugin_hint(&self) -> Option<&GenericContainerStyle> {
        self.app_container.search.plugin_hint.as_ref()
    }

    pub fn scrollable(&self) -> &ScrollerStyles {
        &self.app_container.scrollable
    }

    pub fn app(&self) -> &AppContainerStyles {
        &self.app_container
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            exit_unfocused: false,
            size: (800, 300),
            font: None,
            font_size: 22,
            icon_theme: Some("Papirus".to_string()),
            icon_size: 24,
            background: OnagreColor::DEFAULT_BACKGROUND,
            color: OnagreColor::DEFAULT_BACKGROUND,
            border_color: OnagreColor::from("#00000000").unwrap(),
            border_radius: 0.0,
            border_width: 0.0,
            padding: OnagrePadding::from(50),
            app_container: AppContainerStyles::default(),
        }
    }
}

impl container::StyleSheet for &Theme {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(self.background.into())),
            border_radius: self.border_radius,
            border_width: self.border_width,
            text_color: Some(self.color.into()),
            border_color: self.border_color.into(),
        }
    }
}
