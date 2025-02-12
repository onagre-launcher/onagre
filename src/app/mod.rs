use std::path::Path;
use std::process::exit;
use std::sync::Arc;

use entries::Entry;
use iced::alignment::{Horizontal, Vertical};
use iced::futures::channel::mpsc::{Sender, TrySendError};
use iced::keyboard::key::Named;
use iced::keyboard::Key;
use iced::widget::column;
use iced::widget::scrollable::{self, RelativeOffset};
use iced::widget::{container, text_input, Row, Text};
use iced::window::settings::PlatformSpecific;
use iced::{
    event, keyboard, window, Element, Event, Font, Length, Pixels, Settings, Size, Subscription,
    Task,
};
use onagre_launcher_toolkit::launcher::{IconSource, Request, Response};
use once_cell::sync::{Lazy, OnceCell};
use subscriptions::pop_launcher::pop_launcher;
use tracing::{debug, info, trace};
use widgets::row::theme::Class;
use widgets::row::to_scrollable;

use crate::app::entries::pop_entry::PopSearchResult;
use crate::app::mode::ActiveMode;
use crate::app::state::{Onagre, Selection};
use crate::db;
use crate::db::desktop_entry::DesktopEntryEntity;
use crate::db::plugin::PluginCommandEntity;
use crate::db::web::WebEntity;
use crate::freedesktop::desktop::DesktopEntry;

pub mod cache;
pub mod entries;
pub mod mode;
pub mod plugin_matchers;
pub mod state;
pub mod style;
pub mod subscriptions;
pub mod widgets;

#[derive(Debug, Clone)]
pub enum Message {
    Loading,
    InputChanged(String),
    Click(usize),
    KeyboardEvent(Key),
    PopLauncherReady(Sender<Request>),
    PopMessage(Response),
    Unfocused,
}

static INPUT_ID: Lazy<text_input::Id> = Lazy::new(text_input::Id::unique);
static SCROLL_ID: Lazy<scrollable::Id> = Lazy::new(scrollable::Id::unique);

#[derive(Clone, Debug)]
pub struct OnagreTheme(pub Arc<crate::Theme>);

impl Default for OnagreTheme {
    fn default() -> Self {
        unreachable!()
    }
}

static FONT: OnceCell<Option<String>> = OnceCell::new();

pub fn run(pre_value: Option<String>, theme: OnagreTheme) -> iced::Result {
    debug!("Starting Onagre in debug mode");
    let font = FONT.get_or_init(|| theme.0.font.clone());
    info!("using font {font:?}");
    let default_font = font
        .as_deref()
        .map(Font::with_name)
        .unwrap_or_else(|| Font::default());

    iced::application("Onagre", Onagre::update, Onagre::view)
        .decorations(false)
        .settings(Settings {
            id: Some("onagre".to_string()),
            default_text_size: Pixels::from(theme.0.font_size),
            antialiasing: true,
            default_font,
            fonts: vec![],
        })
        .window(window::Settings {
            transparent: true,
            size: Size {
                width: theme.0.size.0 as f32,
                height: theme.0.size.1 as f32,
            },
            decorations: false,
            resizable: false,
            position: window::Position::Centered,
            min_size: None,
            max_size: None,
            icon: None,
            visible: true,
            platform_specific: PlatformSpecific {
                application_id: "onagre".to_string(),
                override_redirect: true,
            },
            level: Default::default(),
            exit_on_close_request: true,
        })
        .subscription(subscription)
        .theme(Onagre::load_theme)
        .run_with(|| {
            let onagre = if let Some(pre) = pre_value {
                Onagre::with_mode(&pre, theme)
            } else {
                Onagre::new(theme)
            };
            (onagre, Task::perform(async {}, |_| Message::Loading))
        })
}

