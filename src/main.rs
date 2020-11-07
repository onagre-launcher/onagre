#[macro_use]
extern crate serde_derive;

mod config;
mod desktop;
mod entries;
mod style;
mod subscriptions;

use fuzzy_matcher::skim::SkimMatcherV2;

use crate::style::theme::{
    ContainerSelected, MainContainer, RowContainer, Theme, TransparentContainer,
};
use iced::{
    scrollable, text_input, window, Align, Application, Color, Column, Command, Container, Element,
    HorizontalAlignment, Length, Row, Scrollable, Settings, Subscription, Text, TextInput,
};

use crate::entries::{DesktopEntry, Entries, MatchedEntries};
use crate::subscriptions::ToSubScription;
use fuzzy_matcher::FuzzyMatcher;
use iced_native::Event;
use std::process::exit;
use std::rc::{Rc, Weak};
use subscriptions::desktop_entries::DesktopEntryWalker;

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
    theme: Theme,
    state: State,
}

#[derive(Debug, Default)]
struct State {
    loading: bool,
    mode_button_idx: usize,
    selected: usize,
    matches: MatchedEntries,
    scroll: scrollable::State,
    input: text_input::State,
    input_value: String,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    DesktopEntryEvent(DesktopEntry),
    EventOccurred(iced_native::Event),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Mode {
    Drun,
    XdgOpen,
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
            mode_button_idx: 0,
            selected,
            matches: Default::default(),
            scroll: Default::default(),
            input: Default::default(),
            input_value: "".to_string(),
        };

        (
            Onagre {
                entries: Default::default(),
                theme: Theme,
                modes: vec![Mode::Drun, Mode::XdgOpen],
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

        let mode = if let Some(new_mode) = Mode::from_shorcut(&self.state.input_value) {
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
                self.entries.desktop_entries.push(Rc::new(entry));
                self.state.matches.desktop_entries = downgrade_all(&self.entries.desktop_entries);
                Command::none()
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        let event = iced_native::subscription::events().map(Message::EventOccurred);
        let desktop_entries = DesktopEntryWalker::subscription().map(Message::DesktopEntryEvent);
        // let home_entries = HomeWalker::subscription().map(Message::FileEntryEnvent);
        Subscription::batch(vec![event, desktop_entries])
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let mut buttons: Row<Message> =
            Self::build_mode_menu(self.state.mode_button_idx, &self.modes);

        let entry_column = match self.get_current_mode() {
            Mode::Drun => {
                let rows: Vec<Element<Message>> = self
                    .state
                    .matches
                    .desktop_entries
                    .iter()
                    .enumerate()
                    .map(|(idx, entry)| {
                        let content = entry.upgrade().unwrap().name.clone();
                        if idx == self.state.selected {
                            Self::build_row_selected(content).into()
                        } else {
                            Self::build_row(content).into()
                        }
                    })
                    .collect();

                Column::with_children(rows)
            }
            Mode::XdgOpen => Column::new(),
        };

        let scrollable = Scrollable::new(&mut self.state.scroll)
            .style(Theme)
            .with_content(entry_column)
            .padding(40)
            .width(Length::FillPortion(1));

        let mode_menu = Row::new().push(buttons);

        let input = TextInput::new(
            &mut self.state.input,
            "Search",
            &self.state.input_value,
            Message::InputChanged,
        )
        .style(self.theme);

        let search_bar = Row::new()
            .max_width(800)
            .spacing(20)
            .width(Length::Shrink)
            .align_items(Align::Center)
            .width(Length::FillPortion(1))
            .padding(2)
            .push(input);

        let app_container = Container::new(
            Column::new()
                .push(mode_menu)
                .push(search_bar)
                .push(scrollable)
                .align_items(Align::Start),
        )
        .padding(20)
        .style(MainContainer);

        Container::new(app_container)
            .style(TransparentContainer)
            .padding(40)
            .into()
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
                        .style(ContainerSelected)
                        .into()
                } else {
                    Container::new(Text::new(mode.as_str()))
                        .style(RowContainer)
                        .into()
                }
            })
            .collect();

        Row::with_children(rows)
    }

    fn build_row<'a>(content: String) -> Container<'a, Message> {
        Container::new(
            Row::new().push(
                Text::new(content)
                    .width(Length::Fill)
                    .horizontal_alignment(HorizontalAlignment::Left),
            ),
        )
        .style(ContainerSelected)
    }

    fn build_row_selected<'a>(content: String) -> Container<'a, Message> {
        Container::new(
            Row::new().push(
                Text::new(content)
                    .width(Length::Fill)
                    .horizontal_alignment(HorizontalAlignment::Left),
            ),
        )
        .style(RowContainer)
    }

    fn run_command(&self) {
        let selected = self.state.selected;
        let entry = self.state.matches.desktop_entries.get(selected).unwrap();
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

    fn update_desktop_entries_match(&self, input: &str) -> Vec<Weak<DesktopEntry>> {
        let matcher = SkimMatcherV2::default().ignore_case();

        self.entries
            .desktop_entries
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
                        if self.state.selected != self.state.matches.desktop_entries.len() - 1 {
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
            self.state.matches.desktop_entries = downgrade_all(&self.entries.desktop_entries)
        } else {
            self.state.matches.desktop_entries =
                self.update_desktop_entries_match(&self.state.input_value)
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
            Mode::XdgOpen => 1 as usize,
        };

        self.state.mode_button_idx = new_mod_idx;
        mode
    }
}

fn downgrade_all<T>(vec_rc: &[Rc<T>]) -> Vec<Weak<T>> {
    vec_rc.iter().map(|entry| Rc::downgrade(&entry)).collect()
}

impl Mode {
    fn as_str(&self) -> &'static str {
        match &self {
            Mode::Drun => "Drun",
            Mode::XdgOpen => "XdgOpen",
        }
    }

    fn from_shorcut(input: &str) -> Option<Mode> {
        if input.starts_with("fs") {
            Some(Mode::XdgOpen)
        } else {
            None
        }
    }
}
