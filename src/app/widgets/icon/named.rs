// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

use crate::THEME;

use super::{Handle, Icon};
use std::{borrow::Cow, path::PathBuf, sync::Arc};

#[derive(Debug, Clone, Default, Hash)]
/// Fallback icon to use if the icon was not found.
pub enum IconFallback {
    #[default]
    /// Default fallback using the icon name.
    Default,
    /// Fallback to specific icon names.
    Names(Vec<Cow<'static, str>>),
}

#[must_use]
#[derive(derive_setters::Setters, Clone, Debug, Hash)]
pub struct Named {
    /// Name of icon to locate in an XDG icon path.
    pub(super) name: Arc<str>,

    /// Checks for a fallback if the icon was not found.
    pub fallback: Option<IconFallback>,

    /// Restrict the lookup to a given scale.
    #[setters(strip_option)]
    pub scale: Option<u16>,

    /// Restrict the lookup to a given size.
    #[setters(strip_option)]
    pub size: Option<u16>,

    /// Whether the icon is symbolic or not.
    pub symbolic: bool,

    /// Prioritizes SVG over PNG
    pub prefer_svg: bool,
}

impl Named {
    pub fn new(name: impl Into<Arc<str>>) -> Self {
        let name = name.into();
        Self {
            symbolic: name.ends_with("-symbolic"),
            name,
            fallback: Some(IconFallback::Default),
            size: None,
            scale: None,
            prefer_svg: false,
        }
    }

    #[must_use]
    pub fn path(&self) -> Option<PathBuf> {
        let name = &*self.name;
        let fallback = &self.fallback;
        let locate = |theme: &str, name| {
            let mut lookup = freedesktop_icons::lookup(name)
                .with_theme(theme.as_ref())
                .with_cache();

            if let Some(scale) = self.scale {
                lookup = lookup.with_scale(scale);
            }

            if let Some(size) = self.size {
                lookup = lookup.with_size(size);
            }

            if self.prefer_svg {
                lookup = lookup.force_svg();
            }
            lookup.find()
        };

        let theme = &THEME.icon_theme.clone().unwrap_or("Adwaita".to_string());

        let mut result = locate(theme, name);

        // On failure, attempt to locate fallback icon.
        if result.is_none() {
            if matches!(fallback, Some(IconFallback::Default)) {
                for new_name in name.rmatch_indices('-').map(|(pos, _)| &name[..pos]) {
                    result =
                        freedesktop_icons::default_theme_gtk().and_then(|t| locate(&t, new_name));
                    if result.is_some() {
                        break;
                    }
                }
            } else if let Some(IconFallback::Names(fallbacks)) = fallback {
                for fallback in fallbacks {
                    result = freedesktop_icons::default_theme_gtk()
                        .and_then(|t| locate(&t, fallback.as_ref()));
                    if result.is_some() {
                        break;
                    }
                }
            }
        }

        result
    }

    pub fn handle(self) -> Handle {
        Handle {
            symbolic: self.symbolic,
            data: super::Data::Name(self),
        }
    }

    pub fn icon(self) -> Icon {
        let size = self.size;

        let icon = super::icon(self.handle());

        match size {
            Some(size) => icon.size(size),
            None => icon,
        }
    }
}

impl From<Named> for Handle {
    fn from(builder: Named) -> Self {
        builder.handle()
    }
}

impl From<Named> for Icon {
    fn from(builder: Named) -> Self {
        builder.icon()
    }
}

impl<Message: 'static> From<Named> for iced::Element<'_, Message, crate::Theme> {
    fn from(builder: Named) -> Self {
        builder.icon().into()
    }
}