impl Onagre {
    fn view(&self) -> Element<Message, OnagreTheme> {
        // Build rows from current mode search entries
        let layout = &self.theme.0.app_container.rows.row;

        let scroll = to_scrollable(
            self.plugin_icon.clone(),
            layout,
            self.entries.as_slice(),
            self.selected().unwrap_or(0),
            self.get_theme().icon_theme.as_deref(),
        );

        let scrollable = container(scroll)
            .class(Class::Rows)
            .padding(
                self.get_theme()
                    .app_container
                    .rows
                    .padding
                    .to_iced_padding(),
            )
            .width(self.get_theme().app_container.rows.width)
            .height(self.get_theme().app_container.rows.height); // TODO: add this to stylesheet

        let text_input = text_input("Search", &self.input_value.input_display)
            .on_input(Message::InputChanged)
            .id(INPUT_ID.clone())
            // .style(&self.get_theme().search_input())
            .padding(self.get_theme().search_input().padding.to_iced_padding())
            .width(self.get_theme().search_input().text_width)
            .size(self.get_theme().search_input().font_size);

        let search_input = container(text_input)
            .width(self.get_theme().search_input().width)
            .height(self.get_theme().search_input().height)
            .align_x(self.get_theme().search_input().align_x)
            .align_y(self.get_theme().search_input().align_y)
            .class(Class::SearchInput);

        let search_bar = Row::new().width(Length::Fill).height(Length::Fill);
        // Either plugin_hint is enabled and we try to display it
        // Or we display the normal search input
        let search_bar = match self.get_theme().plugin_hint() {
            None => search_bar.push(search_input),
            Some(plugin_hint_style) => if !self.input_value.modifier_display.is_empty() {
                let plugin_hint = container(
                    Text::new(&self.input_value.modifier_display)
                        .align_y(Vertical::Center)
                        .align_x(Horizontal::Center)
                        .size(plugin_hint_style.font_size),
                )
                .class(Class::PluginHint)
                .width(plugin_hint_style.width)
                .height(plugin_hint_style.height)
                .align_y(plugin_hint_style.align_y)
                .align_x(plugin_hint_style.align_x)
                .padding(plugin_hint_style.padding.to_iced_padding());

                search_bar.push(plugin_hint).push(search_input)
            } else {
                search_bar.push(search_input)
            }
            .spacing(self.get_theme().search().spacing),
        };

        let search_bar = container(search_bar)
            .align_x(self.get_theme().search().align_x)
            .align_y(self.get_theme().search().align_y)
            .padding(self.get_theme().search().padding.to_iced_padding())
            .width(self.get_theme().search().width)
            .height(self.get_theme().search().height);

        let app_container = container(column![search_bar, scrollable])
            .padding(self.get_theme().app().padding.to_iced_padding())
            .class(Class::AppContainer)
            .center_y(Length::Fill)
            .center_x(Length::Fill);

        let app_wrapper = container(app_container)
            .center_y(Length::Fill)
            .center_x(Length::Fill)
            .height(Length::Fill)
            .width(Length::Fill)
            .padding(self.get_theme().padding.to_iced_padding())
            .class(Class::Main);

        app_wrapper.into()
    }

    fn update(&mut self, event: Message) -> Task<Message> {
        let message = match event {
            Message::Loading => text_input::focus(INPUT_ID.clone()),
            Message::InputChanged(input) => self.on_input_changed(input),
            Message::KeyboardEvent(event) => self.handle_input(event),
            Message::PopLauncherReady(sender) => {
                self.request_tx = Some(sender);
                Task::none()
            }
            Message::PopMessage(response) => {
                match response {
                    Response::Close => exit(0),
                    Response::Context { .. } => todo!("Discrete graphics is not implemented"),
                    Response::DesktopEntry { path, .. } => {
                        debug!("Launch DesktopEntry {path:?} via run_command");
                        let _ = self.run_command(path);
                    }
                    Response::Update(search_updates) => {
                        if self.exec_on_next_search {
                            debug!("Launch entry 0 via PopRequest::Activate");
                            self.pop_request(Request::Activate(0))
                                .expect("Unable to send Activate request to pop-launcher");
                            return Task::none();
                        }
                        self.pop_search = search_updates;
                    }
                    Response::Fill(fill) => self.complete(fill),
                };
                Task::none()
            }
            Message::Unfocused => {
                if self.get_theme().exit_unfocused {
                    exit(0);
                }
                Task::none()
            }
            Message::Click(row_idx) => {
                match self.get_active_mode() {
                    ActiveMode::History => self.selected = Selection::History(row_idx),
                    _ => self.selected = Selection::PopLauncher(row_idx),
                }

                self.on_execute()
            }
        };
        let (icon, entries) = match self.get_active_mode() {
            ActiveMode::Plugin {
                plugin_name,
                history,
                ..
            } if *history => (
                self.plugin_matchers.get_plugin_icon(plugin_name),
                self.cache
                    .plugin_history(plugin_name)
                    .iter()
                    .map(|entry| Box::new(entry.clone()) as Box<dyn Entry>)
                    .collect::<Vec<Box<dyn Entry>>>(),
            ),
            ActiveMode::Web { modifier, .. } => (
                self.plugin_matchers.get_plugin_icon("web"),
                self.cache
                    .web_history(modifier)
                    .iter()
                    .map(|entry| Box::new(entry.clone()) as Box<dyn Entry>)
                    .collect(),
            ),
            ActiveMode::History => (
                self.plugin_matchers.get_plugin_icon("desktop_entries"),
                self.cache
                    .de_history()
                    .iter()
                    .cloned()
                    .map(|entry| Box::new(entry.clone()) as Box<dyn Entry>)
                    .collect(),
            ),
            _ => (
                None,
                self.pop_search
                    .iter()
                    .cloned()
                    .map(|entry| Box::new(PopSearchResult(entry)) as Box<dyn Entry>)
                    .collect(),
            ),
        };

        self.plugin_icon = icon;
        self.entries = entries;

        message
    }
}

