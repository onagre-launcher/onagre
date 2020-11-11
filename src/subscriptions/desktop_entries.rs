use crate::entries::DesktopEntry;
use crate::freedesktop::desktop::DesktopEntryIni;
use crate::freedesktop::icons::IconFinder;
use crate::SETTINGS;
use async_std::fs;
use async_std::path::PathBuf as AsyncPathBuf;
use futures::channel::mpsc::Sender;
use futures::future::{BoxFuture, FutureExt};
use iced_native::futures::stream::BoxStream;
use iced_native::futures::StreamExt;
use iced_native::Subscription;
use std::borrow::Borrow;
use std::hash::Hash;
use std::sync::Arc;

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

            futures::future::join(
                get_root_desktop_entries(sender.clone(), Arc::clone(&finder)),
                get_user_desktop_entries(sender, finder),
            )
            .await
        });

        Box::pin(receiver)
    }
}

async fn get_root_desktop_entries(sender: Sender<DesktopEntry>, finder: Arc<Option<IconFinder>>) {
    let desktop_dir = AsyncPathBuf::from("/usr/share");
    walk_dir(sender, desktop_dir.join("applications"), finder).await;
}

async fn get_user_desktop_entries(sender: Sender<DesktopEntry>, finder: Arc<Option<IconFinder>>) {
    let desktop_dir: AsyncPathBuf = dirs::data_local_dir().unwrap().into();
    walk_dir(sender, desktop_dir.join("applications"), finder).await;
}

fn walk_dir(
    mut sender: futures::channel::mpsc::Sender<DesktopEntry>,
    desktop_dir: AsyncPathBuf,
    finder: Arc<Option<IconFinder>>,
) -> BoxFuture<'static, ()> {
    async move {
        let mut entries = fs::read_dir(desktop_dir).await.unwrap();

        while let Some(res) = entries.next().await {
            let entry = res.unwrap();

            if entry.path().is_dir().await {
                walk_dir(
                    sender.clone(),
                    entry.path().to_path_buf(),
                    Arc::clone(&finder),
                )
                .await;
            } else {
                let desktop_entry = fs::read_to_string(entry.path()).await.unwrap();
                if let Ok(desktop_entry) = serde_ini::from_str::<DesktopEntryIni>(&desktop_entry) {
                    if let Some(content) = desktop_entry.content {
                        if let Some(finder) = finder.borrow() {
                            sender
                                .start_send(DesktopEntry::with_icon(&content, finder))
                                .unwrap();
                        } else {
                            sender.start_send(DesktopEntry::from(&content)).unwrap();
                        }
                    }
                }
            }
        }
    }
    .boxed()
}
