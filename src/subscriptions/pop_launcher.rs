use crate::entries::pop_entry::PopResponse;
use crate::subscriptions::pop_launcher::futures::join;
use async_process::{ChildStderr, ChildStdin, ChildStdout, Command};
use iced::futures;
use iced::futures::channel::mpsc;
use iced::futures::channel::mpsc::channel;
use iced::futures::channel::mpsc::Sender;
use iced::futures::io::BufReader;
use iced::futures::stream;
use iced::futures::AsyncBufReadExt;
use iced::futures::{AsyncWriteExt, SinkExt};
use iced_native::futures::stream::BoxStream;
use iced_native::futures::StreamExt;
use iced_native::Subscription;
use pop_launcher::{json_input_stream, Request, Response};
use std::hash::Hash;
use std::process::Stdio;

// Whenever a message is red from pop-launcher stdout, send it to the subscription receiver
async fn handle_stdout(stdout: ChildStdout, mut sender: Sender<Response>) {
    let mut stream = json_input_stream::<_, Response>(stdout);

    while let Some(response) = stream.next().await {
        debug!("Got a response from pop-launcher");
        trace!("{:?}", response);
        sender.send(response.unwrap()).await.unwrap();
    }
}

// Whenever a message is red from pop-launcher stderr, print it to onagre stderr
async fn handle_stderr(stderr: ChildStderr) {
    let mut lines = BufReader::new(stderr).lines();

    while let Some(line) = lines.next().await {
        debug!("line : {}", line.unwrap());
    }
}

// Listen for incoming `pop_launcher::Request` from the receiver and forward them to
// pop launcher stdin
async fn handle_stdin(mut stdin: ChildStdin, mut request_rx: mpsc::Receiver<Request>) {
    while let Some(request) = request_rx.next().await {
        let request = serde_json::to_string(&request).unwrap();
        let request = format!("{}\n", request);
        stdin.write(request.as_bytes()).await.unwrap();
        debug!("Wrote request {:?} to pop-launcher stdin", request);
        stdin.flush().await.unwrap();
    }
}

pub struct PopLauncherSubscription {
    id: u8,
}

#[derive(Debug, Clone)]
pub enum SubscriptionMessage {
    Ready(Sender<Request>),
    PopMessage(PopResponse),
}

impl PopLauncherSubscription {
    pub fn create() -> Subscription<SubscriptionMessage> {
        iced::Subscription::from_recipe(PopLauncherSubscription { id: 0 })
    }
}

impl<H, I> iced_native::subscription::Recipe<H, I> for PopLauncherSubscription
where
    H: std::hash::Hasher,
{
    type Output = SubscriptionMessage;

    fn hash(&self, state: &mut H) {
        std::any::TypeId::of::<Self>().hash(state);
        self.id.hash(state)
    }

    fn stream(self: Box<Self>, _: BoxStream<I>) -> BoxStream<Self::Output> {
        debug!("Starting `pop-launcher` subscription");
        let child = Command::new("pop-launcher")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();

        let (response_tx, response_rx) = channel(32);
        let (request_tx, request_rx) = channel(32);

        let stdout = child.stdout.unwrap();
        let stderr = child.stderr.unwrap();
        let stdin = child.stdin.unwrap();

        let stdout_handle = handle_stdout(stdout, response_tx);
        let stderr_handle = handle_stderr(stderr);
        let stdin_handle = handle_stdin(stdin, request_rx);

        async_std::task::spawn(async {
            join!(stdout_handle, stderr_handle, stdin_handle);
        });

        let pop_response_rx = response_rx
            .map(PopResponse::from)
            .map(SubscriptionMessage::PopMessage);

        Box::pin(stream::iter(vec![SubscriptionMessage::Ready(request_tx)]).chain(pop_response_rx))
    }
}
