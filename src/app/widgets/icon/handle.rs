// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

use super::Named;
use iced::widget::image;
use iced::widget::svg;
use std::borrow::Cow;

#[must_use]
#[derive(Clone, Debug, derive_setters::Setters)]
pub struct Handle {
    pub symbolic: bool,
    #[setters(skip)]
    pub data: Data,
}

#[must_use]
#[derive(Clone, Debug)]
pub enum Data {
    Name(Named),
    Image(image::Handle),
    Svg(svg::Handle),
}

/// Create a SVG handle from memory.
pub fn from_svg_bytes(bytes: impl Into<Cow<'static, [u8]>>) -> Handle {
    Handle {
        symbolic: false,
        data: Data::Svg(svg::Handle::from_memory(bytes)),
    }
}
