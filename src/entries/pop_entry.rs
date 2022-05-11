use pop_launcher_toolkit::launcher::SearchResult;

use crate::entries::AsEntry;
use crate::freedesktop::IconPath;

pub struct PopSearchResult<'a>(pub &'a SearchResult);

impl<'a> AsEntry<'a> for PopSearchResult<'a> {
    fn get_display_name(&self) -> &str {
        self.0.name.as_str()
    }

    fn get_icon(&self) -> Option<IconPath> {
        let source = self.0.icon.as_ref().or(self.0.category_icon.as_ref());

        IconPath::from_icon_source(source)
    }
}
