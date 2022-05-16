use std::borrow::Cow;
use pop_launcher_toolkit::launcher::{IconSource, SearchResult};

use crate::entries::AsEntry;
use crate::freedesktop::IconPath;
use crate::THEME;

pub struct PopSearchResult<'a>(pub &'a SearchResult);

impl<'a> AsEntry<'a> for PopSearchResult<'a> {
    fn get_display_name(&self) -> &str {
        self.0.name.as_str()
    }

    fn get_icon(&self) -> Option<IconPath> {
        match &THEME.icon_theme {
            Some(theme) => self.0.icon.as_ref().or(self.0.category_icon.as_ref())
                .and_then(|source| match source {
                    IconSource::Name(name) => IconPath::lookup(name, &theme, THEME.icon_size),
                    IconSource::Mime(_) => None, // TODO: MRS Mimes
                }),
            _ => None,
        }
    }

    fn get_description(&self) -> Option<Cow<'_, str>> {
        Some(Cow::Borrowed(self.0.description.as_str()))
    }
}
