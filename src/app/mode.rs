#[derive(Debug, PartialEq, Clone)]
pub enum ActiveMode {
    Default(String),
    Plugin {
        plugin_name: String,
        modifier: String,
        query: String,
        history: bool,
        isolate: bool,
    },
}

impl ActiveMode {
    pub fn is_empty_query(&self) -> bool {
        match self {
            ActiveMode::Default(query) => query.is_empty(),
            ActiveMode::Plugin { query, .. } => query.trim().is_empty(),
        }
    }

    pub fn query(&self) -> &str {
        match self {
            ActiveMode::Default(query) => query,
            ActiveMode::Plugin { query, .. } => query,
        }
    }

    pub fn modifier(&self) -> Option<&str> {
        match self {
            ActiveMode::Default(_) => None,
            ActiveMode::Plugin { modifier, .. } => Some(modifier),
        }
    }

    pub fn pop_query(&self) -> String {
        match self {
            ActiveMode::Default(query) => query.clone(),
            ActiveMode::Plugin {
                modifier, query, ..
            } => format!("{modifier}{query}"),
        }
    }
}
