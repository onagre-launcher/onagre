use crate::style::color::OnagreColor;
use crate::style::rows::RowContainerStyles;
use crate::style::scrollable::ScrollableStyles;
use crate::style::search::SearchContainerStyles;
use iced::container;
use iced_native::Background;

#[derive(Deserialize, Serialize, Debug)]
#[serde(default)]
pub struct Theme {
    pub background: OnagreColor,
    pub foreground: OnagreColor,
    pub border_color: OnagreColor,
    pub border_radius: f32,
    pub border_width: f32,
    pub rows: RowContainerStyles,
    pub scrollable: ScrollableStyles,
    pub search: SearchContainerStyles,
    pub menu: RowContainerStyles,
}

impl Theme {
    pub fn load() -> Self {
        if let Ok(theme) = Theme::get() {
            theme
        } else {
            Theme::default()
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            background: OnagreColor::DEFAULT_BACKGROUND,
            foreground: OnagreColor::DEFAULT_BACKGROUND,
            border_color: OnagreColor::from("#00000000").unwrap(),
            border_radius: 0.0,
            border_width: 0.0,
            rows: RowContainerStyles::default(),
            scrollable: ScrollableStyles::default(),
            search: SearchContainerStyles::default(),
            menu: RowContainerStyles::mode_entries(),
        }
    }
}

impl container::StyleSheet for &Theme {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(self.background.into())),
            border_radius: self.border_radius,
            border_width: self.border_width,
            text_color: Some(self.foreground.into()),
            border_color: self.border_color.into(),
        }
    }
}

impl AsRef<Theme> for Theme {
    fn as_ref(&self) -> &Theme {
        self
    }
}
