use std::fmt::Debug;

use onagre_launcher_toolkit::launcher::IconSource;

use crate::db::{desktop_entry::DesktopEntryEntity, plugin::PluginCommandEntity, web::WebEntity};

use super::pop_entry::PopSearchResult;

pub trait Entry2
where
    Self: 'static + Debug,
{
    fn get_display_name(&self) -> &str;
    fn get_icon(&self) -> Option<IconSource>;
    fn get_description(&self) -> Option<String>;
}

impl Entry2 for PopSearchResult {
    fn get_display_name(&self) -> &str {
        &self.0.name
    }

    fn get_icon(&self) -> Option<IconSource> {
        self.0.icon.clone()
    }

    fn get_description(&self) -> Option<String> {
        Some(self.0.description.clone())
    }
}
impl Entry2 for DesktopEntryEntity<'static> {
    fn get_display_name(&self) -> &str {
        &self.name
    }

    fn get_icon(&self) -> Option<IconSource> {
        self.icon.as_ref().map(|i| IconSource::Name(i.clone()))
    }

    fn get_description(&self) -> Option<String> {
        self.description.as_ref().map(|d| d.to_string())
    }
}
impl Entry2 for PluginCommandEntity<'static> {
    fn get_display_name(&self) -> &str {
        self.query.as_ref()
    }

    fn get_icon(&self) -> Option<IconSource> {
        None
    }

    fn get_description(&self) -> Option<String> {
        None
    }
}

impl Entry2 for WebEntity<'static> {
    fn get_display_name(&self) -> &str {
        self.query.as_ref()
    }

    fn get_icon(&self) -> Option<IconSource> {
        None
    }

    fn get_description(&self) -> Option<String> {
        None
    }
}
