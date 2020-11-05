#[macro_use]
extern crate serde_derive;

mod config;
mod desktop;
mod style;
mod subscriptions;

use crate::desktop::{OnagreEntry, FileEntry};
use fuzzy_matcher::skim::SkimMatcherV2;

use iced::{Color, scrollable, text_input, Align, Application, Column, Command, Container, Element, HorizontalAlignment, Length, Row, Scrollable, Settings, Subscription, Text, TextInput, window};

use crate::style::{ContainerSelected, Theme};

use subscriptions::desktop_entries::DesktopEntryWalker;
use subscriptions::home_entries::HomeWalker;
use fuzzy_matcher::FuzzyMatcher;
use iced_native::Event;
use std::process::exit;
use std::rc::{Rc, Weak};
use crate::subscriptions::ToSubScription;

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
    desktop_entries: Vec<Rc<OnagreEntry>>,
    home_files: Vec<Rc<FileEntry>>,
    theme: style::Theme,
    state: State,
}

#[derive(Debug, Default)]
struct State {
    loading: bool,
    current_mode: usize,
    selected: usize,
    desktop_entry_matches: Vec<Weak<OnagreEntry>>,
    home_mathces: Vec<Weak<FileEntry>>,
    scroll: scrollable::State,
    input: text_input::State,
    input_value: String,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    DesktopEntryEvent(OnagreEntry),
    FileEntryEnvent(FileEntry),
    EventOccurred(iced_native::Event),
}

#[derive(Debug, Clone)]
enum Mode {
    Drun,
    XdgOpen
}

impl Application for Onagre {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        std::process::Command::new("swaymsg")
            .arg("for_window [app_id=\"Onagre\"] opacity 12")
            .output()
            .expect("not on sway");

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

        // By default the first entry is selected
        let selected = 0;
        let state = State {
            loading: true,
            current_mode: 0,
            selected,
            desktop_entry_matches: vec![],
            home_mathces: vec![],
            scroll: Default::default(),
            input: text_input::State::default(),
            input_value: "".to_string(),
        };

        (
            Onagre {
                modes: vec![Mode::Drun, Mode::XdgOpen],
                desktop_entries: vec![],
                home_files: vec![],
                theme: Theme,
                state,
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
                self.desktop_entries.push(Rc::new(entry));
                self.state.desktop_entry_matches = downgrade_all(&self.desktop_entries);
                Command::none()
            }
            Message::FileEntryEnvent(entry) => {
                self.home_files.push(Rc::new(entry));
                self.state.home_mathces = downgrade_all(&self.home_files);
                Command::none()
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        let event = iced_native::subscription::events().map(Message::EventOccurred);
        let desktop_entries = DesktopEntryWalker::subscription().map(Message::DesktopEntryEvent);
        // let home_entries = HomeWalker::subscription().map(Message::FileEntryEnvent);
        Subscription::batch(vec![event, desktop_entries ])
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let input = TextInput::new(
            &mut self.state.input,
            "Search",
            &self.state.input_value,
            Message::InputChanged,
        )
            .style(self.theme);

        let search_bar = Row::new().max_width(800).spacing(20).push(input);

        let mut buttons = Row::new();

        for (idx, mode) in self.modes.iter().enumerate() {
            if idx == self.state.current_mode {
                buttons = buttons
                    .push(
                        Container::new(
                            Text::new(mode.as_str()).horizontal_alignment(HorizontalAlignment::Left),
                        )
                        .style(ContainerSelected),
                    )
                    .spacing(10)
                    .padding(10)
            } else {
                buttons = buttons
                    .push(
                        Container::new(
                            Text::new(mode.as_str()).horizontal_alignment(HorizontalAlignment::Left),
                        )
                        .style(style::RowContainer),
                    )
                    .spacing(10)
                    .padding(10);
            };
        }

        let mut scrollable = Scrollable::new(&mut self.state.scroll)
            .style(Theme)
            .padding(40);

        for (idx, entry) in self.state.desktop_entry_matches.iter().enumerate() {
            let container = if idx == self.state.selected {
                Container::new(Row::new().push(
                    Text::new(&entry.upgrade().unwrap().as_ref().name)
                        .width(Length::Fill)
                        .horizontal_alignment(HorizontalAlignment::Left),
                )).style(ContainerSelected)
            } else {
                Container::new(Row::new().push(
                    Text::new(&entry.upgrade().unwrap().as_ref().name)
                        .width(Length::Fill)
                        .horizontal_alignment(HorizontalAlignment::Left),
                )).style(style::RowContainer)
            };

            scrollable = scrollable.push(container);
        }

        Container::new(
            Container::new(
                Column::new()
                    .push(buttons)
                    .push(search_bar.width(Length::Shrink)
                        .align_items(Align::Center)
                        .width(Length::FillPortion(1))
                        .padding(2)
                    )
                    .push(scrollable
                        .width(Length::FillPortion(1))
                    )
                    .align_items(Align::Start))
                .padding(20)
                .style(style::MainContainer)
        )
        .style(style::TransparentContainer)
            .padding(40)
            .into()
    }
}

impl Onagre {
    fn run_command(&self) {
        let selected = self.state.selected;
        let entry = self.state.desktop_entry_matches.get(selected).unwrap();
        let argv = shell_words::split(&entry.upgrade().unwrap().exec);

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

        exit(0);
    }

    fn update_matches(&self, input: &str) -> Vec<Weak<OnagreEntry>> {
        let matcher = SkimMatcherV2::default().ignore_case();

        self.desktop_entries
            .iter()
            .map(|entry| (entry, matcher.fuzzy_match(&entry.name, input).unwrap_or(0)))
            .filter(|(_, score)| *score > 10i64)
            .map(|(entry, _)| Rc::downgrade(entry))
            .collect()
    }

    fn handle_input(&mut self, event: iced_native::Event) -> Option<Message> {
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
                        if self.state.selected != self.state.desktop_entry_matches.len() - 1 {
                            self.state.selected += 1
                        }
                    }
                    KeyCode::Enter => self.run_command(),
                    KeyCode::Tab => self.cycle_mode(),
                    KeyCode::Backspace => {
                        self.reset_matches();
                    }
                    KeyCode::Escape => {
                        exit(0);
                    }
                    _ => {}
                }
            }
        }

        None
    }

    fn reset_matches(&mut self) {
        self.state.selected = 0;

        if self.state.input_value == "" {
            self.state.desktop_entry_matches = downgrade_all(&self.desktop_entries)
        } else {
            self.state.desktop_entry_matches = self.update_matches(&self.state.input_value)
        }
    }

    fn cycle_mode(&mut self) {
        if self.state.current_mode == self.modes.len() - 1 {
            self.state.current_mode = 0
        } else {
            self.state.current_mode += 1
        }
    }
}

fn downgrade_all<T>(vec_rc: &Vec<Rc<T>>) -> Vec<Weak<T>> {
    vec_rc
    .iter()
        .map(|entry| Rc::downgrade(&entry))
        .collect()
}

impl Mode {
    fn as_str(&self) -> &'static str {
        match &self {
            Mode::Drun => "Drun",
            Mode::XdgOpen => "XdgOpen"
        }
    }
}
