// Copyright 2022 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0
use super::entries::theme::Class;
use crate::app::OnagreTheme;
use derive_setters::Setters;
use iced::Rotation;
use iced::{ContentFit, Length, Rectangle};
use std::ffi::OsStr;

mod handle;
mod named;
mod theme;

pub use handle::Handle;
pub use named::{IconFallback, Named};

/// Create an [`Icon`] from a pre-existing [`Handle`]
pub fn icon(handle: Handle, selected: bool) -> Icon {
    Icon {
        content_fit: ContentFit::Fill,
        handle,
        height: None,
        size: 16,
        class: Class::Icon { selected },
        rotation: None,
        width: None,
    }
}

/// An image which may be an SVG or PNG.
#[must_use]
#[derive(Clone, Setters)]
pub struct Icon {
    #[setters(skip)]
    handle: Handle,
    class: Class,
    pub(super) size: u16,
    content_fit: ContentFit,
    #[setters(strip_option)]
    width: Option<Length>,
    #[setters(strip_option)]
    height: Option<Length>,
    #[setters(strip_option)]
    rotation: Option<Rotation>,
}

impl Icon {
    #[must_use]
    fn view<'a, Message: 'a>(self) -> iced::Element<'a, Message, OnagreTheme> {
        let from_image = |handle: &iced::widget::image::Handle| {
            iced::widget::Image::new(handle)
                .width(
                    self.width
                        .unwrap_or_else(|| Length::Fixed(f32::from(self.size))),
                )
                .height(
                    self.height
                        .unwrap_or_else(|| Length::Fixed(f32::from(self.size))),
                )
                .rotation(self.rotation.unwrap_or_default())
                .content_fit(self.content_fit)
                .into()
        };

        let from_svg = |handle: &iced::widget::svg::Handle| {
            iced::widget::svg::Svg::<OnagreTheme>::new(handle.clone())
                .width(
                    self.width
                        .unwrap_or_else(|| Length::Fixed(f32::from(self.size))),
                )
                .height(
                    self.height
                        .unwrap_or_else(|| Length::Fixed(f32::from(self.size))),
                )
                .rotation(self.rotation.unwrap_or_default())
                .content_fit(self.content_fit)
                .class(self.class.clone())
                .into()
        };

        if let Some(path) = self.handle.data.path() {
            if path.extension().is_some_and(|ext| ext == OsStr::new("svg")) {
                from_svg(&iced::widget::svg::Handle::from_path(path))
            } else {
                from_image(&iced::widget::image::Handle::from_path(path))
            }
        } else {
            let bytes: &'static [u8] = &[];
            from_svg(&iced::widget::svg::Handle::from_memory(bytes))
        }
    }
}

impl<'a, Message: 'a> From<Icon> for iced::Element<'a, Message, OnagreTheme> {
    fn from(icon: Icon) -> Self {
        icon.view::<Message>()
    }
}
