use crate::app::entries::pop_entry::PopSearchResult;
use crate::app::entries::AsEntry;
use crate::app::mode::ActiveMode;
use crate::app::plugin_matchers::Plugin;
use crate::app::state::{Selection, State};
use crate::app::subscriptions::plugin_configs::PluginMatcherSubscription;
use crate::app::subscriptions::pop_launcher::{PopLauncherSubscription, SubscriptionMessage};
use crate::db::desktop_entry::DesktopEntryEntity;
use crate::db::plugin::PluginCommandEntity;
use crate::db::web::WebEntity;
use crate::font::DEFAULT_FONT;
use crate::freedesktop::desktop::DesktopEntry;
use crate::{font, THEME};
use iced::alignment::{Horizontal, Vertical};
use iced::futures::channel::mpsc::{Sender, TrySendError};
use iced::keyboard::KeyCode;
use iced::{
    window, Alignment, Application, Color, Column, Container, Element, Length, Row, Scrollable,
    Settings, Text, TextInput,
};
use iced_native::{Command, Event, Subscription};
use log::{debug, trace};
use pop_launcher_toolkit::launcher::{Request, Response};
use std::path::Path;
use std::process::exit;

pub mod cache;
pub mod entries;
pub mod mode;
pub mod plugin_matchers;
pub mod state;
pub mod style;
pub mod subscriptions;

pub fn run() -> iced::Result {
    debug!("Starting Onagre in debug mode");

    let default_font = THEME
        .font
        .as_ref()
        .and_then(|font| font::load(font))
        .unwrap_or(DEFAULT_FONT);

    Onagre::run(Settings {
        id: Some("onagre".to_string()),
        window: window::Settings {
            transparent: true,
            size: THEME.size,
            decorations: false,
            always_on_top: true,
            resizable: false,
            position: window::Position::Centered,
            min_size: None,
            max_size: None,
            icon: None,
        },
        default_text_size: THEME.font_size,
        text_multithreading: false,
        antialiasing: true,
        exit_on_close_request: false,
        default_font: Some(default_font),
        flags: (),
        try_opengles_first: false,
    })
}

#[derive(Debug)]
pub struct Onagre<'a> {
    state: State<'a>,
    request_tx: Option<Sender<Request>>,
}

#[derive(Debug, Clone)]
pub enum Message {
    InputChanged(String),
    KeyboardEvent(KeyCode),
    SubscriptionResponse(SubscriptionMessage),
    PluginConfig(Plugin),
    Unfocused,
}

