#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;

mod config;
pub mod entries;
mod freedesktop;
mod style;
mod subscriptions;

use iced::{
    scrollable, text_input, window, Align, Application, Color, Column, Command, Container, Element,
    Length, Row, Scrollable, Settings, Subscription, Text, TextInput,
};
use style::theme::Theme;

use crate::config::OnagreSettings;
use crate::entries::Entries;
use crate::entries::{EntriesState, Entry};
use fuzzy_matcher::skim::SkimMatcherV2;
use iced_native::Event;
use serde::export::Formatter;
use std::process::exit;
use std::rc::{Rc, Weak};
use subscriptions::custom::ExternalCommandSubscription;
use subscriptions::desktop_entries::DesktopEntryWalker;

lazy_static! {
    static ref THEME: Theme = Theme::load();
    pub static ref SETTINGS: OnagreSettings = OnagreSettings::get().unwrap_or_default();
}

fn main() -> iced::Result {
    env_logger::init();
    debug!("Starting Onagre in debug mode");

    Onagre::run(Settings {
        window: window::Settings {
            transparent: true,
            ..Default::default()
        },
        default_text_size: 20,
        antialiasing: true,
        ..Default::default()
    })
}

#[derive(Debug)]
struct Onagre {
    modes: Vec<Mode>,
    state: State,
    matcher: OnagreMatcher,
}

#[derive(Debug)]
struct State {
    loading: bool,
    mode_button_idx: usize,
    selected: usize,
    entries: EntriesState,
    scroll: scrollable::State,
    input: text_input::State,
    input_value: String,
}

struct OnagreMatcher {
    matcher: SkimMatcherV2,
}

impl std::fmt::Debug for OnagreMatcher {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "SkimMatcherV2")
    }
}

