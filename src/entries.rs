use crate::desktop::DesktopEntryInContent;
use std::rc::{Rc, Weak};

#[derive(Debug, Clone)]
pub struct DesktopEntry {
    pub name: String,
    pub exec: String,
}

#[derive(Debug, Clone)]
pub struct FileEntry {
    pub path: String,
}

#[derive(Debug, Default, Clone)]
pub struct Entries {
    pub desktop_entries: Vec<Rc<DesktopEntry>>,
    pub xdg_entries: Vec<Rc<FileEntry>>,
}

#[derive(Debug, Default, Clone)]
pub struct MatchedEntries {
    pub desktop_entries: Vec<Weak<DesktopEntry>>,
    pub xdg_entries: Vec<Weak<FileEntry>>,
}

impl From<&DesktopEntryInContent> for DesktopEntry {
    fn from(desktop_entry: &DesktopEntryInContent) -> Self {
        DesktopEntry {
            name: desktop_entry.name.clone(),
            exec: desktop_entry.exec.clone(),
        }
    }
}

