use anyhow::Result;
use async_process::{Command, Stdio};
use async_std::fs;
use async_std::{io::BufReader, prelude::*};
use futures::future::{BoxFuture, FutureExt};
use iced_native::futures::stream::BoxStream;
use iced_native::futures::StreamExt;
use iced_native::Subscription;
use std::hash::Hash;

pub struct ExternalCommandSubscription {
    command: String,
}

impl ExternalCommandSubscription {
    pub fn subscription() -> Subscription<Vec<String>> {
        iced::Subscription::from_recipe(ExternalCommandSubscription {
            command: format!("fd . /home/okno"),
        })
    }
}

impl<H, I> iced_native::subscription::Recipe<H, I> for ExternalCommandSubscription
    where
        H: std::hash::Hasher,
{
    type Output = Vec<String>;

    fn hash(&self, state: &mut H) {
        std::any::TypeId::of::<Self>().hash(state);
        self.command.hash(state)
    }

    fn stream(self: Box<Self>, _: BoxStream<I>) -> BoxStream<Self::Output> {
        let (sender, receiver) = futures::channel::mpsc::channel(100000);
        let command = self.command.clone();
        std::thread::spawn(|| async_std::task::spawn(run_process(sender, command)));
        Box::pin(receiver)
    }
}

async fn run_process(mut sender: futures::channel::mpsc::Sender<Vec<String>>, args: String) {
    let args = shell_words::split(&args).unwrap();

    let mut child = Command::new(&args[0])
        .args(&args[1..])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let lines = BufReader::new(child.stdout.take().unwrap()).lines();
    let mut chunks = lines.chunks(100);

    while let Some(chunk) = chunks.next().await {
        let mut next_batch = Vec::with_capacity(100);
        for entry in chunk {
            next_batch.push(entry.unwrap())
        }
        sender.start_send(next_batch).unwrap();
    }
}
