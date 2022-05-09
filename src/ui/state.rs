use crate::db::Database;
use crate::entries::EntryCache;
use crate::ui::mode::ActiveMode;
use iced_native::widget::{scrollable, text_input};

#[derive(Debug)]
pub struct State {
    pub mode: ActiveMode,
    pub db: Database,
    pub line_selected_idx: Option<usize>,
    pub entries: EntryCache,
    pub scroll: scrollable::State,
    pub input: text_input::State,
    pub input_value: String,
    pub exec_on_next_search: bool,
}

impl Default for State {
    fn default() -> Self {
        State {
            mode: ActiveMode::History,
            db: Default::default(),
            line_selected_idx: Some(0),
            entries: EntryCache {
                pop_search: vec![],
                de_history: vec![],
                web_history: vec![],
                terminal: vec![],
            },
            scroll: Default::default(),
            input: Default::default(),
            input_value: "".to_string(),
            exec_on_next_search: false,
        }
    }
}