impl Application for Onagre<'_> {
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

        match message {
            Message::InputChanged(input) => self.on_input_changed(input),
            Message::KeyboardEvent(event) => self.handle_input(event),
            Message::SubscriptionResponse(message) => self.on_pop_launcher_message(message),
            Message::Unfocused => {
                if THEME.exit_unfocused {
                    exit(0);
                } else {
                    Command::none()
                }
            }
            Message::PluginConfig(plugin) => {
                self.state
                    .plugin_matchers
                    .insert(plugin.name.clone(), plugin);
                Command::none()
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        let keyboard_event = Onagre::keyboard_event();
        let pop_launcher = PopLauncherSubscription::create().map(Message::SubscriptionResponse);
        let matchers = PluginMatcherSubscription::create().map(Message::PluginConfig);
        let subs = vec![keyboard_event, pop_launcher, matchers];
        Subscription::batch(subs)
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        // Build rows from current mode search entries
        let selected = self.selected();
        let rows = match &self.state.get_active_mode() {
            ActiveMode::Plugin {
                plugin_name,
                history,
                ..
            } if *history => self
                .state
                .cache
                .plugin_history(plugin_name)
                .iter()
                .enumerate()
                .map(|(idx, entry)| entry.to_row(selected, idx).into())
                .collect(),
            ActiveMode::Web(web_name) => self
                .state
                .cache
                .web_history(web_name)
                .iter()
                .enumerate()
                .map(|(idx, entry)| entry.to_row(selected, idx).into())
                .collect(),
            ActiveMode::History => self
                .state
                .cache
                .de_history()
                .iter()
                .enumerate()
                .map(|(idx, entry)| entry.to_row(selected, idx).into())
                .collect(),
            _ => self
                .state
                .pop_search
                .iter()
                .map(|entry| {
                    PopSearchResult(entry)
                        .to_row(selected, entry.id as usize)
                        .into()
                })
                .collect(),
        };

        // Scrollable element containing the rows
        let scrollable = Scrollable::new(&mut self.state.scroll)
            .push(Column::with_children(rows))
            .style(THEME.scrollable())
            .scrollbar_margin(THEME.scrollable().scrollbar_margin)
            .scrollbar_width(THEME.scrollable().scrollbar_width)
            .scroller_width(THEME.scrollable().scroller_width)
            .width(Length::Fill)
            .height(Length::Fill);

        let scrollable = Container::new(scrollable)
            .style(&THEME.app_container.rows)
            .padding(THEME.app_container.rows.padding.to_iced_padding())
            .width(THEME.app_container.rows.width)
            .height(THEME.app_container.rows.height); // TODO: add this to stylesheet

        let search_input = Container::new(
            TextInput::new(
                &mut self.state.input,
                "Search",
                &self.state.input_value.input_display,
                Message::InputChanged,
            )
            .padding(THEME.search_input().padding.to_iced_padding())
            .width(THEME.search_input().text_width)
            .size(THEME.search_input().font_size)
            .style(THEME.search_input()),
        )
        .width(THEME.search_input().width)
        .height(THEME.search_input().height)
        .align_x(THEME.search_input().align_x)
        .align_y(THEME.search_input().align_y);

        let search_bar = Row::new().width(Length::Fill).height(Length::Fill);
        // Either plugin_hint is enabled and we try to display it
        // Or we display the normal search input
        let search_bar = match THEME.plugin_hint() {
            None => search_bar.push(search_input),
            Some(plugin_hint_style) => if !self.state.input_value.modifier_display.is_empty() {
                let plugin_hint = Container::new(
                    Text::new(&self.state.input_value.modifier_display)
                        .vertical_alignment(Vertical::Center)
                        .horizontal_alignment(Horizontal::Center)
                        .size(plugin_hint_style.font_size),
                )
                .style(plugin_hint_style)
                .width(plugin_hint_style.width)
                .height(plugin_hint_style.height)
                .align_y(plugin_hint_style.align_y)
                .align_x(plugin_hint_style.align_x)
                .padding(plugin_hint_style.padding.to_iced_padding());

                search_bar.push(plugin_hint).push(search_input)
            } else {
                search_bar.push(search_input)
            }
            .spacing(THEME.search().spacing),
        };

        let search_bar = Container::new(search_bar)
            .style(THEME.search())
            .align_x(THEME.search().align_x)
            .align_y(THEME.search().align_y)
            .padding(THEME.search().padding.to_iced_padding())
            .width(THEME.search().width)
            .height(THEME.search().height);

        let app_container = Container::new(
            Column::new()
                .push(search_bar)
                .push(scrollable)
                .align_items(Alignment::Start),
        )
        .padding(THEME.app().padding.to_iced_padding())
        .style(THEME.app())
        .center_y()
        .center_x();

        let app_wrapper = Container::new(app_container)
            .center_y()
            .center_x()
            .height(Length::Fill)
            .width(Length::Fill)
            .padding(THEME.padding.to_iced_padding())
            .style(&*THEME);

        app_wrapper.into()
    }

    fn background_color(&self) -> Color {
        Color::TRANSPARENT
    }
}

