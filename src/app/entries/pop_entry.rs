use onagre_launcher_toolkit::launcher::SearchResult;
use std::borrow::Cow;

use crate::icons::IconPath;
use crate::THEME;

pub struct PopSearchResult<'a>(pub &'a SearchResult);
