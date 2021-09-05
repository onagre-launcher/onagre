use iced::{scrollable, text_input, window, Align, Application, Color, Column, Command, Container, Element, Length, Row, Scrollable, Settings, Subscription, Text, TextInput, Clipboard};

use crate::entries::{Entries, EntriesState, Entry};
use crate::subscriptions::custom::ExternalCommandSubscription;
use crate::subscriptions::desktop_entries::DesktopEntryWalker;
use crate::SETTINGS;
use crate::THEME;
use fuzzy_matcher::skim::SkimMatcherV2;
use iced_native::keyboard::KeyCode;
use iced_native::Event;
use std::collections::{HashMap, HashSet};
use std::process::exit;
use core::fmt::Formatter;

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
    matcher: OnagreMatcher,
}

#[derive(Debug)]
struct State {
    // This is used to ensure we never unsubscribe to a mode command
    // we ensure the subscription command is never executed more than once
    mode_subs: HashSet<Mode>,
    current_mode_idx: usize,
    line_selected_idx: usize,
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
    fn new(startup_mode: Mode) -> Self {
        let mut mode_subs = HashSet::new();
        mode_subs.insert(startup_mode);

        State {
            mode_subs,
            current_mode_idx: 0,
            line_selected_idx: 0,
            entries: EntriesState::default(),
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
    KeyboardEvent(KeyCode),
    Loaded(HashMap<Mode, Vec<Entry>>),
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
                matcher: OnagreMatcher {
                    matcher: SkimMatcherV2::default().ignore_case(),
                },
            },
            Command::perform(
                crate::entries::cache::get_cached_entries(modes),
                Message::Loaded,
            ),
        )
    }

    fn title(&self) -> String {
        "Onagre".to_string()
    }

    fn update(&mut self, message: Self::Message, _clipboard: &mut Clipboard) -> Command<Self::Message> {
        self.state.input.focus();

        match message {
            Message::CustomModeEvent(new_entries) => {
                let current_mode = self.get_current_mode().clone();
                let entries = self
                    .state
                    .entries
                    .mode_entries
                    .get_mut(&current_mode)
                    .unwrap();

                let len = entries.len();
                entries.extend(new_entries);
                entries.sort();
                entries.dedup();

                if entries.len() != len {
                    self.reset_matches();
                }

                Command::none()
            }
            Message::InputChanged(input) => {
                self.state.input_value = input;
                self.reset_matches();
                self.reset_selection();
                Command::none()
            }
            Message::KeyboardEvent(event) => {
                self.handle_input(event);
                Command::none()
            }
            Message::DesktopEntryEvent(entry) => {
                let entries = self
                    .state
                    .entries
                    .mode_entries
                    .get_mut(&Mode::Drun)
                    .unwrap();

                if !entries.contains(&entry) {
                    entries.push(entry);
                    self.reset_matches();
                }

                Command::none()
            }
            Message::Loaded(entries) => {
                self.state.entries.mode_entries = entries;
                self.reset_matches();
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

        let mut subs = vec![keyboard_event];

        let mode_subs: Vec<Subscription<Message>> = self
            .state
            .mode_subs
            .iter()
            .cloned()
            .filter_map(Mode::into_subscription)
            .collect();

        subs.extend(mode_subs);

        Subscription::batch(subs)
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let mode_buttons: Row<Message> =
            Self::build_mode_menu(self.state.current_mode_idx, &self.modes);

        let current_mode = self.get_current_mode();
        let matches = self.state.entries.mode_matches.get(current_mode);

        // Build rows from current mode search entries
        let entries_column = if let Some(matches) = matches {
            let rows: Vec<Element<Message>> = matches
                .iter()
                .enumerate()
                .map(|(idx, entry)| {
                    if idx == self.state.line_selected_idx {
                        self.entry_by_idx(*entry).to_row_selected().into()
                    } else {
                        self.entry_by_idx(*entry).to_row().into()
                    }
                })
                .collect();

            Column::with_children(rows)
        } else {
            Column::new()
        };

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

    fn background_color(&self) -> Color {
        Color::TRANSPARENT
    }
}

impl Onagre {
    fn entry_by_idx(&self, idx: usize) -> &Entry {
        let mode = self.get_current_mode();
        self.state
            .entries
            .mode_entries
            .get(mode)
            .unwrap()
            .get(idx)
            .unwrap()
    }

    fn entry_mut_by_idx(&mut self, idx: usize) -> Option<&mut Entry> {
        let mode = self.get_current_mode().clone();
        self.state
            .entries
            .mode_entries
            .get_mut(&mode)
            .unwrap()
            .get_mut(idx)
    }

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

    fn run_command(&mut self) -> Command<Message> {
        let mode = self.get_current_mode().clone();
        let selected = self.state.line_selected_idx;

        let mode_entries = self.state.entries.mode_matches.get(&mode).unwrap();

        // Get the selected entry or fall back to user input for template/sourceless mode
        let current_entry: Option<&mut Entry> = if mode_entries.is_empty() {
            None
        } else {
            let current_entry_idx = *mode_entries.get(selected).unwrap();
            self.entry_mut_by_idx(current_entry_idx)
        };

        if let Some(entry) = current_entry {
            // This is the single mutable operation we have to do for entry
            entry.weight += 1;

            match mode {
                Mode::Drun => {
                    let argv = shell_words::split(&entry.exec.as_ref().unwrap());
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
                }
                Mode::Custom(mode_name) => {
                    let command = &SETTINGS.modes.get(&mode_name).unwrap().target;
                    let command = command.replace("%", &entry.display_name);
                    let args = shell_words::split(&command).unwrap();
                    let args = args.iter().collect::<Vec<&String>>();

                    std::process::Command::new(&args[0])
                        .args(&args[1..])
                        .spawn()
                        .expect("Command failure");
                }
            };
        } else {
            let input = &self.state.input_value;
            let command = &SETTINGS.modes.get(&mode.to_string()).unwrap().target;
            let command = command.replace("%", input);
            let args = shell_words::split(&command).unwrap();
            let args = args.iter().collect::<Vec<&String>>();
            let entries = self.state.entries.mode_entries.get_mut(&mode).unwrap();

            entries.push(Entry {
                weight: 1,
                display_name: input.clone(),
                exec: None,
                search_terms: None,
                icon: None,
            });

            std::process::Command::new(&args[0])
                .args(&args[1..])
                .spawn()
                .expect("Command failure");
        }

        self.flush_all();

        // Is this ok with iced or shall we exit with and internal command ?
        exit(0);
    }

    fn handle_input(&mut self, key_code: KeyCode) {
        debug!("Handle input : {:?}", key_code);
        match key_code {
            KeyCode::Up => {
                if self.state.line_selected_idx != 0 {
                    self.state.line_selected_idx -= 1
                }
            }
            KeyCode::Down => {
                let mode = self.get_current_mode();

                let max_idx = self.state.entries.mode_matches.get(mode).unwrap().len();

                if max_idx != 0 && self.state.line_selected_idx < max_idx - 1 {
                    self.state.line_selected_idx += 1
                }
            }
            KeyCode::Enter => {
                self.run_command();
            }
            KeyCode::Tab => {
                self.cycle_mode();
                let mode = self.get_current_mode().clone();
                let _ = self.state.mode_subs.insert(mode);
            }
            KeyCode::Escape => {
                self.flush_all();
                exit(0);
            }
            _ => {}
        }
    }

    fn reset_selection(&mut self) {
        debug!("reset selected line index to 0");
        self.state.line_selected_idx = 0;
    }

    fn reset_matches(&mut self) {
        let mode = self.get_current_mode().clone();
        if self.state.input_value.is_empty() {
            let matches = self
                .state
                .entries
                .mode_entries
                .get(&mode)
                .unwrap()
                .default_matches();

            self.set_custom_matches(mode, matches);
        } else {
            let matches = self
                .state
                .entries
                .mode_entries
                .get(&mode)
                .unwrap()
                .get_matches(&self.state.input_value, &self.matcher.matcher);

            self.set_custom_matches(mode, matches)
        }
    }

    fn cycle_mode(&mut self) {
        println!("{}/{}", self.state.current_mode_idx, self.modes.len());
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

    fn set_custom_matches(&mut self, mode: Mode, matches: Vec<usize>) {
        self.state.entries.mode_matches.insert(mode, matches);
    }

    fn flush_all(&mut self) {
        // This is really dirty but for now the only solution I see to not take the exclusive lock
        self.state
            .entries
            .mode_entries
            .iter()
            .for_each(|(mode, entries)| {
                let mut entries = entries.clone();
                entries.sort_unstable_by(|entry, other| other.weight.cmp(&entry.weight));
                crate::entries::cache::flush_mode_cache(mode, &entries);
            });
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

impl Mode {
    fn into_subscription(self) -> Option<Subscription<Message>> {
        match self {
            Mode::Drun => Some(DesktopEntryWalker::subscription().map(Message::DesktopEntryEvent)),
            Mode::Custom(name) => {
                SETTINGS
                    .modes
                    .get(&name)
                    .unwrap()
                    .source
                    .as_ref()
                    .map(|source| {
                        ExternalCommandSubscription::subscription(&source)
                            .map(Message::CustomModeEvent)
                    })
            }
        }
    }
}
