use std::collections::HashSet;
use std::path::PathBuf;
use std::process::exit;

use iced::futures::channel::mpsc::Sender;
use iced::{Application, Color, Column, Command, Container, Element, Length, Row, Scrollable, Settings, Subscription, Text, TextInput, scrollable, text_input, window, Alignment};
use iced_native::keyboard::KeyCode;
use iced_native::{Event, Clipboard};
use pop_launcher::Request;

use crate::backend::launcher::{PopLauncherSubscription, PopMessage};
use crate::backend::{PopResponse, PopSearchResult};
use crate::freedesktop::desktop::DesktopEntry;
use crate::SETTINGS;
use crate::THEME;

pub fn run(requested_modes: Vec<&str>, dmenu: bool) -> iced::Result {
    debug!("Starting Onagre in debug mode");
    debug!(
        "Settings : \n\tAvailable modes : {:#?}\n\t Icon theme : {:#?}",
        SETTINGS.modes, SETTINGS.icons
    );
    debug!(
        "Args : \n\tSelected modes : {:#?}\n\t dmenu : {:#?}",
        requested_modes, dmenu
    );

    // Custom modes from user settings
    let mut possible_modes: Vec<&str> = SETTINGS
        .modes
        .iter()
        .map(|(name, _)| name.as_str())
        .collect();

    // Merge custom mode from config and default modes to match user input args
    possible_modes.push("drun");
    // TODO : possible_modes.push("run");

    // match possible modes against user selection
    let mut modes = if requested_modes.is_empty() {
        possible_modes
            .iter()
            .map(|name| Mode::from(*name))
            .collect::<Vec<Mode>>()
    } else {
        possible_modes
            .iter()
            .filter(|name| requested_modes.contains(name))
            .map(|name| Mode::from(*name))
            .collect::<Vec<Mode>>()
    };

    // Keep user args in place (first mode provided is the default one)
    modes.reverse();

    debug!("Got modes {:?} from args", modes);

    Onagre::run(Settings {
        flags: modes,
        window: window::Settings {
            transparent: true,
            size: (800, 300),
            ..Default::default()
        },
        default_text_size: 20,
        antialiasing: true,
        ..Default::default()
    })
}

#[derive(Debug)]
struct Onagre {
    dmenu: bool,
    modes: Vec<Mode>,
    state: State,
    request_tx: Option<Sender<Request>>,
}

#[derive(Debug)]
struct State {
    // This is used to ensure we never unsubscribe to a mode command
    // we ensure the subscription command is never executed more than once
    mode_subs: HashSet<Mode>,
    current_mode_idx: usize,
    line_selected_idx: usize,
    entries: Vec<PopSearchResult>,
    scroll: scrollable::State,
    input: text_input::State,
    input_value: String,
}

