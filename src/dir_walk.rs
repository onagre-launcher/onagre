use crate::desktop::{DesktopEntry, OnagreEntry};
use async_std::fs;
use async_std::path::PathBuf as AsyncPathBuf;
use futures::future::{BoxFuture, FutureExt};
use iced_native::futures::stream::BoxStream;
use iced_native::futures::StreamExt;
use std::hash::Hash;

pub struct FileWalker {
    path: String,
}

impl FileWalker {
    pub fn to_subscription() -> iced::Subscription<OnagreEntry> {
        iced::Subscription::from_recipe(FileWalker {
            path: "cocuou".to_string(),
        })
    }
}

impl<H, I> iced_native::subscription::Recipe<H, I> for FileWalker
where
    H: std::hash::Hasher,
{
    type Output = OnagreEntry;

    fn hash(&self, state: &mut H) {
        std::any::TypeId::of::<Self>().hash(state);
        self.path.hash(state)
    }

    fn stream(self: Box<Self>, _: BoxStream<I>) -> BoxStream<Self::Output> {
        let (sender, receiver) = futures::channel::mpsc::channel(100000);

        // Spawn the file reader
        async_std::task::spawn(async {
            futures::future::join(
                get_root_desktop_entries(sender.clone()),
                get_user_desktop_entries(sender),
            )
            .await
        });

        Box::pin(receiver)
    }
}

async fn get_root_desktop_entries(sender: futures::channel::mpsc::Sender<OnagreEntry>) {
    let desktop_dir = AsyncPathBuf::from("/usr/share");
    println!("{:?}", desktop_dir);
    walk_dir(sender, desktop_dir.join("applications")).await;
}

async fn get_user_desktop_entries(sender: futures::channel::mpsc::Sender<OnagreEntry>) {
    let desktop_dir: AsyncPathBuf = dirs::data_local_dir().unwrap().into();
    walk_dir(sender, desktop_dir.join("applications")).await;
}

fn walk_dir(
    mut sender: futures::channel::mpsc::Sender<OnagreEntry>,
    desktop_dir: AsyncPathBuf,
) -> BoxFuture<'static, ()> {
    async move {
        let mut entries = fs::read_dir(desktop_dir).await.unwrap();

        while let Some(res) = entries.next().await {
            let entry = res.unwrap();

            if entry.path().is_dir().await {
                walk_dir(sender.clone(), entry.path().to_path_buf()).await;
            } else {
                let desktop_entry = fs::read_to_string(entry.path()).await.unwrap();
                if let Ok(desktop_entry) = serde_ini::from_str::<DesktopEntry>(&desktop_entry) {
                    if let Some(content) = desktop_entry.content {
                        println!("Sending entry {:?}", content);
                        sender.start_send(OnagreEntry::from(&content)).unwrap();
                    }
                }
            }
        }
    }
    .boxed()
}
