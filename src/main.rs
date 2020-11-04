#[macro_use]
extern crate serde_derive;

mod desktop;
mod style;

use std::path::{PathBuf};

use async_std::fs;
use iced::futures::executor::block_on;
use iced::futures::StreamExt;
use crate::desktop::{DesktopEntry, OnagreEntry};
use fuzzy_matcher::skim::SkimMatcherV2;

use iced::{scrollable, text_input, Application, Column, Command, Element, HorizontalAlignment, Length, Row, Scrollable, Settings, Text, Subscription, Align, TextInput, Container};
use fuzzy_matcher::FuzzyMatcher;
use iced_native::{Event};
use std::process::exit;
use crate::style::{Theme};

fn main() -> iced::Result {
    Onagre::run(Settings::default())
}

#[derive(Debug)]
struct Onagre {
    desktop_entries: Vec<OnagreEntry>,
    theme: style::Theme,
    state: State,
}

#[derive(Debug, Default)]
struct State {
    selected: usize,
    scroll: scrollable::State,
    input: text_input::State,
    input_value: String,
    matches: Vec<OnagreEntry>,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    EventOccurred(iced_native::Event),
}

impl Application for Onagre {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
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

        //

        // Get xdg desktop entries and map them to our internal representation
        let desktop_entries: Vec<OnagreEntry> = block_on(get_all_app())
            .iter()
            .filter_map(|entry| entry.content.as_ref())
            .map(OnagreEntry::from)
            .collect();

        // All entries are displayed on startup
        let matches: Vec<OnagreEntry> = desktop_entries.clone();

        // By default the first entry is selected
        let selected = 0;
        let state = State {
            selected,
            matches,
            scroll: Default::default(),
            input: text_input::State::default(),
            input_value: "".to_string(),
        };

        (Onagre { desktop_entries, theme: Theme, state }, Command::none())
    }

    fn title(&self) -> String {
        "Onagre".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
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
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        iced_native::subscription::events().map(Message::EventOccurred)
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let state = &mut self.state.input;
        state.focus(true);

        let input = TextInput::new(
            state,
            "Search",
            &self.state.input_value,
            Message::InputChanged,
        )
            .style(self.theme);


        let search_bar = Column::new()
            .max_width(800)
            .spacing(20)
            .push(input);

        let mut scrollable = Scrollable::new(&mut self.state.scroll)
            .style(Theme)
            .padding(40);

        for (idx, entry) in self.state.matches.iter().enumerate() {
            let color = if idx == self.state.selected {
                [0.0, 1.0, 0.0]
            } else {
                [0.5, 0.5, 0.5]
            };

            scrollable = scrollable.push(Row::new()
                .push(Text::new(&entry.name)
                    .color(color)
                    .width(Length::Fill)
                    .horizontal_alignment(HorizontalAlignment::Center))
                .align_items(Align::Center)
            );
        }

        Container::new(Column::new().width(Length::Fill)
            .push(search_bar)
            .push(scrollable)
            .width(Length::Fill)
            .align_items(Align::Center))
            .style(self.theme)
            .height(Length::Fill)
            .width(Length::Fill)
            .into()
    }
}


impl Onagre {
    fn run_command(&self) {
        let selected = self.state.selected;
        println!("selected : {}", selected);
        let entry = self.state.matches.get(selected).unwrap();
        println!("entry : {:?}", entry);
        let argv = shell_words::split(&entry.exec);
        let argv = argv.as_ref().unwrap().as_slice();
        println!("{:?}", &argv);
        let err = std::process::Command::new(&argv[0])
            .args(&argv[1..])
            .output()
            .unwrap();

        println!("{:?}", err.status);
        println!("{:?}", String::from_utf8(err.stderr));
        println!("{:?}", String::from_utf8(err.stdout));


        exit(0);
    }

    fn search<'a>(&self, input: &str) -> Vec<OnagreEntry> {
        let matcher = SkimMatcherV2::default()
            .ignore_case();

        self.desktop_entries.iter()
            .map(|entry| (entry, matcher.fuzzy_match(&entry.name, input).unwrap_or(0)))
            .filter(|(_, score)| *score > 10i64)
            .map(|(entry, _)| entry.clone())
            .collect()
    }

    fn handle_input(&mut self, event: iced_native::Event) -> Option<Message> {
        use iced_native::keyboard::KeyCode;

        match event {
            Event::Keyboard(keyboard_event) => {
                match keyboard_event {
                    iced_native::keyboard::Event::KeyPressed { key_code, .. } => {
                        match key_code {
                            KeyCode::Up => if self.state.selected != 0 { self.state.selected -= 1 },
                            KeyCode::Down => if self.state.selected != self.state.matches.len() - 1 { self.state.selected += 1 },
                            KeyCode::Enter => self.run_command(),
                            KeyCode::Backspace => {
                                self.reset_matches();
                            }
                            KeyCode::Escape => {
                                exit(0);
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        None
    }

    fn reset_matches(&mut self) {
        self.state.selected = 0;

        if self.state.input_value == "" {
            self.state.matches = self.desktop_entries.clone();
        } else {
            self.state.matches = self.search(&self.state.input_value);
        }
    }
}

async fn get_all_app() -> Vec<DesktopEntry> {
    let mut apps = get_apps().await;
    let apps_local = get_apps_local().await;
    apps.extend(apps_local);
    apps
}

async fn get_apps() -> Vec<DesktopEntry> {
    let desktop_dir = PathBuf::from("/usr/share");
    println!("{:?}", desktop_dir);
    data_dirs(desktop_dir).await
}

async fn get_apps_local() -> Vec<DesktopEntry> {
    let desktop_dir = dirs::data_local_dir().unwrap();
    data_dirs(desktop_dir).await
}

async fn data_dirs(desktop_dir: PathBuf) -> Vec<DesktopEntry> {
    let mut desktop_entries = vec![];
    let mut entries = fs::read_dir(desktop_dir.join("applications")).await.unwrap();

    while let Some(res) = entries.next().await {
        let entry = res.unwrap();
        let desktop_entry = fs::read_to_string(entry.path()).await.unwrap();
        if let Ok(desktop_entry) = serde_ini::from_str(&desktop_entry) {
            desktop_entries.push(desktop_entry);
        }
    }

    desktop_entries
}