fn subscription(_: &Onagre) -> Subscription<Message> {
    let keyboard_event = keyboard_event();
    let pop_launcher = Subscription::run(pop_launcher);
    let subs = vec![keyboard_event, pop_launcher];
    Subscription::batch(subs)
}

impl Onagre {
    // Only call this if we are using entries from the database
    // in order to re-ask pop-launcher for the exact same entry
    fn current_entry(&self) -> Option<String> {
        let selected = self.selected();
        match &self.get_active_mode() {
            ActiveMode::History => self
                .cache
                .de_history()
                .get(selected.unwrap())
                .map(|entry| entry.path.to_string_lossy().to_string()),
            ActiveMode::Plugin { plugin_name, .. } => {
                // Get user input as pop-entry
                match selected {
                    None => self.pop_search.first().map(|entry| entry.name.clone()),
                    Some(selected) => self
                        .cache
                        .plugin_history(plugin_name)
                        .get(selected)
                        .map(|entry| entry.query.to_string()),
                }
            }
            ActiveMode::Web { modifier, .. } => {
                // Get user input as pop-entry
                match selected {
                    None => self.pop_search.first().map(|entry| entry.name.clone()),
                    Some(selected) => self
                        .cache
                        .web_history(modifier)
                        .get(selected)
                        .map(|entry| entry.query()),
                }
            }
            _pop_mode => None,
        }
    }

    fn on_input_changed(&mut self, input: String) -> Task<Message> {
        self.set_input(&input);
        self.selected = match self.get_active_mode() {
            // For those mode first line is unselected on change
            // We want to issue a pop-launcher search request to get the query at index 0 in
            // the next search response, then activate it
            ActiveMode::Web { .. } | ActiveMode::History => Selection::Reset,
            ActiveMode::Plugin { history, .. } if *history => Selection::Reset,
            _ => Selection::PopLauncher(0),
        };

        let _: Task<Message> = scrollable::snap_to(SCROLL_ID.clone(), RelativeOffset::START);

        match &self.get_active_mode() {
            ActiveMode::History => {}
            _ => {
                let value = self.get_input();

                self.pop_request(Request::Search(value))
                    .expect("Unable to send search request to pop-launcher")
            }
        }

        text_input::focus(INPUT_ID.clone())
    }

