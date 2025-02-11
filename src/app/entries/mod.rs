use crate::app::style::rows::icon::IconStyle;
use crate::app::style::rows::RowStyles;
use crate::app::Message;
use crate::icons::{fallback_icon, Extension, IconPath};
use crate::THEME;
use iced::widget::{column, container, row, text, Button, Container, Image, Row};
use iced::{Alignment, Length};
use std::borrow::Cow;

use super::style::rows::button::no_style;

pub mod entry2;
pub(crate) mod pop_entry;
