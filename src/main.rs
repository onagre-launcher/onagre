#[macro_use]
extern crate serde_derive;

mod config;
mod desktop;
mod dir_walk;
mod style;

use crate::desktop::OnagreEntry;
use fuzzy_matcher::skim::SkimMatcherV2;

use iced::{
    scrollable, text_input, Align, Application, Column, Command, Container, Element,
    HorizontalAlignment, Length, Row, Scrollable, Settings, Subscription, Text, TextInput,
};

use crate::style::{ContainerSelected, Theme};

use crate::dir_walk::FileWalker;
use fuzzy_matcher::FuzzyMatcher;
use iced_native::Event;
use std::process::exit;
use std::rc::{Rc, Weak};

fn main() -> iced::Result {
    Onagre::run(Settings::default())
}

#[derive(Debug)]
struct Onagre {
    modes: Vec<String>,
    entries: Vec<Rc<OnagreEntry>>,
    theme: style::Theme,
    state: State,
}

#[derive(Debug, Default)]
struct State {
    loading: bool,
    current_mode: usize,
    selected: usize,
    matches: Vec<Weak<OnagreEntry>>,
    scroll: scrollable::State,
    input: text_input::State,
    input_value: String,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    FileWalkEvent(OnagreEntry),
    EventOccurred(iced_native::Event),
}

impl Application for Onagre {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        println!("Initializing the app");
        // Tells sway to enable floating mode for Onagre
        std::process::Command::new("swaymsg")
            .arg("for_window [app_id=\"Onagre\"] floating enable")
            .output()
            .expect("not on sway");

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
            matches: vec![],
            scroll: Default::default(),
            input: text_input::State::default(),
            input_value: "".to_string(),
        };

        let mode = vec!["Drun".into(), "Open".into()];

        (
            Onagre {
                modes: mode,
                entries: vec![],
                theme: Theme,
                state,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Onagre".to_string()
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
            Message::FileWalkEvent(entry) => {
                self.entries.push(Rc::new(entry));
                self.state.matches = self.entries_as_refs();
                Command::none()
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        let event = iced_native::subscription::events().map(Message::EventOccurred);
        let file = FileWalker::to_subscription().map(Message::FileWalkEvent);
        Subscription::batch(vec![event, file])
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let input = TextInput::new(
            &mut self.state.input,
            "Search",
            &self.state.input_value,
            Message::InputChanged,
        )
        .style(self.theme);

        let search_bar = Column::new().max_width(800).spacing(20).push(input);

        let mut buttons = Row::new();

        for (idx, mode) in self.modes.iter().enumerate() {
            if idx == self.state.current_mode {
                buttons = buttons
                    .push(
                        Container::new(
                            Text::new(mode).horizontal_alignment(HorizontalAlignment::Left),
                        )
                        .style(ContainerSelected),
                    )
                    .spacing(10)
                    .padding(10)
            } else {
                buttons = buttons
                    .push(
                        Container::new(
                            Text::new(mode).horizontal_alignment(HorizontalAlignment::Left),
                        )
                        .style(style::Container),
                    )
                    .spacing(10)
                    .padding(10);
            };
        }

        let mut scrollable = Scrollable::new(&mut self.state.scroll)
            .style(Theme)
            .padding(40);

        for (idx, entry) in self.state.matches.iter().enumerate() {
            let color = if idx == self.state.selected {
                [0.0, 1.0, 0.0]
            } else {
                [0.5, 0.5, 0.5]
            };

            scrollable = scrollable.push(
                Row::new().push(
                    Text::new(&entry.upgrade().unwrap().as_ref().name)
                        .color(color)
                        .width(Length::Fill)
                        .horizontal_alignment(HorizontalAlignment::Left),
                ),
            );
        }

        Container::new(
            Column::new()
                .width(Length::Fill)
                .push(buttons)
                .push(search_bar)
                .push(scrollable)
                .width(Length::Fill)
                .align_items(Align::Center),
        )
        .style(self.theme)
        .height(Length::Fill)
        .width(Length::Fill)
        .into()
    }
}

impl Onagre {
    fn run_command(&self) {
        let selected = self.state.selected;
        let entry = self.state.matches.get(selected).unwrap();
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

    fn entries_as_refs(&mut self) -> Vec<Weak<OnagreEntry>> {
        self.entries
            .iter()
            .map(|entry| Rc::downgrade(&entry))
            .collect()
    }

    fn update_matches(&self, input: &str) -> Vec<Weak<OnagreEntry>> {
        let matcher = SkimMatcherV2::default().ignore_case();

        self.entries
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
                        if self.state.selected != self.state.matches.len() - 1 {
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
            self.state.matches = self.entries_as_refs()
        } else {
            self.state.matches = self.update_matches(&self.state.input_value)
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
