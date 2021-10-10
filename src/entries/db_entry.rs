use crate::db::entity::DesktopEntryEntity;
use crate::entries::AsEntry;
use crate::freedesktop::IconPath;

impl<'a> AsEntry<'a> for DesktopEntryEntity {
    fn get_display_name(&self) -> &str {
        self.name.as_str()
    }

    fn get_icon(&self) -> Option<IconPath> {
        IconPath::from_path(&self.icon)
    }
}
