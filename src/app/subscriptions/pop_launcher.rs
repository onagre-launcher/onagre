use iced::futures::{channel::mpsc::channel, channel::mpsc::Receiver, channel::mpsc::Sender};
use iced::futures::{join, SinkExt, Stream, StreamExt};
use iced::stream as istream;
use onagre_launcher_toolkit::launcher::{json_input_stream, Request, Response};
use std::process::{exit, Stdio};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{ChildStderr, ChildStdin, ChildStdout, Command};
use tracing::{debug, error};

use crate::app::Message;

// Whenever a message is red from pop-launcher stdout, send it to the subscription receiver
async fn handle_stdout(stdout: ChildStdout, mut sender: Sender<Response>) {
    let mut stream = json_input_stream::<_, Response>(stdout);

    while let Some(response) = stream.next().await {
        debug!("Got a response from pop-launcher");
        debug!("{:?}", response);
        if let Err(err) = sender.send(response.unwrap()).await {
            error!(
                "Failed to send response to subscription receiver: {:?}",
                err
            );
        }
    }
}

// Whenever a message is red from pop-launcher stderr, print it to onagre stderr
async fn handle_stderr(stderr: ChildStderr) {
    let mut lines = BufReader::new(stderr).lines();

    while let Ok(Some(line)) = lines.next_line().await {
        debug!("line : {}", line);
    }
}

// Listen for incoming `pop_launcher::Request` from the receiver and forward them to
// pop launcher stdin
async fn handle_stdin(mut stdin: ChildStdin, mut request_rx: Receiver<Request>) {
    while let Some(request) = request_rx.next().await {
        let request = serde_json::to_string(&request).unwrap();
        let request = format!("{}\n", request);
        stdin.write_all(request.as_bytes()).await.unwrap();
        debug!("Wrote request {:?} to pop-launcher stdin", request);
    }
}

pub fn pop_launcher() -> impl Stream<Item = Message> {
    istream::channel(100, |mut output| async move {
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

        let (response_tx, mut response_rx) = channel(32);
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

        if let Err(err) = output.send(Message::PopLauncherReady(request_tx)).await {
            error!("Pop launcher subscription error: {err}");
        }

        while let Some(message) = response_rx.next().await {
            if let Err(err) = output.send(Message::PopMessage(message)).await {
                error!("pop launcher error: {err}")
            }
        }
    })
}