impl State {
    fn new(startup_mode: Mode) -> Self {
        let mut mode_subs = HashSet::new();
        mode_subs.insert(startup_mode);

        State {
            mode_subs,
            current_mode_idx: 0,
            line_selected_idx: 0,
            entries: Vec::with_capacity(0),
            scroll: Default::default(),
            input: Default::default(),
            input_value: "".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    InputChanged(String),
    KeyboardEvent(KeyCode),
    PopSubscriptionResponse(PopMessage),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Mode {
    Drun,
    Custom(String),
}

impl From<&str> for Mode {
    fn from(name: &str) -> Self {
        match name {
            "drun" => Mode::Drun,
            other => Mode::Custom(other.to_string()),
        }
    }
}

impl Application for Onagre {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = Vec<Mode>;

    fn new(modes: Self::Flags) -> (Self, Command<Self::Message>) {
        Onagre::sway_preloads();

        (
            Onagre {
                dmenu: false,
                modes: modes.clone(),
                state: State::new(modes[0].clone()),
                request_tx: None,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Onagre".to_string()
    }

    fn update(
        &mut self,
        message: Self::Message,
    ) -> Command<Self::Message> {
        self.state.input.focus();

        match message {
            Message::InputChanged(input) => {
                self.state.input_value = input;
                debug!("Input changed");

                if let Some(sender) = &self.request_tx {
                    let mut sender = sender.clone();
                    let value = self.state.input_value.clone();
                    debug!("Sending message to pop thread : {}", value);
                    sender.try_send(Request::Search(value)).unwrap();
                }

                Command::none()
            }
            Message::KeyboardEvent(event) => {
                self.handle_input(event);
                Command::none()
            }
            Message::PopSubscriptionResponse(message) => {
                match message {
                    PopMessage::Ready(sender) => {
                        debug!("Subscription read, sender set");
                        self.request_tx = Some(sender);
                    }
                    PopMessage::Message(response) => match response {
                        PopResponse::Close => todo!(),
                        PopResponse::Context { .. } => todo!(),
                        PopResponse::DesktopEntry { path, .. } => {
                            self.run_command(path);
                        }
                        PopResponse::Update(search_updates) => {
                            self.state.entries = search_updates;
                        }
                        PopResponse::Fill(_) => todo!(),
                    },
                };
                Command::none()
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        let keyboard_event = iced_native::subscription::events_with(|event, _status| match event {
            Event::Keyboard(iced::keyboard::Event::KeyPressed {
                modifiers: _,
                key_code,
            }) => Some(Message::KeyboardEvent(key_code)),
            _ => None,
        });

        let subs = vec![
            keyboard_event,
            PopLauncherSubscription::subscription().map(Message::PopSubscriptionResponse),
        ];

        Subscription::batch(subs)
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let mode_buttons: Row<Message> =
            Self::build_mode_menu(self.state.current_mode_idx, &self.modes);

        // Build rows from current mode search entries
        let rows = self
            .state
            .entries
            .iter()
            .map(|entry| {
                if entry.id as usize == self.state.line_selected_idx {
                    entry.to_row_selected().into()
                } else {
                    entry.to_row().into()
                }
            })
            .collect();

        let entries_column = Column::with_children(rows);

        // Scrollable element containing the rows
        let scrollable = Container::new(
            Scrollable::new(&mut self.state.scroll)
                .push(entries_column)
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
                .align_items(Alignment::Center)
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
                .align_items(Alignment::Start)
                .height(Length::Fill)
                .width(Length::Fill)
                .padding(20),
        )
        .style(THEME.as_ref());

        app_container.into()
    }

    fn background_color(&self) -> Color {
        Color::TRANSPARENT
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

    fn run_command(&mut self, desktop_entry_path: PathBuf) -> Command<Message> {
        let desktop_entry = DesktopEntry::from_path(desktop_entry_path).unwrap();
        let argv = shell_words::split(&desktop_entry.exec);
        let args = argv.unwrap();
        let args = args
            .iter()
            // Filtering out special freedesktop syntax
            .filter(|entry| !entry.starts_with('%'))
            .collect::<Vec<&String>>();

        std::process::Command::new(&args[0])
            .args(&args[1..])
            .spawn()
            .expect("Command failure");

        exit(0);
    }

    fn handle_input(&mut self, key_code: KeyCode) {
        debug!("Handle input : {:?}", key_code);
        match key_code {
            KeyCode::Up => {
                if self.state.line_selected_idx != 0 {
                    self.state.line_selected_idx -= 1
                }
                self.snap();
            }
            KeyCode::Down => {
                let total_items = self.state.entries.len();
                if total_items != 0 && self.state.line_selected_idx < total_items - 1 {
                    self.state.line_selected_idx += 1
                }
                self.snap();
            }
            KeyCode::Enter => {
                if let Some(sender) = &self.request_tx {
                    let mut sender = sender.clone();
                    let selected = self.state.line_selected_idx;
                    sender.try_send(Request::Activate(selected as u32)).unwrap();
                }
            }
            KeyCode::Tab => {
                self.cycle_mode();
                let mode = self.get_current_mode().clone();
                let _ = self.state.mode_subs.insert(mode);
            }
            KeyCode::Escape => {
                exit(0);
            }
            _ => {}
        }
    }

    fn snap(&mut self) {
        let total_items = self.state.entries.len() as f32;

        let line_offset = if self.state.line_selected_idx == 0 {
            0
        } else {
            self.state.line_selected_idx + 1
        } as f32;

        let offset = (1.0 / total_items) * (line_offset) as f32;
        self.state.scroll.snap_to(offset);
    }

    fn cycle_mode(&mut self) {
        if self.state.current_mode_idx == self.modes.len() - 1 {
            debug!("Changing mode {} -> 0", self.state.current_mode_idx);
            self.state.current_mode_idx = 0
        } else {
            debug!(
                "Changing mode {} -> {}",
                self.state.current_mode_idx,
                self.state.current_mode_idx + 1
            );
            self.state.current_mode_idx += 1
        }
    }

    fn get_current_mode(&self) -> &Mode {
        // Safe unwrap, we control the idx here
        let mode = self.modes.get(self.state.current_mode_idx).unwrap();
        mode
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

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(0.2 * 5 as f32, 1.0);
    }
}
