use std::path::Path;
use std::process::exit;

use fuzzy_matcher::skim::SkimMatcherV2;
use iced::futures::channel::mpsc::{Sender, TrySendError};
use iced::{
    scrollable, text_input, window, Alignment, Application, Color, Column, Command, Container,
    Element, Length, Row, Scrollable, Settings, Subscription, TextInput,
};
use iced_native::keyboard::KeyCode;
use iced_native::Event;
use pop_launcher::Request;

use crate::app::active_mode::ActiveMode;
use crate::config::ModeSettings;
use crate::db::desktop_entry::DesktopEntryEntity;
use crate::db::run::RunCommandEntity;
use crate::db::Database;
use crate::entries::external_entry::ExternalEntries;
use crate::entries::pop_entry::PopResponse;
use crate::entries::{AsEntry, EntryCache};
use crate::freedesktop::desktop::DesktopEntry;
use crate::subscriptions::external::ExternalCommandSubscription;
use crate::subscriptions::pop_launcher::{PopLauncherSubscription, SubscriptionMessage};
use crate::SETTINGS;
use crate::THEME;

mod active_mode;

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
    mode: ActiveMode,
    db: Database,
    line_selected_idx: usize,
    entries: EntryCache,
    external_entries_match: ExternalEntries,
    scroll: scrollable::State,
    input: text_input::State,
    input_value: String,
    exec_on_next_search: bool,
}

