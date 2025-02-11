use std::borrow::Cow;

use onagre_launcher_toolkit::launcher::IconSource;

use crate::{db::desktop_entry::DesktopEntryEntity, freedesktop::desktop::DesktopEntry};

pub trait Entry2
where
    Self: 'static,
{
    fn get_display_name(&self) -> &str;
    fn get_icon(&self) -> Option<IconSource>;
    fn get_description(&self) -> Option<&str>;
}

impl Entry2 for DesktopEntryEntity<'static> {
    fn get_display_name(&self) -> &str {
        &self.name
    }

    fn get_icon(&self) -> Option<IconSource> {
        self.icon.as_ref().map(|i| IconSource::Name(i.clone()))
    }

    fn get_description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}
