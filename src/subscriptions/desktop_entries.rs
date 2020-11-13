use crate::entries::desktop::DesktopEntry;
use crate::freedesktop::desktop::DesktopEntryIni;
use crate::freedesktop::icons::IconFinder;
use crate::SETTINGS;
use futures::channel::mpsc::Sender;
use glob::glob;
use iced_native::futures::stream::BoxStream;
use iced_native::Subscription;
use std::borrow::Borrow;
use std::hash::Hash;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

pub struct DesktopEntryWalker {
    id: String,
}

impl DesktopEntryWalker {
    pub fn subscription() -> Subscription<DesktopEntry> {
        iced::Subscription::from_recipe(DesktopEntryWalker {
            id: "file_walker_subscription".to_string(),
        })
    }
}

impl<H, I> iced_native::subscription::Recipe<H, I> for DesktopEntryWalker
where
    H: std::hash::Hasher,
{
    type Output = DesktopEntry;

    fn hash(&self, state: &mut H) {
        std::any::TypeId::of::<Self>().hash(state);
        self.id.hash(state)
    }

    fn stream(self: Box<Self>, _: BoxStream<I>) -> BoxStream<Self::Output> {
        let (sender, receiver) = futures::channel::mpsc::channel(100000);

        // Spawn the file reader
        async_std::task::spawn(async {
            let finder = Arc::new(
                SETTINGS
                    .icons
                    .as_ref()
                    .map(|theme_name| IconFinder::build(theme_name).ok())
                    .flatten(),
            );

            let entry_map = Arc::new(RwLock::new(Vec::new()));
            futures::future::join(
                get_root_desktop_entries(
                    sender.clone(),
                    Arc::clone(&finder),
                    Arc::clone(&entry_map),
                ),
                get_user_desktop_entries(sender, finder, entry_map),
            )
            .await
        });

        Box::pin(receiver)
    }
}

// # From the freedesktop spec https://specifications.freedesktop.org/menu-spec/menu-spec-1.0.html :
// This directory contains a .desktop file for each possible menu item.
// Each directory in the $XDG_DATA_DIRS search path should be used
// (i.e. desktop entries are collected from all of them, not just the first one that exists).
// When two desktop entries have the same name, the one appearing earlier in the path is used.
// The <DefaultAppDirs> element in a menu file indicates that this default list of desktop entry
// locations should be scanned at that point. If a menu file does not contain <DefaultAppDirs>,
// then these locations are not scanned.
async fn get_root_desktop_entries(
    sender: Sender<DesktopEntry>,
    finder: Arc<Option<IconFinder>>,
    entry_map: Arc<RwLock<Vec<String>>>,
) {
    let desktop_dir = PathBuf::from("/usr/share");
    get_desktop_entries(sender, desktop_dir.join("applications"), finder, entry_map).await;
}

async fn get_user_desktop_entries(
    sender: Sender<DesktopEntry>,
    finder: Arc<Option<IconFinder>>,
    entry_map: Arc<RwLock<Vec<String>>>,
) {
    let desktop_dir = dirs::data_local_dir().unwrap();
    get_desktop_entries(sender, desktop_dir.join("applications"), finder, entry_map).await;
}

async fn get_desktop_entries(
    mut sender: futures::channel::mpsc::Sender<DesktopEntry>,
    desktop_dir: PathBuf,
    finder: Arc<Option<IconFinder>>,
    entry_map: Arc<RwLock<Vec<String>>>,
) {
    let pattern = format!("{}/**/*.desktop", desktop_dir.to_str().unwrap());

    debug!(
        "Start glob walk in {:?}, with pattern {}",
        &desktop_dir, &pattern
    );
    for entry in glob(&pattern).expect("Failed to read glob pattern") {
        let desktop_entry = std::fs::read_to_string(entry.unwrap()).unwrap();
        // deserialize the .desktop file ignoring failure
        if let Ok(desktop_entry) = serde_ini::from_str::<DesktopEntryIni>(&desktop_entry) {
            let content = desktop_entry.content;

            // We need to keep track of already sent entries
            // "When two desktop entries have the same name, the one appearing earlier in the path is used"
            let mut map_entry_write_lock = entry_map.write().unwrap();
            if !map_entry_write_lock.contains(&content.name) {
                debug!("Sending desktop entry : {:?} to main thread", &content);
                if let Some(finder) = finder.borrow() {
                    sender
                        .start_send(DesktopEntry::with_icon(&content, finder))
                        .unwrap();
                } else {
                    sender.start_send(DesktopEntry::from(&content)).unwrap();
                }
                map_entry_write_lock.push(content.name);
            } else {
                debug!("Desktop entry {} already present", &content.name);
            }
        }
    }
}