impl Default for State {
    fn default() -> Self {
        State {
            mode: ActiveMode::History,
            db: Default::default(),
            line_selected_idx: 0,
            entries: EntryCache {
                external: Default::default(),
                pop_search: vec![],
                de_history: vec![],
                terminal: vec![],
            },
            external_entries_match: Default::default(),
            scroll: Default::default(),
            input: Default::default(),
            input_value: "".to_string(),
            exec_on_next_search: false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    InputChanged(String),
    KeyboardEvent(KeyCode),
    SubscriptionResponse(SubscriptionMessage),
}

impl Application for Onagre {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_: Self::Flags) -> (Self, Command<Self::Message>) {
        let onagre = Onagre {
            state: Default::default(),
            request_tx: Default::default(),
        };

        (onagre, Command::none())
    }

    fn title(&self) -> String {
        "Onagre".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        self.state.input.focus();
        match self.state.mode {
            ActiveMode::Calc => {}
            ActiveMode::DesktopEntry => {}
            ActiveMode::Find => {}
            ActiveMode::Files => {}
            ActiveMode::Recent => {}
            ActiveMode::Scripts => {}
            ActiveMode::Terminal => {
                if self.state.entries.terminal.is_empty() {
                    let entries = self.state.db.get_all::<RunCommandEntity>();
                    self.state.entries.terminal = entries;
                }
            }
            ActiveMode::Web(_) => {}
            ActiveMode::External(_) => {}
            ActiveMode::History => {
                if self.state.entries.de_history.is_empty() {
                    self.state.entries.de_history = self.state.db.get_all();
                }
            }
        }

        match message {
            Message::InputChanged(input) => self.on_input_changed(input),
            Message::KeyboardEvent(event) => self.handle_input(event),
            Message::SubscriptionResponse(message) => self.on_pop_launcher_message(message),
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        let keyboard_event = Onagre::keyboard_event();

        let pop_launcher = PopLauncherSubscription::create().map(Message::SubscriptionResponse);
        let sub = match &self.state.mode {
            ActiveMode::External(key) => {
                let command = SETTINGS.modes.get(key).unwrap();
                Some(
                    ExternalCommandSubscription::create(command.source.as_ref().unwrap())
                        .map(Message::SubscriptionResponse),
                )
            }
            _ => None,
        };

        let mut subs = vec![keyboard_event, pop_launcher];

        if let Some(external) = sub {
            subs.push(external)
        };

        Subscription::batch(subs)
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        // Build rows from current mode search entries
        let selected = self.state.line_selected_idx;
        let rows = match &self.state.mode {
            ActiveMode::DesktopEntry
            | ActiveMode::Find
            | ActiveMode::Files
            | ActiveMode::Recent
            | ActiveMode::Scripts
            | ActiveMode::Calc
            | ActiveMode::Web(_) => self
                .state
                .entries
                .pop_search
                .iter()
                .map(|entry| entry.to_row(selected, entry.id as usize).into())
                .collect(),
            ActiveMode::Terminal => self
                .state
                .entries
                .terminal
                .iter()
                .enumerate()
                .map(|(idx, entry)| entry.to_row(selected, idx).into())
                .collect(),
            ActiveMode::External(_) => self
                .state
                .external_entries_match
                .get()
                .iter()
                .enumerate()
                .map(|(idx, entry)| entry.to_row(selected, idx).into())
                .collect(),
            ActiveMode::History => self
                .state
                .entries
                .de_history
                .iter()
                .enumerate()
                .map(|(idx, entry)| entry.to_row(selected, idx).into())
                .collect(),
        };

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
    // Only call this if we are using a non pop_launcher mode
    // Pop Launcher entries provides their indices
    fn current_entry(&self) -> Option<&str> {
        match &self.state.mode {
            ActiveMode::History => self
                .state
                .entries
                .de_history
                .get(self.state.line_selected_idx)
                .map(|entry| entry.path.to_str().unwrap()),
            ActiveMode::External(_custom) => self
                .state
                .external_entries_match
                .get()
                .get(self.state.line_selected_idx)
                .map(|entry| entry.value.as_str()),
            ActiveMode::Terminal => self
                .state
                .entries
                .terminal
                .get(self.state.line_selected_idx)
                .map(|entry| entry.command.as_str()),
            _pop_mode => None,
        }
    }

    fn on_input_changed(&mut self, input: String) -> Command<Message> {
        self.state.line_selected_idx = 0;
        self.state.scroll.snap_to(0.0);
        self.state.input_value = input;
        self.state.mode = ActiveMode::from(self.state.input_value.as_str());

        debug!("Current mode : {:?}", self.state.mode);

        match &self.state.mode {
            ActiveMode::External(mode) => {
                let term = self
                    .state
                    .input_value
                    .strip_prefix(mode)
                    .map(str::trim_start);

                match term {
                    Some(term) if !term.is_empty() => {
                        debug!("Search term for external command : {:?}", term);
                        let entries = self
                            .state
                            .entries
                            .external
                            .match_external(term, &SkimMatcherV2::default().ignore_case());
                        let entries = ExternalEntries::new(entries);
                        self.state.external_entries_match = entries;
                        debug!("{:?}", self.state.entries);
                    }
                    _ => self.state.external_entries_match = self.state.entries.external.clone(),
                }
            }
            ActiveMode::Calc
            | ActiveMode::DesktopEntry
            | ActiveMode::Find
            | ActiveMode::Files
            | ActiveMode::Recent
            | ActiveMode::Scripts
            | ActiveMode::Terminal
            | ActiveMode::Web(_) => {
                self.state.external_entries_match = ExternalEntries::new(Vec::with_capacity(0));
                let value = self.state.input_value.clone();
                self.pop_request(Request::Search(value))
                    .expect("Unable to send search request to pop-launcher")
            }
            ActiveMode::History => {}
        }

        Command::none()
    }
    fn run_command<P: AsRef<Path>>(&self, desktop_entry_path: P) -> Command<Message> {
        let desktop_entry = DesktopEntry::from_path(&desktop_entry_path).unwrap();

        DesktopEntryEntity::persist(&desktop_entry, desktop_entry_path.as_ref(), &self.state.db);

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

    fn handle_input(&mut self, key_code: KeyCode) -> Command<Message> {
        debug!("Handle input : {:?}", key_code);
        match key_code {
            KeyCode::Up => {
                if self.state.line_selected_idx != 0 {
                    self.state.line_selected_idx -= 1
                }
                self.snap();
            }
            KeyCode::Down => {
                let total_items = self.current_entries_len();
                if total_items != 0 && self.state.line_selected_idx < total_items - 1 {
                    self.state.line_selected_idx += 1
                }
                self.snap();
            }
            KeyCode::Enter => return self.on_execute(),
            KeyCode::Tab => {
                self.pop_request(Request::Complete(self.state.line_selected_idx as u32))
                    .expect("Unable to send request to pop-launcher");
            }
            KeyCode::Escape => {
                exit(0);
            }
            _ => {}
        };

        Command::none()
    }

    fn snap(&mut self) {
        let total_items = self.current_entries_len() as f32;

        let line_offset = if self.state.line_selected_idx == 0 {
            0
        } else {
            self.state.line_selected_idx + 1
        } as f32;

        let offset = (1.0 / total_items) * (line_offset) as f32;
        self.state.scroll.snap_to(offset);
    }

    fn on_pop_launcher_message(&mut self, message: SubscriptionMessage) -> Command<Message> {
        match message {
            SubscriptionMessage::Ready(sender) => {
                debug!("Subscription read, sender set");
                self.request_tx = Some(sender);
            }
            SubscriptionMessage::PopMessage(response) => match response {
                PopResponse::Close => exit(0),
                PopResponse::Context { .. } => {
                    todo!("Discrete graphics is not implemented")
                }
                PopResponse::DesktopEntry { path, .. } => {
                    self.run_command(path);
                }
                PopResponse::Update(search_updates) => {
                    if self.state.exec_on_next_search {
                        self.pop_request(Request::Activate(0))
                            .expect("Unable to send Activate request to pop-launcher");
                        return Command::none();
                    }
                    self.state.entries.pop_search = search_updates;
                }
                PopResponse::Fill(fill) => self.state.input_value = fill,
            },
            SubscriptionMessage::ExternalMessage(entries) => {
                self.state.entries.external.extend_from_slice(&entries);
            }
        };
        Command::none()
    }

    fn on_execute(&mut self) -> Command<Message> {
        match &self.state.mode {
            ActiveMode::Calc
            | ActiveMode::DesktopEntry
            | ActiveMode::Find
            | ActiveMode::Files
            | ActiveMode::Recent
            | ActiveMode::Scripts
            | ActiveMode::Web(_) => {
                if let Some(sender) = &self.request_tx {
                    let mut sender = sender.clone();
                    let selected = self.state.line_selected_idx;
                    sender.try_send(Request::Activate(selected as u32)).unwrap();
                }
            }
            ActiveMode::Terminal => {
                RunCommandEntity::persist(&self.state.input_value, &self.state.db);
                self.state.exec_on_next_search = true;
                self.state.input_value = self.current_entry().unwrap().to_string();
                self.pop_request(Request::Search(self.state.input_value.clone()))
                    .expect("Error sending search request to pop-launcher")
            }
            ActiveMode::External(mode) => {
                let mode_settings = SETTINGS.modes.get(mode).unwrap();
                let entry = self.current_entry().unwrap();
                mode_settings.execute(entry);
            }
            ActiveMode::History => {
                let path = self.current_entry();
                let path = path.unwrap();
                self.run_command(path);
            }
        }

        Command::none()
    }

    fn current_entries_len(&self) -> usize {
        match self.state.mode {
            ActiveMode::Calc
            | ActiveMode::DesktopEntry
            | ActiveMode::Find
            | ActiveMode::Files
            | ActiveMode::Recent
            | ActiveMode::Web(_)
            | ActiveMode::Scripts => self.state.entries.pop_search.len(),
            ActiveMode::Terminal => self.state.entries.terminal.len(),
            ActiveMode::External(_) => self.state.external_entries_match.len(),
            ActiveMode::History => self.state.entries.de_history.len(),
        }
    }

    fn pop_request(&self, request: Request) -> Result<(), TrySendError<Request>> {
        let sender = self.request_tx.as_ref().unwrap();
        let mut sender = sender.clone();
        debug!("Sending message to pop launcher : {:?}", request);
        sender.try_send(request)
    }

    fn keyboard_event() -> Subscription<Message> {
        iced_native::subscription::events_with(|event, _status| match event {
            Event::Keyboard(iced::keyboard::Event::KeyPressed {
                modifiers: _,
                key_code,
            }) => Some(Message::KeyboardEvent(key_code)),
            _ => None,
        })
    }
}

impl ModeSettings {
    fn execute(&self, entry: &str) {
        let command = self.target.replace("%", entry);
        let args = shell_words::split(&command).unwrap();
        let args = args.iter().collect::<Vec<&String>>();

        std::process::Command::new(&args[0])
            .args(&args[1..])
            .spawn()
            .expect("Command failure");

        exit(0);
    }
}
