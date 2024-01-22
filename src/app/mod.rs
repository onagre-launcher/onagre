use std::path::Path;
use std::process::exit;

use iced::alignment::{Horizontal, Vertical};
use iced::futures::channel::mpsc::{Sender, TrySendError};
use iced::keyboard::KeyCode;
use iced::widget::{column, container, scrollable, text_input, Column, Container, Row, Text};
use iced::window::PlatformSpecific;
use iced::{
    subscription, window, Application, Command, Element, Length, Renderer, Settings, Subscription,
};
use iced_core::widget::operation::scrollable::RelativeOffset;
use iced_core::{Event, Font};
use iced_style::Theme;
use log::{debug, trace};
use onagre_launcher_toolkit::launcher::{Request, Response};
use once_cell::sync::Lazy;

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
use crate::freedesktop::desktop::DesktopEntry;
use crate::icons::IconPath;
use crate::THEME;

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
        .map(|font| Font::with_name(font))
        .unwrap_or_default();

    Onagre::run(Settings {
        id: Some("onagre".to_string()),
        window: window::Settings {
            transparent: true,
            size: THEME.size,
            decorations: false,
            resizable: false,
            position: window::Position::Centered,
            min_size: None,
            max_size: None,
            icon: None,
            visible: true,
            platform_specific: PlatformSpecific {
                application_id: "onagre".to_string(),
            },
            level: Default::default(),
        },
        default_text_size: THEME.font_size as f32,
        antialiasing: true,
        exit_on_close_request: false,
        default_font,
        flags: (),
    })
}

#[derive(Debug)]
pub struct Onagre<'a> {
    state: State<'a>,
    request_tx: Option<Sender<Request>>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Loading,
    InputChanged(String),
    KeyboardEvent(KeyCode),
    SubscriptionResponse(SubscriptionMessage),
    PluginConfig(Plugin),
    Unfocused,
}

static INPUT_ID: Lazy<text_input::Id> = Lazy::new(text_input::Id::unique);
static SCROLL_ID: Lazy<scrollable::Id> = Lazy::new(scrollable::Id::unique);

