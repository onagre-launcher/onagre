use onagre_launcher_toolkit::launcher::SearchResult;
use std::borrow::Cow;

use crate::app::entries::AsEntry;
use crate::icons::IconPath;
use crate::THEME;

pub struct PopSearchResult<'a>(pub &'a SearchResult);

impl<'a> AsEntry<'a> for PopSearchResult<'a> {
    fn get_display_name(&self) -> &str {
        self.0.name.as_str()
    }

    fn get_icon(&self) -> Option<IconPath> {
        match &THEME.icon_theme {
            Some(theme) => self
                .0
                .icon
                .as_ref()
                .and_then(|source| IconPath::from_source(source, theme)),
            _ => None,
        }
    }

    fn get_description(&self) -> Option<Cow<'_, str>> {
        Some(Cow::Borrowed(self.0.description.as_str()))
    }
}
