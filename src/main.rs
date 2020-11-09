#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate lazy_static;

mod desktop;
mod entries;
mod style;
mod subscriptions;

use crate::style::theme_settings::Theme;
use iced::{
    scrollable, text_input, window, Align, Application, Color, Column, Command, Container, Element,
    HorizontalAlignment, Length, Row, Scrollable, Settings, Subscription, Text, TextInput,
};

use crate::entries::{DesktopEntry, Entries, MatchedEntries};
use iced_native::Event;
use std::collections::HashMap;
use std::process::exit;
use subscriptions::custom::ExternalCommandSubscription;
use subscriptions::desktop_entries::DesktopEntryWalker;

lazy_static! {
    static ref THEME: Theme = Theme::load();
}

fn main() -> iced::Result {
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
    entries: Entries,
    state: State,
}

#[derive(Debug)]
struct State {
    loading: bool,
    mode_button_idx: usize,
    selected: usize,
    matches: MatchedEntries,
    scroll: scrollable::State,
    input: text_input::State,
    input_value: String,
}

impl Default for State {
    fn default() -> Self {
        let mut custom_entries = HashMap::new();
        custom_entries.insert("placeholder".to_string(), vec![]);

        State {
            loading: true,
            mode_button_idx: 0,
            selected: 0,
            matches: MatchedEntries {
                desktop_entries: vec![],
                custom_entries,
            },
            scroll: Default::default(),
            input: Default::default(),
            input_value: "".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    DesktopEntryEvent(DesktopEntry),
    CustomModeEvent(Vec<String>),
    EventOccurred(iced_native::Event),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Mode {
    Drun,
    Custom(&'static str),
}

impl Application for Onagre {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        Onagre::sway_preloads();
        let modes = vec![Mode::Drun, Mode::Custom("placeholder")];
        (
            Onagre {
                modes: modes.clone(),
                entries: Entries::new(modes.as_slice()),
                state: State::default(),
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
        // FIXME
        let _mode = if let Some(new_mode) = Mode::from_shorcut(&self.state.input_value) {
            println!(
                "Shortcut {} typed, moving to mode {}",
                self.state.input_value,
                new_mode.as_str()
            );
            self.set_current_mode(new_mode)
        } else {
            self.get_current_mode()
        };

        match message {
            Message::CustomModeEvent(entry) => {
                if let Some(entries) = self.entries.custom_entries.get_mut("placeholder") {
                    entries.extend(entry);
                }
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
                self.entries.desktop_entries.push(entry);
                Command::none()
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        let event = iced_native::subscription::events().map(Message::EventOccurred);
        let desktop_entries = DesktopEntryWalker::subscription().map(Message::DesktopEntryEvent);
        let files = ExternalCommandSubscription::subscription().map(Message::CustomModeEvent);
        let subscriptions = vec![event, desktop_entries, files];

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
                    .matches
                    .desktop_entries
                    .iter()
                    .enumerate()
                    .map(|(idx, entry)| {
                        if idx == self.state.selected {
                            Self::build_row_selected(&entry.name).into()
                        } else {
                            Self::build_row(&entry.name).into()
                        }
                    })
                    .collect();

                Column::with_children(rows)
            }
            Mode::Custom(name) => {
                let matches = self.state.matches.custom_entries.get(name);

                if let Some(matches) = matches {
                    let rows: Vec<Element<Message>> = matches
                        .iter()
                        .take(50)
                        .enumerate()
                        .map(|(idx, entry)| {
                            if idx == self.state.selected {
                                Self::build_row_selected(entry).into()
                            } else {
                                Self::build_row(entry).into()
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
        // Container::new(app_container)
        //     .style(TransparentContainer)
        //     .padding(40)
        //     .into()e
    }
}

impl Onagre {
    fn build_mode_menu(mode_idx: usize, modes: &[Mode]) -> Row<'_, Message> {
        let rows: Vec<Element<Message>> = modes
            .iter()
            .enumerate()
            .map(|(idx, mode)| {
                if idx == mode_idx {
                    Container::new(Text::new(mode.as_str()))
                        .style(&THEME.menu.lines.selected)
                        .width(THEME.menu.lines.selected.width.into())
                        .height(THEME.menu.lines.selected.height.into())
                        .padding(THEME.menu.lines.selected.padding)
                        .into()
                } else {
                    Container::new(Text::new(mode.as_str()))
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

    fn build_row<'a>(content: &str) -> Container<'a, Message> {
        Container::new(
            Row::new().push(
                Text::new(content)
                    .width(Length::Fill)
                    .horizontal_alignment(HorizontalAlignment::Left),
            ),
        )
        .width(THEME.rows.lines.default.width.into())
        .height(THEME.rows.lines.default.height.into())
        .style(&THEME.rows.lines.default)
        .padding(THEME.rows.lines.default.padding)
    }

    fn build_row_selected<'a>(content: &str) -> Container<'a, Message> {
        Container::new(
            Row::new().push(
                Text::new(content)
                    .width(Length::Fill)
                    .horizontal_alignment(HorizontalAlignment::Left),
            ),
        )
        .width(THEME.rows.lines.selected.width.into())
        .height(THEME.rows.lines.selected.height.into())
        .style(&THEME.rows.lines.selected)
        .padding(THEME.rows.lines.selected.padding)
    }

    fn run_command(&self) {
        match self.get_current_mode() {
            Mode::Drun => {
                let selected = self.state.selected;
                let entry = self.state.matches.desktop_entries.get(selected).unwrap();
                let argv = shell_words::split(&entry.exec);

                let argv = argv
                    .as_ref()
                    .unwrap()
                    .iter()
                    .filter(|entry| !entry.starts_with('%')) // FIXME: freedesktop entry spec
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
                    .matches
                    .custom_entries
                    .get(mode_name)
                    .unwrap()
                    .get(selected)
                    .unwrap();

                std::process::Command::new("xdg-open")
                    .arg(entry)
                    .spawn()
                    .expect("Command failure");
            }
        }
        exit(0);
    }

    fn handle_input(&mut self, event: iced_native::Event) {
        use iced_native::keyboard::KeyCode;

        // TODO : refactor
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
                            Mode::Drun => self.state.matches.desktop_entries.len(),
                            Mode::Custom(name) => {
                                self.state.matches.custom_entries.get(name).unwrap().len()
                            }
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
                    KeyCode::Backspace => self.reset_matches(),
                    KeyCode::Escape => {
                        exit(1);
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
                    self.set_desktop_matches(self.entries.take_50_desktop_entries());
                } else {
                    self.set_desktop_matches(self.entries.get_matches(&self.state.input_value));
                }
            }
            Mode::Custom(mode_name) => {
                if self.state.input_value == "" {
                    self.set_custom_matches(
                        mode_name,
                        self.entries.take_50_custom_entries(mode_name),
                    );
                } else {
                    self.set_custom_matches(
                        mode_name,
                        self.entries
                            .get_matches_custom_mode(mode_name, &self.state.input_value),
                    )
                }
            }
        }
    }

    fn cycle_mode(&mut self) {
        println!("{}/{}", self.state.mode_button_idx, self.modes.len());
        if self.state.mode_button_idx == self.modes.len() - 1 {
            println!("Changing mode {} -> 0", self.state.mode_button_idx);
            self.state.mode_button_idx = 0
        } else {
            println!(
                "Changing mode {} -> {}",
                self.state.mode_button_idx,
                self.state.mode_button_idx + 1
            );
            self.state.mode_button_idx += 1
        }
    }

    fn get_current_mode(&self) -> Mode {
        // Safe unwrap, we control the idx here
        let mode = self.modes.get(self.state.mode_button_idx).unwrap();
        *mode
    }

    fn set_current_mode(&mut self, mode: Mode) -> Mode {
        let new_mod_idx = match mode {
            Mode::Drun => 0 as usize,
            Mode::Custom(mode_name) => self
                .modes
                .iter()
                .position(|mode| mode_name == mode.as_str())
                .unwrap(),
        };

        self.state.mode_button_idx = new_mod_idx;
        mode
    }

    fn set_desktop_matches(&mut self, matches: Vec<DesktopEntry>) {
        self.state.matches.desktop_entries = matches;
    }

    fn set_custom_matches(&mut self, mode_key: &str, matches: Vec<String>) {
        self.state
            .matches
            .custom_entries
            .insert(mode_key.to_string(), matches);
    }
}

impl Mode {
    fn as_str(&self) -> &'static str {
        match &self {
            Mode::Drun => "Drun",
            Mode::Custom(name) => &name,
        }
    }

    fn from_shorcut(input: &str) -> Option<Mode> {
        if input.starts_with("fs") {
            Some(Mode::Custom("placeholder")) // TODO
        } else {
            None
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
