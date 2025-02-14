use std::path::Path;
use std::process::exit;
use std::sync::Arc;

use entries::Entry;
use iced::futures::channel::mpsc::{Sender, TrySendError};
use iced::keyboard::key::Named;
use iced::keyboard::Key;
use iced::widget::column;
use iced::widget::scrollable::{self, RelativeOffset};
use iced::widget::{container, text_input};
use iced::window::settings::PlatformSpecific;
use iced::{
    event, keyboard, window, Element, Event, Font, Length, Pixels, Settings, Size, Subscription,
    Task,
};
use onagre_launcher_toolkit::launcher::{IconSource, Request, Response};
use once_cell::sync::{Lazy, OnceCell};
use subscriptions::pop_launcher::pop_launcher;
use tracing::{debug, info, trace};
use widgets::entries::theme::Class;
use widgets::entries::to_scrollable;
use widgets::search::search_bar;

use crate::app::entries::pop_entry::PopSearchResult;
use crate::app::mode::ActiveMode;
use crate::app::state::Onagre;
use crate::db;
use crate::db::desktop_entry::{DesktopEntryEntity, DEFAULT_PLUGIN};
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
    let default_font = font.as_deref().map(Font::with_name).unwrap_or_default();

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
                Onagre::start_with_mode(&pre, theme)
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
            layout,
            self.entries.as_slice(),
            self.selected,
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

        let input_display = self.active_mode.query();
        let modifier = self.active_mode.modifier();
        debug!("{modifier:?}{input_display}");

        let search_bar = search_bar(
            INPUT_ID.clone(),
            input_display,
            modifier,
            self.get_theme().search(),
        );

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
                        let current = &self.entries[self.selected];
                        let _ = self.run_command(current.as_ref(), path);
                    }
                    Response::Update(search_updates) => {
                        if self.exec_on_next_search {
                            debug!("Launch entry 0 via PopRequest::Activate");
                            self.pop_request(Request::Activate(0))
                                .expect("Unable to send Activate request to pop-launcher");
                            return Task::none();
                        }
                        self.entries = search_updates
                            .iter()
                            .cloned()
                            .map(|entry| Box::new(PopSearchResult(entry)) as Box<dyn Entry>)
                            .collect();
                    }
                    Response::Fill(fill) => {
                        let _ = self.on_input_changed(fill);
                        let _: Task<Message> = text_input::move_cursor_to_end(INPUT_ID.clone());
                    }
                };
                Task::none()
            }
            Message::Unfocused => {
                if self.get_theme().exit_unfocused {
                    exit(0);
                }
                Task::none()
            }
            Message::Click(row_idx) => self.on_execute(),
        };

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
    fn on_input_changed(&mut self, input: String) -> Task<Message> {
        self.active_mode = self
            .plugin_matchers
            .get_active_mode(&input, &self.active_mode);
        let mode = &self.active_mode;
        match mode {
            // For those mode first line is unselected on change
            // We want to issue a pop-launcher search request to get the query at index 0 in
            // the next search response, then activate it
            ActiveMode::Plugin {
                history, isolate, ..
            } if !*isolate && *history && mode.query().is_empty() => {
                self.entries = self
                    .cache
                    .plugin_history(DEFAULT_PLUGIN)
                    .iter()
                    .map(|entry| Box::new(entry.clone()) as Box<dyn Entry>)
                    .collect::<Vec<Box<dyn Entry>>>();
            }
            ActiveMode::Plugin {
                plugin_name,
                history,
                isolate,
                ..
            } if *history && mode.is_empty_query() => {
                self.entries = self
                    .cache
                    .plugin_history(plugin_name)
                    .iter()
                    .map(|entry| Box::new(entry.clone()) as Box<dyn Entry>)
                    .collect::<Vec<Box<dyn Entry>>>()
            }
            _ => { /*pop entry already set*/ }
        };

        let _: Task<Message> = scrollable::snap_to(SCROLL_ID.clone(), RelativeOffset::START);

        if !mode.is_empty_query() {
            let query = mode.pop_query();

            self.pop_request(Request::Search(query))
                .expect("Unable to send search request to pop-launcher")
        }

        text_input::focus(INPUT_ID.clone())
    }

    fn run_command<P: AsRef<Path>>(
        &self,
        entry: &dyn Entry,
        desktop_entry_path: P,
    ) -> Task<Message> {
        let desktop_entry = DesktopEntry::from_path(&desktop_entry_path).unwrap();

        DesktopEntryEntity::persist(
            entry,
            &desktop_entry,
            desktop_entry_path.as_ref(),
            &self.cache.db
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
                trace!("Selected line : {:?}", self.selected);
                return self.dec_selected();
            }
            Key::Named(Named::ArrowDown) => {
                trace!("Selected line : {:?}", self.selected);
                return self.inc_selected();
            }
            Key::Named(Named::Enter) => return self.on_execute(),
            Key::Named(Named::Tab) => self
                .pop_request(Request::Complete(self.selected as u32))
                .expect("Unable to send request to pop-launcher"),
            Key::Named(Named::Escape) => {
                exit(0);
            }
            _ => {}
        };

        Task::none()
    }

    fn snap(&mut self) -> Task<Message> {
        let total_items = self.current_entries_len() as f32;
        let offset = (1.0 / total_items) * self.selected as f32;
        scrollable::snap_to(SCROLL_ID.clone(), RelativeOffset { x: 0.0, y: offset })
    }

    fn on_execute(&mut self) -> Task<Message> {
        let mode = &self.active_mode;

        match mode {
            ActiveMode::Plugin {
                plugin_name,
                history,
                isolate,
                ..
            } if *history && *isolate && mode.is_empty_query() => {
                let current = &self.entries[self.selected];
                DesktopEntryEntity::persist_with_mode(
                    current.as_ref(),
                    &mode.pop_query(),
                    &self.cache.db,
                );

                self.pop_request(Request::Activate(0))
                    .expect("Unable to send pop-launcher request")
            }
            mode => {
                debug!("Activation with mode {mode:?}");
                
                self
                    .pop_request(Request::Activate(0))
                    .expect("Unable to send pop-launcher request")
            },
        }

        Task::none()
    }

    fn current_entries_len(&self) -> usize {
        self.entries.len()
    }

    fn pop_request(&self, request: Request) -> Result<(), TrySendError<Request>> {
        let sender = self.request_tx.as_ref().unwrap();
        let mut sender = sender.clone();
        debug!("Sending message to pop launcher : {:?}", request);
        sender.try_send(request)
    }

    fn dec_selected(&mut self) -> Task<Message> {
        self.selected = self.selected.saturating_sub(1);
        self.snap()
    }

    fn inc_selected(&mut self) -> Task<Message> {
        if self.selected + 1 >= self.current_entries_len() {
            return Task::none();
        }

        self.selected += 1;
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
