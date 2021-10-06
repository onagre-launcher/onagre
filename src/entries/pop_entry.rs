use core::option::Option::{None, Some};
use std::path::PathBuf;

use iced::alignment::Horizontal;
use iced::{Alignment, Container, Length, Row, Text};
use iced_native::widget::image::Image;
use iced_native::widget::svg::Svg;
use pop_launcher::{
    ContextOption, Generation, GpuPreference, IconSource, Indice, Response, SearchResult,
};

use crate::app::Message;
use crate::entries::AsEntry;
use crate::freedesktop::{Extension, IconPath};
use crate::{ICON_FINDER, SETTINGS};

#[derive(Debug, Clone)]
pub enum PopResponse {
    Close,
    Context {
        id: Indice,
        options: Vec<PopContextOption>,
    },
    DesktopEntry {
        path: PathBuf,
        gpu_preference: PopGpuPreference,
    },
    Update(Vec<PopSearchResult>),
    Fill(String),
}

#[derive(Debug, Clone)]
pub struct PopContextOption {
    pub id: Indice,
    pub name: String,
}

impl From<ContextOption> for PopContextOption {
    fn from(context: ContextOption) -> Self {
        Self {
            id: context.id,
            name: context.name,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PopSearchResult {
    pub id: Indice,
    pub name: String,
    pub description: String,
    pub icon: Option<IconSource>,
    pub category_icon: Option<IconSource>,
    pub window: Option<(Generation, Indice)>,
}

impl From<SearchResult> for PopSearchResult {
    fn from(result: SearchResult) -> Self {
        Self {
            id: result.id,
            name: result.name,
            description: result.description,
            icon: result.icon,
            category_icon: result.category_icon,
            window: result.window,
        }
    }
}

#[derive(Debug, Clone)]
pub enum PopGpuPreference {
    Default,
    NonDefault,
}

impl From<GpuPreference> for PopGpuPreference {
    fn from(pref: GpuPreference) -> Self {
        match pref {
            GpuPreference::Default => PopGpuPreference::Default,
            GpuPreference::NonDefault => PopGpuPreference::NonDefault,
        }
    }
}

impl From<Response> for PopResponse {
    fn from(response: Response) -> Self {
        match response {
            Response::Close => PopResponse::Close,
            Response::Context { id, options } => {
                let options = options.into_iter().map(PopContextOption::from).collect();
                PopResponse::Context { id, options }
            }
            Response::DesktopEntry {
                path,
                gpu_preference,
            } => PopResponse::DesktopEntry {
                gpu_preference: gpu_preference.into(),
                path,
            },
            Response::Update(updates) => {
                let updates = updates.into_iter().map(PopSearchResult::from).collect();
                PopResponse::Update(updates)
            }
            Response::Fill(fill) => PopResponse::Fill(fill),
        }
    }
}

impl<'a> AsEntry<'a> for PopSearchResult {
    fn as_row(&self) -> Container<'a, Message> {
        let icon_path = self
            .icon
            .as_ref()
            .or_else(|| self.category_icon.as_ref())
            .map(|icon| {
                ICON_FINDER.as_ref().map(|finder| {
                    let name = match icon {
                        IconSource::Name(icon) => icon,
                        IconSource::Mime(icon) => icon,
                        IconSource::Window(_) => todo!("What is this ?"),
                    };

                    let path = PathBuf::from(name.to_string());
                    if path.is_absolute() && path.exists() {
                        let extension = path.extension().unwrap().to_str().unwrap();
                        let extension = match extension {
                            "svg" => Some(Extension::Svg),
                            "png" => Some(Extension::Png),
                            _ => None,
                        };

                        extension.map(|extension| IconPath { path, extension })
                    } else {
                        finder.lookup(name, 48)
                    }
                })
            })
            .flatten()
            .flatten();

        let mut row = if let (Some(_icon_enabled), Some(icon_path)) = (&SETTINGS.icons, icon_path) {
            match &icon_path.extension {
                Extension::Svg => Row::new().push(
                    Svg::from_path(&icon_path.path)
                        .height(Length::Units(32))
                        .width(Length::Units(32)),
                ),

                Extension::Png => Row::new().push(
                    Image::new(&icon_path.path)
                        .height(Length::Units(32))
                        .width(Length::Units(32)),
                ),
            }
        } else {
            Row::new()
        };

        row = row
            .push(
                Text::new(&self.name)
                    .width(Length::Fill)
                    .horizontal_alignment(Horizontal::Left),
            )
            .spacing(10)
            .align_items(Alignment::Center);

        Container::new(row)
    }
}
