use async_process::{Command, Stdio};
use async_std::{io::BufReader, prelude::*};
use anyhow::Result;
use crate::subscriptions::ToSubScription;
use async_std::fs;
use futures::future::{BoxFuture, FutureExt};
use iced_native::futures::stream::BoxStream;
use iced_native::futures::StreamExt;
use iced_native::Subscription;
use std::hash::Hash;

pub struct ExternalCommandSubscription {
    command: String,
}

impl ToSubScription<Vec<String>> for ExternalCommandSubscription {
    fn subscription() -> Subscription<Vec<String>> {
        iced::Subscription::from_recipe(ExternalCommandSubscription {
            command: "fd . /home/okno".to_string(),
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

        println!("{}", command);
        async_std::task::spawn(run_process(sender, command));

        Box::pin(receiver)
    }
}

async fn run_process(mut sender: futures::channel::mpsc::Sender<Vec<String>>, args: String) {
    let args = shell_words::split(&args).unwrap();

    let mut child = Command::new(&args[0])
        .args(&args[1..])
        .stdout(Stdio::piped())
        .spawn().unwrap();

    println!("Let's go !");

    let mut lines = BufReader::new(child.stdout.take().unwrap()).lines();

    let mut acc = vec![];
    while let Some(line) = lines.next().await {
        println!("Custom Sub Entry {:?}", line);
        acc.push(line.unwrap());
        if acc.len() > 100 {
            sender.start_send(acc).unwrap();
            acc = vec![];
        }
    }
}
