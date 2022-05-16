use crate::entries::Cache;
use crate::ui::mode::ActiveMode;
use crate::ui::plugin_matchers::{match_web_plugins, Plugin};
use iced_native::widget::{scrollable, text_input};
use log::debug;
use pop_launcher_toolkit::launcher::SearchResult;

use std::collections::HashMap;

// TODO: make this accessible for state binding only in ui via pub(super)
#[derive(Debug)]
pub struct State<'a> {
    pub input_value: SearchInput,
    pub selected: Selection,
    pub cache: Cache<'a>,
    pub pop_search: Vec<SearchResult>,
    pub scroll: scrollable::State,
    pub input: text_input::State,
    pub exec_on_next_search: bool,
    pub plugin_matchers: HashMap<String, Plugin>,
}

#[derive(Debug, Clone, Copy)]
pub enum Selection {
    // The selection is the content of the search bar, not something we got from pop-launcher
    // moving down will change selection to `History(0)`
    Reset,
    // This means we are trying to activate and item from the history
    // We need to issue a `Request::Search` before activating it.
    History(usize),
    // The selected item is one of the pop-launcher response items
    // It's safe to call `Request::Activate`.
    PopLauncher(usize),
}

impl State<'_> {
    pub fn get_active_mode(&self) -> &ActiveMode {
        &self.input_value.mode
    }

    pub fn get_input(&self) -> String {
        self.input_value.pop_query.clone()
    }

    // TODO : TEST ME
    pub fn set_input(&mut self, input: &str) {
        let previous_modi = self.input_value.modifier_display.clone();

        if !previous_modi.is_empty() {
            if input.is_empty() {
                self.input_value.modifier_display = "".to_string();
                self.input_value.input_display = previous_modi;
                self.input.move_cursor_to_end();
                self.input_value.mode = ActiveMode::DesktopEntry;
            } else {
                self.input_value.input_display = input.to_string();
            };
        } else {
            let terms = &format!("{}{}", previous_modi, input);
            let plugin_split = match_web_plugins(terms).or(self
                .plugin_matchers
                .values()
                .map(|matcher| matcher.try_match(terms))
                .find_map(|match_| match_));

            if let Some((modi, query)) = plugin_split {
                self.input_value.modifier_display = modi.modifier.clone();
                self.input_value.mode = ActiveMode::from(modi);
                self.input_value.input_display = query;
            } else {
                self.input_value.input_display = input.to_string();

                if input.is_empty() {
                    self.input_value.mode = ActiveMode::History
                } else {
                    self.input_value.mode = ActiveMode::DesktopEntry
                }
            }
        };

        let pop_query = match &self.input_value.mode {
            ActiveMode::History | ActiveMode::DesktopEntry => {
                self.input_value.input_display.clone()
            }
            ActiveMode::Web(modifier) => format!("{modifier} {}", self.input_value.input_display),
            ActiveMode::Plugin { modifier, .. } => {
                format!("{modifier}{}", self.input_value.input_display)
            }
        };

        self.input_value.pop_query = pop_query;
        debug!(
            "State: mode={:?}, input={}",
            self.input_value.mode, self.input_value.input_display
        );
    }
}

#[derive(Debug, Default)]
pub struct SearchInput {
    pub mode: ActiveMode,
    pub modifier_display: String,
    pub input_display: String,
    pub pop_query: String,
}

impl Default for State<'_> {
    fn default() -> Self {
        State {
            selected: Selection::History(0),
            cache: Default::default(),
            pop_search: Default::default(),
            scroll: Default::default(),
            input: Default::default(),
            input_value: SearchInput::default(),
            exec_on_next_search: false,
            plugin_matchers: HashMap::new(),
        }
    }
}
