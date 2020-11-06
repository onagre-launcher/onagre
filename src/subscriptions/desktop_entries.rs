use crate::desktop::{DesktopEntryIni};
use async_std::fs;
use async_std::path::PathBuf as AsyncPathBuf;
use futures::future::{BoxFuture, FutureExt};
use iced_native::futures::stream::BoxStream;
use iced_native::futures::StreamExt;
use std::hash::Hash;
use crate::subscriptions::ToSubScription;
use iced_native::Subscription;
use crate::entries::DesktopEntry;

pub struct DesktopEntryWalker {
    id: String,
}

impl ToSubScription<DesktopEntry> for DesktopEntryWalker {
    fn subscription() -> Subscription<DesktopEntry> {
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
            futures::future::join(
                get_root_desktop_entries(sender.clone()),
                get_user_desktop_entries(sender),
            )
            .await
        });

        Box::pin(receiver)
    }
}

async fn get_root_desktop_entries(sender: futures::channel::mpsc::Sender<DesktopEntry>) {
    let desktop_dir = AsyncPathBuf::from("/usr/share");
    println!("{:?}", desktop_dir);
    walk_dir(sender, desktop_dir.join("applications")).await;
}

async fn get_user_desktop_entries(sender: futures::channel::mpsc::Sender<DesktopEntry>) {
    let desktop_dir: AsyncPathBuf = dirs::data_local_dir().unwrap().into();
    walk_dir(sender, desktop_dir.join("applications")).await;
}

fn walk_dir(
    mut sender: futures::channel::mpsc::Sender<DesktopEntry>,
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
                if let Ok(desktop_entry) = serde_ini::from_str::<DesktopEntryIni>(&desktop_entry) {
                    if let Some(content) = desktop_entry.content {
                        println!("Found {:?}", content); // FIXME : add a logger
                        sender.start_send(DesktopEntry::from(&content)).unwrap();
                    }
                }
            }
        }
    }
        .boxed()
}