impl Application for Onagre<'_> {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;

    type Flags = ();

    fn new(_: Self::Flags) -> (Self, Command<Self::Message>) {
        let onagre = Onagre {
            state: Default::default(),
            request_tx: Default::default(),
        };

        (
            onagre,
            Command::perform(async {}, move |()| Message::Loading),
        )
    }

    fn title(&self) -> String {
        "Onagre".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Loading => text_input::focus(INPUT_ID.clone()),
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

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        // Build rows from current mode search entries
        let selected = self.selected();
        let rows = match &self.state.get_active_mode() {
            ActiveMode::Plugin {
                plugin_name,
                history,
                ..
            } if *history => {
                let icon = self.state.plugin_matchers.get_plugin_icon(plugin_name);
                self.state
                    .cache
                    .plugin_history(plugin_name)
                    .iter()
                    .enumerate()
                    .map(|(idx, entry)| entry.to_row(selected, idx, icon.as_ref()).into())
                    .collect()
            }
            ActiveMode::Web { modifier, .. } => {
                let icon = self.state.plugin_matchers.get_plugin_icon("web");
                self.state
                    .cache
                    .web_history(modifier)
                    .iter()
                    .enumerate()
                    .map(|(idx, entry)| entry.to_row(selected, idx, icon.as_ref()).into())
                    .collect()
            }
            ActiveMode::History => {
                let icon = self
                    .state
                    .plugin_matchers
                    .get_plugin_icon("desktop_entries");
                self.state
                    .cache
                    .de_history()
                    .iter()
                    .enumerate()
                    .map(|(idx, entry)| entry.to_row(selected, idx, icon.as_ref()).into())
                    .collect()
            }
            _ => self
                .state
                .pop_search
                .iter()
                .map(|entry| {
                    let icon = match &THEME.icon_theme {
                        Some(theme) => entry
                            .category_icon
                            .as_ref()
                            .and_then(|source| IconPath::from_source(source, theme)),
                        _ => None,
                    };

                    PopSearchResult(entry)
                        .to_row(selected, entry.id as usize, icon.as_ref())
                        .into()
                })
                .collect(),
        };

        // Scrollable element containing the rows
        let scrollable =
            scrollable(column(rows))
                .id(SCROLL_ID.clone())
                .style(iced::theme::Scrollable::Custom(Box::new(
                    THEME.scrollable(),
                )));

        let scrollable = container(scrollable)
            .style(iced::theme::Container::Custom(Box::new(
                &THEME.app_container.rows,
            )))
            .padding(THEME.app_container.rows.padding.to_iced_padding())
            .width(THEME.app_container.rows.width)
            .height(THEME.app_container.rows.height); // TODO: add this to stylesheet

        let text_input = text_input("Search", &self.state.input_value.input_display)
            .on_input(Message::InputChanged)
            .id(INPUT_ID.clone())
            .style(iced::theme::TextInput::Custom(Box::new(
                THEME.search_input(),
            )))
            .padding(THEME.search_input().padding.to_iced_padding())
            .width(THEME.search_input().text_width)
            .size(THEME.search_input().font_size);

        let search_input = container(text_input)
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
                .style(iced::theme::Container::Custom(Box::new(plugin_hint_style)))
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
            .style(iced::theme::Container::Custom(Box::new(THEME.search())))
            .align_x(THEME.search().align_x)
            .align_y(THEME.search().align_y)
            .padding(THEME.search().padding.to_iced_padding())
            .width(THEME.search().width)
            .height(THEME.search().height);

        let app_container = Container::new(
            Column::new()
                .push(search_bar)
                .push(scrollable)
                .align_items(iced_core::Alignment::Start),
        )
        .padding(THEME.app().padding.to_iced_padding())
        .style(iced::theme::Container::Custom(Box::new(THEME.app())))
        .center_y()
        .center_x();

        let app_wrapper = Container::new(app_container)
            .center_y()
            .center_x()
            .height(Length::Fill)
            .width(Length::Fill)
            .padding(THEME.padding.to_iced_padding())
            .style(iced::theme::Container::Custom(Box::new(&*THEME)));

        app_wrapper.into()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        let keyboard_event = Onagre::keyboard_event();
        let pop_launcher = PopLauncherSubscription::create().map(Message::SubscriptionResponse);
        let matchers = PluginMatcherSubscription::create().map(Message::PluginConfig);
        let subs = vec![keyboard_event, pop_launcher, matchers];
        iced::Subscription::batch(subs)
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
            ActiveMode::Plugin { plugin_name, .. } => {
                // Get user input as pop-entry
                match selected {
                    None => {
                        return self
                            .state
                            .pop_search
                            .first()
                            .map(|entry| entry.name.clone());
                    }
                    Some(selected) => self
                        .state
                        .cache
                        .plugin_history(plugin_name)
                        .get(selected)
                        .map(|entry| entry.query.to_string()),
                }
            }
            ActiveMode::Web { modifier, .. } => {
                // Get user input as pop-entry
                match selected {
                    None => {
                        return self
                            .state
                            .pop_search
                            .first()
                            .map(|entry| entry.name.clone());
                    }
                    Some(selected) => self
                        .state
                        .cache
                        .web_history(modifier)
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
            ActiveMode::Web { .. } | ActiveMode::History => Selection::Reset,
            ActiveMode::Plugin { history, .. } if *history => Selection::Reset,
            _ => Selection::PopLauncher(0),
        };

        let _: iced::Command<Message> =
            scrollable::snap_to(SCROLL_ID.clone(), RelativeOffset::START);

        match &self.state.get_active_mode() {
            ActiveMode::History => {}
            _ => {
                let value = self.state.get_input();

                self.pop_request(Request::Search(value))
                    .expect("Unable to send search request to pop-launcher")
            }
        }

        text_input::focus(INPUT_ID.clone())
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

        std::process::Command::new(args[0])
            .args(&args[1..])
            .spawn()
            .expect("Command failure");

        exit(0);
    }

    fn handle_input(&mut self, key_code: KeyCode) -> Command<Message> {
        match key_code {
            KeyCode::Up => {
                trace!("Selected line : {:?}", self.selected());
                return self.dec_selected();
            }
            KeyCode::Down => {
                trace!("Selected line : {:?}", self.selected());
                return self.inc_selected();
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

    fn snap(&mut self) -> Command<Message> {
        let total_items = self.current_entries_len() as f32;
        match self.selected() {
            None => scrollable::snap_to(SCROLL_ID.clone(), RelativeOffset::START),
            Some(selected) => {
                let offset = (1.0 / total_items) * selected as f32;
                scrollable::snap_to(SCROLL_ID.clone(), RelativeOffset { x: 0.0, y: offset })
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
                    let _ = self.run_command(path);
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
            self.state.input_value.input_display = fill;
            let _: iced::Command<Message> = text_input::move_cursor_to_end(INPUT_ID.clone());
            self.state.input_value.input_display.clone()
        } else {
            let mode_prefix = &self.state.input_value.modifier_display;
            let fill = fill
                .strip_prefix(mode_prefix)
                .expect("Auto-completion Error");
            self.state.input_value.input_display = fill.into();
            let _: iced::Command<Message> = text_input::move_cursor_to_end(INPUT_ID.clone());
            self.state.input_value.input_display.clone()
        };

        let _ = self.on_input_changed(filled);
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
            ActiveMode::Web { modifier, .. } => {
                let query = self.state.get_input();
                let query = query.strip_prefix(modifier).unwrap();
                WebEntity::persist(query, modifier, &self.state.cache.db);
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
                let _ = self.run_command(path);
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
            ActiveMode::Web { modifier, .. } => self.state.cache.web_history_len(modifier),
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

    fn dec_selected(&mut self) -> Command<Message> {
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

        self.snap()
    }

    fn inc_selected(&mut self) -> Command<Message> {
        match self.state.selected {
            Selection::Reset => self.state.selected = Selection::History(0),
            Selection::History(selected) => {
                let total_items = self.current_entries_len();
                if total_items != 0 && selected < total_items - 1 {
                    self.state.selected = Selection::History(selected + 1);
                }
            }
            Selection::PopLauncher(selected) => {
                let total_items = self.current_entries_len();
                if total_items != 0 && selected < total_items - 1 {
                    self.state.selected = Selection::PopLauncher(selected + 1);
                }
            }
        };

        self.snap()
    }

    fn keyboard_event() -> Subscription<Message> {
        subscription::events_with(|event, _status| match event {
            Event::Window(window::Event::Unfocused) => Some(Message::Unfocused),
            Event::Keyboard(iced::keyboard::Event::KeyPressed {
                modifiers: _,
                key_code,
            }) => Some(Message::KeyboardEvent(key_code)),
            _ => None,
        })
    }
}
