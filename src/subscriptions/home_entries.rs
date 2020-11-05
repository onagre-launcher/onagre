use crate::desktop::{DesktopEntry, FileEntry};
use async_std::fs;
use async_std::path::PathBuf as AsyncPathBuf;
use futures::future::{BoxFuture, FutureExt};
use iced_native::futures::stream::BoxStream;
use iced_native::futures::StreamExt;
use std::hash::Hash;
use crate::subscriptions::ToSubScription;
use iced_native::Subscription;
use config::File;

pub struct HomeWalker {
    id: String,
}

impl ToSubScription<FileEntry> for HomeWalker {
    fn subscription() -> Subscription<FileEntry> {
        iced::Subscription::from_recipe(HomeWalker {
            id: "file_walker_subscription".to_string(),
        })
    }
}

impl<H, I> iced_native::subscription::Recipe<H, I> for HomeWalker
    where
        H: std::hash::Hasher,
{
    type Output = FileEntry;

    fn hash(&self, state: &mut H) {
        std::any::TypeId::of::<Self>().hash(state);
        self.id.hash(state)
    }

    fn stream(self: Box<Self>, _: BoxStream<I>) -> BoxStream<Self::Output> {
        let (sender, receiver) = futures::channel::mpsc::channel(100);

        // Spawn the file reader
        async_std::task::spawn(async {
            get_home_entries(sender)
                .await
        });

        Box::pin(receiver)
    }
}

async fn get_home_entries(sender: futures::channel::mpsc::Sender<FileEntry>) {
    let home = dirs::home_dir().unwrap();
    let home = AsyncPathBuf::from(home);
    walk_dir(sender, home).await;
}

fn walk_dir(
    mut sender: futures::channel::mpsc::Sender<FileEntry>,
    desktop_dir: AsyncPathBuf,
) -> BoxFuture<'static, ()> {
    async move {
        if let Ok(mut entries) = fs::read_dir(desktop_dir).await {
            while let Some(res) = entries.next().await {
                if let Ok(entry) = res {
                    if entry.path().is_dir().await {
                        walk_dir(sender.clone(), entry.path().to_path_buf()).await;
                    } else {
                        let path = entry.path().as_os_str().to_string_lossy().to_string();
                        println!("Found {:?}", path); // FIXME : add a logger
                        sender.start_send(FileEntry { path }).unwrap();
                    }
                }
            }
        }
    }
        .boxed()
}
