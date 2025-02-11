use std::path::Path;
use std::process::exit;

use iced::alignment::{Horizontal, Vertical};
use iced::futures::channel::mpsc::{Sender, TrySendError};
use iced::keyboard::key::Named;
use iced::keyboard::Key;
use iced::widget::scrollable::RelativeOffset;
use iced::widget::{column, container, scrollable, text_input, Column, Container, Row, Text};
use iced::window::settings::PlatformSpecific;
use iced::{
    event, keyboard, window, Element, Event, Font, Length, Pixels, Settings, Size, Subscription,
    Task,
};
use onagre_launcher_toolkit::launcher::{Request, Response};
use once_cell::sync::Lazy;
use style::scrollable::row_container_style;
use subscriptions::pop_launcher::pop_launcher;
use tracing::{debug, trace};
use widgets::row::LauncherEntry;

use crate::app::entries::pop_entry::PopSearchResult;
use crate::app::mode::ActiveMode;
use crate::app::state::{Onagre, Selection};
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

pub fn run(pre_value: Option<String>) -> iced::Result {
    debug!("Starting Onagre in debug mode");

    let default_font = THEME
        .font
        .as_ref()
        .map(|font| Font::with_name(font))
        .unwrap_or_default();

    iced::application("Onagre", Onagre::update, Onagre::view)
        .decorations(false)
        .settings(Settings {
            id: Some("onagre".to_string()),
            default_text_size: Pixels::from(THEME.font_size),
            antialiasing: true,
            default_font,
            fonts: vec![],
        })
        .window(window::Settings {
            transparent: true,
            size: Size {
                width: THEME.size.0 as f32,
                height: THEME.size.1 as f32,
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
        .run_with(|| {
            let onagre = if let Some(pre) = pre_value {
                Onagre::with_mode(&pre)
            } else {
                Onagre::default()
            };
            (onagre, Task::perform(async {}, |_| Message::Loading))
        })
}

impl Onagre {
    fn view(&self) -> Element<Message> {
        // Build rows from current mode search entries
        let selected = self.selected();
        let rows = match self.get_active_mode() {
            ActiveMode::Plugin {
                plugin_name,
                history,
                ..
            } if *history => {
                let icon = self.plugin_matchers.get_plugin_icon(plugin_name);
                self.cache
                    .plugin_history(plugin_name)
                    .iter()
                    .enumerate()
                    .map(|(idx, entry)| LauncherEntry::new(Box::new(entry), idx, selected))
                    .collect::<Vec<LauncherEntry>>()
            }
            ActiveMode::Web { modifier, .. } => {
                let icon = self.plugin_matchers.get_plugin_icon("web");
                self.cache
                    .web_history(modifier)
                    .iter()
                    .enumerate()
                    .map(|(idx, entry)| LauncherEntry::new(entry, idx, selected))
                    .collect()
            }
            ActiveMode::History => {
                let icon = self.plugin_matchers.get_plugin_icon("desktop_entries");
                self.cache
                    .de_history()
                    .iter()
                    .enumerate()
                    .map(|(idx, entry)| LauncherEntry::new(entry, idx, selected))
                    .collect()
            }
            _ => self
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
        let scrollable = scrollable(column(rows));
        // .id(self.clone())
        //  .style(&THEME.scrollable());

        let scrollable = container(scrollable)
            .style(row_container_style)
            .padding(THEME.app_container.rows.padding.to_iced_padding())
            .width(THEME.app_container.rows.width)
            .height(THEME.app_container.rows.height); // TODO: add this to stylesheet

        let text_input = text_input("Search", &self.input_value.input_display)
            .on_input(Message::InputChanged)
            .id(INPUT_ID.clone())
            // .style(&THEME.search_input())
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
            Some(plugin_hint_style) => if !self.input_value.modifier_display.is_empty() {
                let plugin_hint = Container::new(
                    Text::new(&self.input_value.modifier_display)
                        .align_y(Vertical::Center)
                        .align_x(Horizontal::Center)
                        .size(plugin_hint_style.font_size),
                )
                .style(|_| plugin_hint_style.into())
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
            .style(|_| THEME.search().into())
            .align_x(THEME.search().align_x)
            .align_y(THEME.search().align_y)
            .padding(THEME.search().padding.to_iced_padding())
            .width(THEME.search().width)
            .height(THEME.search().height);

        let app_container = Container::new(
            Column::new().push(search_bar).push(scrollable),
            //           .align_items(iced_core::Alignment::Start),
        )
        .padding(THEME.app().padding.to_iced_padding())
        //    .style(&THEME.app())
        .center_y(Length::Fill)
        .center_x(Length::Fill);

        let app_wrapper = Container::new(app_container)
            .center_y(Length::Fill)
            .center_x(Length::Fill)
            .height(Length::Fill)
            .width(Length::Fill)
            .padding(THEME.padding.to_iced_padding());
        // .style(|_| &*THEME);

        app_wrapper.into()
    }

    fn update(&mut self, event: Message) -> Task<Message> {
        match event {
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
                if THEME.exit_unfocused {
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
        }
    }
}
/* fn new(flags: OnagreFlags) -> (Self, Command<Self::Message>) {
        let onagre;
        if let Some(pre_value) = flags.pre_value {
            onagre = Onagre {
                state: State::with_mode(&pre_value),
                request_tx: Default::default(),
            };
        } else {
            onagre = Onagre {
                state: Default::default(),
                request_tx: Default::default(),
            };
        }

        (
            onagre,
            Command::perform(async {}, move |()| Message::Loading),
        )
    }

    fn title(&self) -> String {
        "Onagre".to_string()
    }
} */

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

        DesktopEntryEntity::persist(&desktop_entry, desktop_entry_path.as_ref(), &self.cache.db);

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
        let filled = if THEME.plugin_hint().is_none() {
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
                PluginCommandEntity::persist(plugin_name, &self.get_input(), &self.cache.db);

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
                WebEntity::persist(query, modifier, &self.cache.db);
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
                    self.cache.plugin_history_len(plugin_name)
                } else {
                    self.pop_search.len()
                }
            }
            ActiveMode::History => self.cache.de_len(),
            ActiveMode::DesktopEntry => self.pop_search.len(),
            ActiveMode::Web { modifier, .. } => self.cache.web_history_len(modifier),
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
