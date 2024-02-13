use iced::futures::channel::mpsc;
use iced::futures::channel::mpsc::{channel, Sender};
use iced::futures::stream::BoxStream;
use iced::futures::{join, SinkExt, StreamExt};
use iced::Subscription;
use iced_core::event::Status;
use iced_runtime::futures::futures::stream;
use iced_runtime::futures::subscription::Recipe;
use onagre_launcher_toolkit::launcher::{json_input_stream, Request, Response};
use std::hash::Hash;
use std::process::{exit, Stdio};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{ChildStderr, ChildStdin, ChildStdout, Command};
use tracing::{debug, error};

// Whenever a message is red from pop-launcher stdout, send it to the subscription receiver
async fn handle_stdout(stdout: ChildStdout, mut sender: Sender<Response>) {
    let mut stream = json_input_stream::<_, Response>(stdout);

    while let Some(response) = stream.next().await {
        debug!("Got a response from pop-launcher");
        debug!("{:?}", response);
        sender.send(response.unwrap()).await.unwrap();
    }
}

// Whenever a message is red from pop-launcher stderr, print it to onagre stderr
async fn handle_stderr(stderr: ChildStderr) {
    let mut lines = BufReader::new(stderr).lines();

    while let Ok(line) = lines.next_line().await {
        debug!("line : {}", line.unwrap());
    }
}

// Listen for incoming `pop_launcher::Request` from the receiver and forward them to
// pop launcher stdin
async fn handle_stdin(mut stdin: ChildStdin, mut request_rx: mpsc::Receiver<Request>) {
    while let Some(request) = request_rx.next().await {
        let request = serde_json::to_string(&request).unwrap();
        let request = format!("{}\n", request);
        stdin.write_all(request.as_bytes()).await.unwrap();
        debug!("Wrote request {:?} to pop-launcher stdin", request);
        stdin.flush().await.unwrap();
    }
}

pub struct PopLauncherSubscription;

#[derive(Debug, Clone)]
pub enum SubscriptionMessage {
    Ready(Sender<Request>),
    PopMessage(Response),
}

impl PopLauncherSubscription {
    pub fn create() -> Subscription<SubscriptionMessage> {
        Subscription::from_recipe(PopLauncherSubscription)
    }
}

impl Recipe for PopLauncherSubscription {
    type Output = SubscriptionMessage;

    fn hash(&self, state: &mut iced_core::Hasher) {
        std::any::TypeId::of::<Self>().hash(state);
        "PopLauncherSubscription".hash(state)
    }

    fn stream(self: Box<Self>, _: BoxStream<(iced::Event, Status)>) -> BoxStream<Self::Output> {
        debug!("Starting `pop-launcher` subscription");
        let Ok(child) = Command::new("pop-launcher")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
        else {
            error!("Failed to start pop-launcher backend.");
            error!("Make sure either pop-launcher or onagre-launcher is installed.");
            error!("See: https://github.com/pop-os/launcher or https://github.com/onagre-launcher/launcher");
            exit(1);
        };

        let (response_tx, response_rx) = channel(32);
        let (request_tx, request_rx) = channel(32);

        let stdout = child.stdout.unwrap();
        let stderr = child.stderr.unwrap();
        let stdin = child.stdin.unwrap();

        let stdout_handle = handle_stdout(stdout, response_tx);
        let stderr_handle = handle_stderr(stderr);
        let stdin_handle = handle_stdin(stdin, request_rx);

        tokio::spawn(async {
            join!(stdout_handle, stderr_handle, stdin_handle);
        });

        let pop_response_rx = response_rx.map(SubscriptionMessage::PopMessage);

        Box::pin(stream::iter(vec![SubscriptionMessage::Ready(request_tx)]).chain(pop_response_rx))
    }
}
