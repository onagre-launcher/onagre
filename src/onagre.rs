use std::path::PathBuf;
use std::process::exit;

use iced::{
    Alignment, Application, Color, Column, Command, Container, Element, Length, Row,
    scrollable, Scrollable, Settings, Subscription, text_input, TextInput, window,
};
use iced::futures::channel::mpsc::Sender;
use iced_native::Event;
use iced_native::keyboard::KeyCode;
use pop_launcher::Request;

use crate::backend::{PopResponse, PopSearchResult};
use crate::backend::launcher::{PopLauncherSubscription, PopMessage};
use crate::freedesktop::desktop::DesktopEntry;
use crate::SETTINGS;
use crate::THEME;

pub fn run() -> iced::Result {
    debug!("Starting Onagre in debug mode");
    debug!(
        "Settings : \n\tAvailable modes : {:#?}\n\t Icon theme : {:#?}",
        SETTINGS.modes, SETTINGS.icons
    );

    Onagre::run(Settings {
        window: window::Settings {
            transparent: true,
            size: (800, 300),
            ..Default::default()
        },
        default_text_size: 20,
        antialiasing: true,
        ..Default::default()
    })
}

#[derive(Debug)]
struct Onagre {
    state: State,
    request_tx: Option<Sender<Request>>,
}

#[derive(Debug)]
struct State {
    current_mode_idx: usize,
    line_selected_idx: usize,
    entries: Vec<PopSearchResult>,
    scroll: scrollable::State,
    input: text_input::State,
    input_value: String,
}

impl Default for State {
    fn default() -> Self {
        State {
            current_mode_idx: 0,
            line_selected_idx: 0,
            entries: Vec::with_capacity(0),
            scroll: Default::default(),
            input: Default::default(),
            input_value: "".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    InputChanged(String),
    KeyboardEvent(KeyCode),
    PopSubscriptionResponse(PopMessage),
}

// #[derive(Debug, Clone, Eq, PartialEq, Hash)]
// pub enum Mode {
//     PopLancher,
//     Manual,
// }


impl Application for Onagre {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Onagre {
                state: Default::default(),
                request_tx: Default::default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Onagre".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        self.state.input.focus();

        match message {
            Message::InputChanged(input) => {
                self.state.input_value = input;
                debug!("Input changed");

                if let Some(sender) = &self.request_tx {
                    let mut sender = sender.clone();
                    let value = self.state.input_value.clone();
                    debug!("Sending message to pop thread : {}", value);
                    sender.try_send(Request::Search(value)).unwrap();
                }

                Command::none()
            }
            Message::KeyboardEvent(event) => {
                self.handle_input(event);
                Command::none()
            }
            Message::PopSubscriptionResponse(message) => {
                match message {
                    PopMessage::Ready(sender) => {
                        debug!("Subscription read, sender set");
                        self.request_tx = Some(sender);
                    }
                    PopMessage::Message(response) => match response {
                        PopResponse::Close => exit(0),
                        PopResponse::Context { .. } => {
                            todo!("Discrete graphics is not implemented")
                        }
                        PopResponse::DesktopEntry { path, .. } => {
                            self.run_command(path);
                        }
                        PopResponse::Update(search_updates) => {
                            self.state.entries = search_updates;
                        }
                        PopResponse::Fill(fill) => self.state.input_value = fill,
                    },
                };
                Command::none()
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        let keyboard_event = iced_native::subscription::events_with(|event, _status| match event {
            Event::Keyboard(iced::keyboard::Event::KeyPressed {
                                modifiers: _,
                                key_code,
                            }) => Some(Message::KeyboardEvent(key_code)),
            _ => None,
        });

        let subs = vec![
            keyboard_event,
            PopLauncherSubscription::subscription().map(Message::PopSubscriptionResponse),
        ];

        Subscription::batch(subs)
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        // Build rows from current mode search entries
        let rows = self
            .state
            .entries
            .iter()
            .map(|entry| {
                if entry.id as usize == self.state.line_selected_idx {
                    entry.to_row_selected().into()
                } else {
                    entry.to_row().into()
                }
            })
            .collect();

        let entries_column = Column::with_children(rows);

        // Scrollable element containing the rows
        let scrollable = Container::new(
            Scrollable::new(&mut self.state.scroll)
                .push(entries_column)
                .height(THEME.scrollable.height.into())
                .width(THEME.scrollable.width.into())
                .scrollbar_width(THEME.scrollable.scroller_width)
                .scroller_width(THEME.scrollable.scrollbar_width)
                .style(&THEME.scrollable),
        )
            .style(&THEME.scrollable)
            .padding(THEME.scrollable.padding);

        let search_input = TextInput::new(
            &mut self.state.input,
            "Search",
            &self.state.input_value,
            Message::InputChanged,
        )
            .width(THEME.search.bar.text_width.into())
            .style(&THEME.search.bar);

        let search_bar = Container::new(
            Row::new()
                .spacing(20)
                .align_items(Alignment::Center)
                .padding(2)
                .push(search_input)
                .width(THEME.search.width.into())
                .height(THEME.search.height.into()),
        )
            .padding(THEME.search.padding)
            .style(&THEME.search);

        let app_container = Container::new(
            Column::new()
                .push(search_bar)
                .push(scrollable)
                .align_items(Alignment::Start)
                .height(Length::Fill)
                .width(Length::Fill)
                .padding(20),
        )
            .height(Length::Fill)
            .width(Length::Fill)
            .style(THEME.as_ref());

        app_container.into()
    }

    fn background_color(&self) -> Color {
        Color::TRANSPARENT
    }
}

impl Onagre {
    fn run_command(&mut self, desktop_entry_path: PathBuf) -> Command<Message> {
        let desktop_entry = DesktopEntry::from_path(desktop_entry_path).unwrap();
        let argv = shell_words::split(&desktop_entry.exec);
        let args = argv.unwrap();
        let args = args
            .iter()
            // Filtering out special freedesktop syntax
            .filter(|entry| !entry.starts_with('%'))
            .collect::<Vec<&String>>();

        std::process::Command::new(&args[0])
            .args(&args[1..])
            .spawn()
            .expect("Command failure");

        exit(0);
    }

    fn handle_input(&mut self, key_code: KeyCode) {
        debug!("Handle input : {:?}", key_code);
        match key_code {
            KeyCode::Up => {
                if self.state.line_selected_idx != 0 {
                    self.state.line_selected_idx -= 1
                }
                self.snap();
            }
            KeyCode::Down => {
                let total_items = self.state.entries.len();
                if total_items != 0 && self.state.line_selected_idx < total_items - 1 {
                    self.state.line_selected_idx += 1
                }
                self.snap();
            }
            KeyCode::Enter => {
                if let Some(sender) = &self.request_tx {
                    let mut sender = sender.clone();
                    let selected = self.state.line_selected_idx;
                    sender.try_send(Request::Activate(selected as u32)).unwrap();
                }
            }
            KeyCode::Tab => { /* Todo */ }
            KeyCode::Escape => {
                exit(0);
            }
            _ => {}
        }
    }

    fn snap(&mut self) {
        let total_items = self.state.entries.len() as f32;

        let line_offset = if self.state.line_selected_idx == 0 {
            0
        } else {
            self.state.line_selected_idx + 1
        } as f32;

        let offset = (1.0 / total_items) * (line_offset) as f32;
        self.state.scroll.snap_to(offset);
    }
}