impl Onagre<'_> {
    // Only call this if we are using entries from the database
    // in order to re-ask pop-launcher for the exact same entry
    fn current_entry(&self) -> Option<String> {
        let selected = self.selected();
        match &self.state.get_active_mode() {
            ActiveMode::History => self
                .state
                .cache
                .de_history()
                .get(selected.unwrap())
                .map(|entry| entry.path.to_string_lossy().to_string()),
            ActiveMode::Plugin {
                modifier,
                plugin_name,
                ..
            } => {
                // Get user input as pop-entry
                match selected {
                    None => {
                        return self.state.pop_search.get(0).map(|entry| entry.name.clone());
                    }
                    Some(selected) => self
                        .state
                        .cache
                        .plugin_history(plugin_name)
                        .get(selected)
                        .map(|entry| format!("{}{}", modifier, entry.query)),
                }
            }
            ActiveMode::Web(web_name) => {
                // Get user input as pop-entry
                match selected {
                    None => {
                        return self.state.pop_search.get(0).map(|entry| entry.name.clone());
                    }
                    Some(selected) => self
                        .state
                        .cache
                        .web_history(web_name)
                        .get(selected)
                        .map(|entry| entry.query()),
                }
            }
            _pop_mode => None,
        }
    }

    fn on_input_changed(&mut self, input: String) -> Command<Message> {
        self.state.set_input(&input);
        self.state.selected = match self.state.get_active_mode() {
            // For those mode first line is unselected on change
            // We want to issue a pop-launcher search request to get the query at index 0 in
            // the next search response, then activate it
            ActiveMode::Web(_) | ActiveMode::History => Selection::Reset,
            ActiveMode::Plugin { history, .. } if *history => Selection::Reset,
            _ => Selection::PopLauncher(0),
        };

        self.state.scroll.snap_to(0.0);

        match &self.state.get_active_mode() {
            ActiveMode::History => {}
            _ => {
                let value = self.state.get_input();

                self.pop_request(Request::Search(value))
                    .expect("Unable to send search request to pop-launcher")
            }
        }

        Command::none()
    }

    fn run_command<P: AsRef<Path>>(&self, desktop_entry_path: P) -> Command<Message> {
        let desktop_entry = DesktopEntry::from_path(&desktop_entry_path).unwrap();

        DesktopEntryEntity::persist(
            &desktop_entry,
            desktop_entry_path.as_ref(),
            &self.state.cache.db,
        );

        let argv = shell_words::split(&desktop_entry.exec);
        let args = argv.unwrap();
        let args = args
            .iter()
            // Filter out special freedesktop syntax
            .filter(|entry| !entry.starts_with('%'))
            .collect::<Vec<&String>>();

        std::process::Command::new(&args[0])
            .args(&args[1..])
            .spawn()
            .expect("Command failure");

        exit(0);
    }

    fn handle_input(&mut self, key_code: KeyCode) -> Command<Message> {
        match key_code {
            KeyCode::Up => {
                self.dec_selected();
                self.snap();
                trace!("Selected line : {:?}", self.selected());
            }
            KeyCode::Down => {
                self.inc_selected();
                trace!("Selected line : {:?}", self.selected());
            }
            KeyCode::Enter => return self.on_execute(),
            KeyCode::Tab => {
                if let Some(selected) = self.selected() {
                    self.pop_request(Request::Complete(selected as u32))
                        .expect("Unable to send request to pop-launcher");
                }
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
        match self.selected() {
            None => self.state.scroll.snap_to(0.0),
            Some(selected) => {
                let line_offset = if selected == 0 { 0 } else { &selected + 1 } as f32;

                let offset = (1.0 / total_items) * (line_offset) as f32;
                self.state.scroll.snap_to(offset);
            }
        }
    }

    fn on_pop_launcher_message(&mut self, message: SubscriptionMessage) -> Command<Message> {
        match message {
            SubscriptionMessage::Ready(sender) => {
                self.request_tx = Some(sender);
            }
            SubscriptionMessage::PopMessage(response) => match response {
                Response::Close => exit(0),
                Response::Context { .. } => todo!("Discrete graphics is not implemented"),
                Response::DesktopEntry { path, .. } => {
                    debug!("Launch DesktopEntry {path:?} via run_command");
                    self.run_command(path);
                }
                Response::Update(search_updates) => {
                    if self.state.exec_on_next_search {
                        debug!("Launch entry 0 via PopRequest::Activate");
                        self.pop_request(Request::Activate(0))
                            .expect("Unable to send Activate request to pop-launcher");
                        return Command::none();
                    }
                    self.state.pop_search = search_updates;
                }
                Response::Fill(fill) => self.complete(fill),
            },
        };

        Command::none()
    }

    fn complete(&mut self, fill: String) {
        let filled = if THEME.plugin_hint().is_none() {
            self.state.input_value.input_display = fill.into();
            self.state.input.move_cursor_to_end();
            self.state.input_value.input_display.clone()
        } else {
            let mode_prefix = &self.state.input_value.modifier_display;
            let fill = fill
                .strip_prefix(mode_prefix)
                .expect("Auto-completion Error");
            self.state.input_value.input_display = fill.into();
            self.state.input.move_cursor_to_end();
            self.state.input_value.input_display.clone()
        };

        self.on_input_changed(filled);
    }

    fn on_execute(&mut self) -> Command<Message> {
        match &self.state.get_active_mode() {
            ActiveMode::Plugin {
                plugin_name,
                history,
                ..
            } if *history => {
                PluginCommandEntity::persist(
                    plugin_name,
                    &self.state.get_input(),
                    &self.state.cache.db,
                );

                // Running the user input query at index zero
                if self.selected().is_none() {
                    self.pop_request(Request::Activate(0))
                        .expect("Unable to send pop-launcher request")
                } else {
                    // Re ask pop-launcher for a stored query
                    self.state.exec_on_next_search = true;
                    let command = self.current_entry().unwrap();
                    self.state.set_input(&command);
                    self.pop_request(Request::Search(command))
                        .expect("Unable to send pop-launcher request");
                }
            }
            ActiveMode::Web(kind) => {
                let query = self.state.get_input();
                let query = query.strip_prefix(kind).unwrap();
                WebEntity::persist(query, kind, &self.state.cache.db);
                // Running the user input query at index zero
                if self.selected().is_none() {
                    self.pop_request(Request::Activate(0))
                        .expect("Unable to send pop-launcher request")
                } else {
                    // Re ask pop-launcher for a stored query
                    let command = self.current_entry().unwrap();
                    self.state.set_input(&command);
                    self.state.exec_on_next_search = true;
                    self.pop_request(Request::Search(command))
                        .expect("Unable to send pop-launcher request")
                }
            }
            ActiveMode::History => {
                let path = self.current_entry();
                let path = path.unwrap();
                self.run_command(path);
            }
            _ => {
                if self.selected().is_none() {
                    self.pop_request(Request::Activate(0))
                        .expect("Unable to send pop-launcher request")
                } else {
                    let selected = self.selected().unwrap() as u32;
                    debug!("Activating pop entry at index {selected}");
                    self.pop_request(Request::Activate(selected))
                        .expect("Unable to send pop-launcher request")
                }
            }
        }

        Command::none()
    }

    fn current_entries_len(&self) -> usize {
        match &self.state.get_active_mode() {
            ActiveMode::Plugin {
                plugin_name,
                history,
                ..
            } => {
                if *history {
                    self.state.cache.plugin_history_len(plugin_name)
                } else {
                    self.state.pop_search.len()
                }
            }
            ActiveMode::History => self.state.cache.de_len(),
            ActiveMode::DesktopEntry => self.state.pop_search.len(),
            ActiveMode::Web(web_name) => self.state.cache.web_history_len(web_name),
        }
    }

    fn pop_request(&self, request: Request) -> Result<(), TrySendError<Request>> {
        let sender = self.request_tx.as_ref().unwrap();
        let mut sender = sender.clone();
        debug!("Sending message to pop launcher : {:?}", request);
        sender.try_send(request)
    }

    fn selected(&self) -> Option<usize> {
        match self.state.selected {
            Selection::Reset => None,
            Selection::History(idx) | Selection::PopLauncher(idx) => Some(idx),
        }
    }

    fn dec_selected(&mut self) {
        match self.state.selected {
            Selection::Reset => self.state.selected = Selection::Reset,
            Selection::History(selected) => {
                if selected > 0 {
                    self.state.selected = Selection::History(selected - 1)
                }
            }
            Selection::PopLauncher(selected) => {
                if selected > 0 {
                    self.state.selected = Selection::PopLauncher(selected - 1)
                }
            }
        };
    }

    fn inc_selected(&mut self) {
        match self.state.selected {
            Selection::Reset => self.state.selected = Selection::History(0),
            Selection::History(selected) => {
                let total_items = self.current_entries_len();
                if total_items != 0 && selected < total_items - 1 {
                    self.state.selected = Selection::History(selected + 1);
                    self.snap();
                }
            }
            Selection::PopLauncher(selected) => {
                let total_items = self.current_entries_len();
                if total_items != 0 && selected < total_items - 1 {
                    self.state.selected = Selection::PopLauncher(selected + 1);
                    self.snap();
                }
            }
        };
    }

    fn keyboard_event() -> Subscription<Message> {
        iced_native::subscription::events_with(|event, _status| match event {
            Event::Window(iced_native::window::Event::Unfocused) => Some(Message::Unfocused),
            Event::Keyboard(iced::keyboard::Event::KeyPressed {
                modifiers: _,
                key_code,
            }) => Some(Message::KeyboardEvent(key_code)),
            _ => None,
        })
    }
}