    fn run_command<P: AsRef<Path>>(&self, desktop_entry_path: P) -> Task<Message> {
        let desktop_entry = DesktopEntry::from_path(&desktop_entry_path).unwrap();

        DesktopEntryEntity::persist(
            &desktop_entry,
            desktop_entry_path.as_ref(),
            &self.cache.db,
            self.get_theme().icon_theme.as_deref(),
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

    fn handle_input(&mut self, key_code: Key) -> Task<Message> {
        match key_code {
            Key::Named(Named::ArrowUp) => {
                trace!("Selected line : {:?}", self.selected());
                return self.dec_selected();
            }
            Key::Named(Named::ArrowDown) => {
                trace!("Selected line : {:?}", self.selected());
                return self.inc_selected();
            }
            Key::Named(Named::Enter) => return self.on_execute(),
            Key::Named(Named::Tab) => {
                if let Some(selected) = self.selected() {
                    self.pop_request(Request::Complete(selected as u32))
                        .expect("Unable to send request to pop-launcher");
                }
            }
            Key::Named(Named::Escape) => {
                exit(0);
            }
            _ => {}
        };

        Task::none()
    }

    fn snap(&mut self) -> Task<Message> {
        let total_items = self.current_entries_len() as f32;
        match self.selected() {
            None => scrollable::snap_to(SCROLL_ID.clone(), RelativeOffset::START),
            Some(selected) => {
                let offset = (1.0 / total_items) * selected as f32;
                scrollable::snap_to(SCROLL_ID.clone(), RelativeOffset { x: 0.0, y: offset })
            }
        }
    }

    fn complete(&mut self, fill: String) {
        let filled = if self.get_theme().plugin_hint().is_none() {
            self.input_value.input_display = fill;
            let _: Task<Message> = text_input::move_cursor_to_end(INPUT_ID.clone());
            self.input_value.input_display.clone()
        } else {
            let mode_prefix = &self.input_value.modifier_display;
            let fill = fill
                .strip_prefix(mode_prefix)
                .expect("Auto-completion Error");
            self.input_value.input_display = fill.into();
            let _: Task<Message> = text_input::move_cursor_to_end(INPUT_ID.clone());
            self.input_value.input_display.clone()
        };

        let _ = self.on_input_changed(filled);
    }

    fn on_execute(&mut self) -> Task<Message> {
        match &self.get_active_mode() {
            ActiveMode::Plugin {
                plugin_name,
                history,
                ..
            } if *history => {
                PluginCommandEntity::persist(
                    plugin_name.as_str(),
                    &self.get_input(),
                    &self.cache.db,
                );

                // Running the user input query at index zero
                if self.selected().is_none() {
                    self.pop_request(Request::Activate(0))
                        .expect("Unable to send pop-launcher request")
                } else {
                    // Re ask pop-launcher for a stored query
                    self.exec_on_next_search = true;
                    let command = self.current_entry().unwrap();
                    self.set_input(&command);
                    self.pop_request(Request::Search(command))
                        .expect("Unable to send pop-launcher request");
                }
            }
            ActiveMode::Web { modifier, .. } => {
                let query = self.get_input();
                let query = query.strip_prefix(modifier).unwrap();
                let entry = &self.entries[self.selected().unwrap_or(0)];
                let icon = entry.get_icon().map(|i| match i {
                    IconSource::Name(i) | IconSource::Mime(i) => i,
                });
                WebEntity::persist(query, modifier.as_str(), icon, &self.cache.db);
                // Running the user input query at index zero
                if self.selected().is_none() {
                    self.pop_request(Request::Activate(0))
                        .expect("Unable to send pop-launcher request")
                } else {
                    // Re ask pop-launcher for a stored query
                    let command = self.current_entry().unwrap();
                    self.set_input(&command);
                    self.exec_on_next_search = true;
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

        Task::none()
    }

    fn current_entries_len(&self) -> usize {
        match &self.get_active_mode() {
            ActiveMode::Plugin {
                plugin_name,
                history,
                ..
            } => {
                if *history {
                    self.cache.history_len(plugin_name)
                } else {
                    self.pop_search.len()
                }
            }
            ActiveMode::History => self.cache.history_len(db::desktop_entry::COLLECTION),
            ActiveMode::DesktopEntry => self.pop_search.len(),
            ActiveMode::Web { modifier, .. } => self.cache.history_len(modifier),
        }
    }

    fn pop_request(&self, request: Request) -> Result<(), TrySendError<Request>> {
        let sender = self.request_tx.as_ref().unwrap();
        let mut sender = sender.clone();
        debug!("Sending message to pop launcher : {:?}", request);
        sender.try_send(request)
    }

    fn selected(&self) -> Option<usize> {
        match self.selected {
            Selection::Reset => None,
            Selection::History(idx) | Selection::PopLauncher(idx) => Some(idx),
        }
    }

    fn dec_selected(&mut self) -> Task<Message> {
        match self.selected {
            Selection::Reset => self.selected = Selection::Reset,
            Selection::History(selected) => {
                if selected > 0 {
                    self.selected = Selection::History(selected - 1)
                }
            }
            Selection::PopLauncher(selected) => {
                if selected > 0 {
                    self.selected = Selection::PopLauncher(selected - 1)
                }
            }
        };

        self.snap()
    }

    fn inc_selected(&mut self) -> Task<Message> {
        match self.selected {
            Selection::Reset => self.selected = Selection::History(0),
            Selection::History(selected) => {
                let total_items = self.current_entries_len();
                if total_items != 0 && selected < total_items - 1 {
                    self.selected = Selection::History(selected + 1);
                }
            }
            Selection::PopLauncher(selected) => {
                let total_items = self.current_entries_len();
                if total_items != 0 && selected < total_items - 1 {
                    self.selected = Selection::PopLauncher(selected + 1);
                }
            }
        };

        self.snap()
    }
}

fn keyboard_event() -> Subscription<Message> {
    event::listen_with(|event, _status, _id| match event {
        Event::Window(window::Event::Unfocused) => Some(Message::Unfocused),
        Event::Keyboard(keyboard::Event::KeyPressed { key, .. }) => {
            Some(Message::KeyboardEvent(key))
        }
        _ => None,
    })
}
