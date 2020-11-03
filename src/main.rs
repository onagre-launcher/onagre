#[macro_use]
extern crate serde_derive;

mod desktop;

use std::path::{PathBuf};

use async_std::fs;
use iced::futures::executor::block_on;
use iced::futures::StreamExt;
use crate::desktop::{DesktopEntry, OnagreEntry};
use fuzzy_matcher::skim::SkimMatcherV2;

use iced::{scrollable, text_input, Application, Column, Command, Container, Element, HorizontalAlignment, Length, Row, Scrollable, Settings, Text, TextInput, Subscription};
use fuzzy_matcher::FuzzyMatcher;
use iced_native::{Event};
use std::process::exit;

fn main() -> iced::Result {
    Onagre::run(Settings::default())
}

#[derive(Debug)]
struct Onagre {
    desktop_entries: Vec<OnagreEntry>,
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
        let desktop_entries: Vec<OnagreEntry> = block_on(get_all_app())
            .iter()
            .filter_map(|entry| entry.content.as_ref())
            .map(OnagreEntry::from)
            .collect();

        let matches: Vec<OnagreEntry> = desktop_entries.clone();

        let state = State {
            selected: 0,
            scroll: Default::default(),
            input: Default::default(),
            input_value: "".to_string(),
            matches,
        };

        (Onagre { desktop_entries, state }, Command::none())
    }

    fn title(&self) -> String {
        "Onagre".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::InputChanged(input) => {
                if self.state.input_value != input {
                    self.state.selected = 0;
                }

                self.state.input_value = input;

                if self.state.input_value == "" {
                    self.state.matches = self.desktop_entries.clone();
                } else {
                    self.state.matches = self.search(&self.state.input_value);
                }

                Command::none()
            }
            Message::EventOccurred(event) => {
                self.handle_hotkey(event);
                Command::none()
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        iced_native::subscription::events().map(Message::EventOccurred)
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let title = Text::new("Onagre")
            .width(Length::Fill)
            .size(100)
            .color([0.5, 0.5, 0.5])
            .horizontal_alignment(HorizontalAlignment::Center);

        let input = TextInput::new(
            &mut self.state.input,
            "Search",
            &mut self.state.input_value,
            Message::InputChanged,
        ).padding(15)
            .size(30);

        let content = Column::new()
            .max_width(800)
            .spacing(20)
            .push(title)
            .push(input);


        /* let rows: Vec<Row<Message>> = self.state.matches.iter()
             .map(|text| Row::new().push(Text::new(text)))
             .collect();*/

        // let mut text = vec![];

        // let rows = Row::with_children(text)
        //     .height(Length::Fill);


        let mut scrollable = Scrollable::new(&mut self.state.scroll)
            .padding(40)
            .push(Container::new(content));


        for (idx, entry) in self.state.matches.iter().enumerate() {
            let color = if idx == self.state.selected {
                [0.0, 1.0, 0.0]
            } else {
                [0.5, 0.5, 0.5]
            };

            scrollable = scrollable.push(Row::new()
                .push(Text::new(&entry.name).color(color).width(Length::Shrink))
            );
        }

        scrollable.into()
    }
}


impl Onagre {
    fn run_command(&self) {
        let selected = self.state.selected;
        println!("selected : {}", selected);
        let entry = self.state.matches.get(selected).unwrap();
        println!("entry : {:?}", entry);
        let argv = format!("\"{}\"", &entry.exec);
        println!("{}", argv);
        let err = std::process::Command::new("swaymsg")
            .arg("exec")
            .arg(argv)
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

    fn handle_hotkey(&mut self, event: iced_native::Event) -> Option<Message> {
        use iced_native::keyboard::KeyCode;

        match event {
            Event::Keyboard(keyboard_event) => {
                match keyboard_event {
                    iced_native::keyboard::Event::KeyPressed { key_code, .. } => {
                        match key_code {
                            KeyCode::Up => if self.state.selected != 0 { self.state.selected -= 1 },
                            KeyCode::Down => if self.state.selected != self.state.matches.len() - 1 { self.state.selected += 1 },
                            KeyCode::Enter => self.run_command(),
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