impl State {
    fn new(modes: &[Mode]) -> Self {
        State {
            loading: true,
            mode_button_idx: 0,
            selected: 0,
            entries: EntriesState::new(modes),
            scroll: Default::default(),
            input: Default::default(),
            input_value: "".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    InputChanged(String),
    DesktopEntryEvent(Entry),
    CustomModeEvent(Vec<Entry>),
    EventOccurred(iced_native::Event),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Mode {
    Drun,
    Custom(String),
}

impl Application for Onagre {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        Onagre::sway_preloads();

        let mut modes = vec![Mode::Drun];

        let custom_modes = SETTINGS
            .modes
            .keys()
            .map(|mode| mode.to_owned())
            .map(Mode::Custom);

        modes.extend(custom_modes);

        (
            Onagre {
                modes: modes.clone(),
                state: State::new(modes.as_slice()),
                matcher: OnagreMatcher {
                    matcher: SkimMatcherV2::default().ignore_case(),
                },
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Onagre".to_string()
    }

    fn background_color(&self) -> Color {
        Color::TRANSPARENT
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        self.state.input.focus(true);

        match message {
            Message::CustomModeEvent(entries) => {
                let new_entries: Vec<Rc<Entry>> = entries.into_iter().map(Rc::new).collect();

                let current_mode = self.get_current_mode().to_string();
                if let Some(entries) = self.state.entries.custom_entries.get_mut(&current_mode) {
                    entries.extend(new_entries);
                }
                self.reset_matches();
                Command::none()
            }
            Message::InputChanged(input) => {
                self.state.input_value = input;
                self.reset_matches();
                Command::none()
            }
            Message::EventOccurred(event) => {
                self.handle_input(event);
                Command::none()
            }
            Message::DesktopEntryEvent(entry) => {
                self.state.entries.desktop_entries.push(Rc::new(entry));
                self.reset_matches();
                Command::none()
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        let event = iced_native::subscription::events().map(Message::EventOccurred);
        let desktop_entries = DesktopEntryWalker::subscription().map(Message::DesktopEntryEvent);

        let mut subscriptions = vec![event, desktop_entries];
        if let Mode::Custom(name) = self.get_current_mode() {
            let command = &SETTINGS.modes.get(name).unwrap().source;
            subscriptions.push(
                ExternalCommandSubscription::subscription(command).map(Message::CustomModeEvent),
            );
        }

        Subscription::batch(subscriptions)
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let mode_buttons: Row<Message> =
            Self::build_mode_menu(self.state.mode_button_idx, &self.modes);

        // Build rows from current mode search entries
        let entry_column = match self.get_current_mode() {
            Mode::Drun => {
                let rows: Vec<Element<Message>> = self
                    .state
                    .entries
                    .desktop_entries_matches
                    .iter()
                    .enumerate()
                    .map(|(idx, entry)| {
                        if idx == self.state.selected {
                            entry.upgrade().unwrap().to_row_selected().into()
                        } else {
                            entry.upgrade().unwrap().to_row().into()
                        }
                    })
                    .collect();

                Column::with_children(rows)
            }
            Mode::Custom(name) => {
                let matches = self.state.entries.custom_entries_matches.get(name);

                if let Some(matches) = matches {
                    let rows: Vec<Element<Message>> = matches
                        .iter()
                        .take(50)
                        .enumerate()
                        .map(|(idx, entry)| {
                            if idx == self.state.selected {
                                entry.upgrade().unwrap().to_row_selected().into()
                            } else {
                                entry.upgrade().unwrap().to_row().into()
                            }
                        })
                        .collect();

                    Column::with_children(rows)
                } else {
                    Column::new()
                }
            }
        };

        // Scrollable element containing the rows
        let scrollable = Container::new(
            Scrollable::new(&mut self.state.scroll)
                .with_content(entry_column)
                .height(THEME.scrollable.height.into())
                .width(THEME.scrollable.width.into())
                .scrollbar_width(THEME.scrollable.scroller_width)
                .scroller_width(THEME.scrollable.scrollbar_width)
                .style(&THEME.scrollable),
        )
        .style(&THEME.scrollable)
        .padding(THEME.scrollable.padding);

        // Switch mode menu
        let mode_menu = Container::new(
            Row::new()
                .push(mode_buttons)
                .height(THEME.menu.width.into())
                .width(THEME.menu.height.into()),
        )
        .padding(THEME.menu.padding)
        .style(&THEME.menu);

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
                .align_items(Align::Center)
                .padding(2)
                .push(search_input)
                .width(THEME.search.width.into())
                .height(THEME.search.height.into()),
        )
        .padding(THEME.search.padding)
        .style(&THEME.search);

        let app_container = Container::new(
            Column::new()
                .push(mode_menu)
                .push(search_bar)
                .push(scrollable)
                .align_items(Align::Start)
                .height(Length::Fill)
                .width(Length::Fill)
                .padding(20),
        )
        .style(THEME.as_ref());

        app_container.into()
    }
}

impl Onagre {
    fn build_mode_menu(mode_idx: usize, modes: &[Mode]) -> Row<'_, Message> {
        let rows: Vec<Element<Message>> = modes
            .iter()
            .enumerate()
            .map(|(idx, mode)| {
                if idx == mode_idx {
                    Container::new(Text::new(mode.to_string()))
                        .style(&THEME.menu.lines.selected)
                        .width(THEME.menu.lines.selected.width.into())
                        .height(THEME.menu.lines.selected.height.into())
                        .padding(THEME.menu.lines.selected.padding)
                        .into()
                } else {
                    Container::new(Text::new(mode.to_string()))
                        .style(&THEME.menu.lines.default)
                        .width(THEME.menu.lines.default.width.into())
                        .height(THEME.menu.lines.default.height.into())
                        .padding(THEME.menu.lines.default.padding)
                        .into()
                }
            })
            .collect();

        Row::with_children(rows)
    }

    fn run_command(&self) -> Command<Message> {
        match self.get_current_mode() {
            Mode::Drun => {
                let selected = self.state.selected;
                let entry = self
                    .state
                    .entries
                    .desktop_entries_matches
                    .get(selected)
                    .unwrap();

                let entry_ref = entry.upgrade().unwrap();
                let options = entry_ref.options.as_ref().unwrap();
                let argv = shell_words::split(&options.exec);

                let argv = argv
                    .as_ref()
                    .unwrap()
                    .iter()
                    // Filtering out special freedesktop syntax
                    .filter(|entry| !entry.starts_with('%'))
                    .collect::<Vec<&String>>();

                std::process::Command::new(&argv[0])
                    .args(&argv[1..])
                    .spawn()
                    .expect("Command failure");
            }
            Mode::Custom(mode_name) => {
                let selected = self.state.selected;
                let entry = self
                    .state
                    .entries
                    .custom_entries_matches
                    .get(mode_name)
                    .unwrap()
                    .get(selected)
                    .unwrap();

                let command = &SETTINGS.modes.get(mode_name).unwrap().target;
                let command = command.replace("%", &entry.upgrade().unwrap().display_name);
                let argv = shell_words::split(&command).unwrap();

                std::process::Command::new(&argv[0])
                    .args(&argv[1..])
                    .spawn()
                    .expect("Command failure");
            }
        }
        exit(0);
    }

    fn handle_input(&mut self, event: iced_native::Event) {
        use iced_native::keyboard::KeyCode;

        if let Event::Keyboard(keyboard_event) = event {
            if let iced_native::keyboard::Event::KeyPressed { key_code, .. } = keyboard_event {
                match key_code {
                    KeyCode::Up => {
                        if self.state.selected != 0 {
                            self.state.selected -= 1
                        }
                    }
                    KeyCode::Down => {
                        let max_idx = match self.get_current_mode() {
                            Mode::Drun => self.state.entries.desktop_entries_matches.len(),
                            Mode::Custom(name) => self
                                .state
                                .entries
                                .custom_entries_matches
                                .get(name)
                                .unwrap()
                                .len(),
                        };

                        if max_idx != 0 && self.state.selected < max_idx - 1 {
                            self.state.selected += 1
                        }
                    }
                    KeyCode::Enter => {
                        self.run_command();
                    }
                    KeyCode::Tab => {
                        self.cycle_mode();
                    }
                    KeyCode::Escape => {
                        exit(0);
                    }
                    _ => {}
                }
            }
        }
    }

    fn reset_matches(&mut self) {
        self.state.selected = 0;

        match self.get_current_mode() {
            Mode::Drun => {
                if self.state.input_value == "" {
                    self.set_desktop_matches(self.state.entries.desktop_entries.default_matches());
                } else {
                    self.set_desktop_matches(
                        self.state
                            .entries
                            .desktop_entries
                            .get_matches(&self.state.input_value, &self.matcher.matcher),
                    );
                }
            }
            Mode::Custom(mode_name) => {
                let mode_name = mode_name.clone();
                if self.state.input_value == "" {
                    self.set_custom_matches(
                        &mode_name,
                        self.state
                            .entries
                            .get_mode_entries(&mode_name)
                            .default_matches(),
                    );
                } else {
                    self.set_custom_matches(
                        &mode_name,
                        self.state
                            .entries
                            .get_mode_entries(&mode_name)
                            .get_matches(&self.state.input_value, &self.matcher.matcher),
                    )
                }
            }
        }
    }

    fn cycle_mode(&mut self) {
        println!("{}/{}", self.state.mode_button_idx, self.modes.len());
        if self.state.mode_button_idx == self.modes.len() - 1 {
            debug!("Changing mode {} -> 0", self.state.mode_button_idx);
            self.state.mode_button_idx = 0
        } else {
            debug!(
                "Changing mode {} -> {}",
                self.state.mode_button_idx,
                self.state.mode_button_idx + 1
            );
            self.state.mode_button_idx += 1
        }
    }

    fn get_current_mode(&self) -> &Mode {
        // Safe unwrap, we control the idx here
        let mode = self.modes.get(self.state.mode_button_idx).unwrap();
        mode
    }

    fn set_desktop_matches(&mut self, matches: Vec<Weak<Entry>>) {
        self.state.entries.desktop_entries_matches = matches;
    }

    fn set_custom_matches(&mut self, mode_key: &str, matches: Vec<Weak<Entry>>) {
        self.state
            .entries
            .custom_entries_matches
            .insert(mode_key.to_string(), matches);
    }
}

impl ToString for Mode {
    fn to_string(&self) -> String {
        match &self {
            Mode::Drun => "Drun".to_string(),
            Mode::Custom(name) => name.clone(),
        }
    }
}

impl Onagre {
    fn sway_preloads() {
        // Tell sway to enable floating mode for Onagre
        std::process::Command::new("swaymsg")
            .arg("for_window [app_id=\"Onagre\"] floating enable")
            .output()
            .expect("not on sway");

        // [set|plus|minus] <value>
        // Tells sway to focus on startup
        std::process::Command::new("swaymsg")
            .arg("[app_id=\"Onagre\"] focus")
            .output()
            .expect("not on sway");

        // Tells sway to remove borders on startup
        std::process::Command::new("swaymsg")
            .arg("for_window [app_id=\"Onagre\"] border none ")
            .output()
            .expect("not on sway");

        // Tells sway to remove borders on startup
        std::process::Command::new("swaymsg")
            .arg("for_window [app_id=\"Onagre\"] resize set width 45 ppt height  35 ppt")
            .output()
            .expect("not on sway");
    }
}
