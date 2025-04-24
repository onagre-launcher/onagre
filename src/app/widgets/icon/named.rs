// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

use onagre_launcher_toolkit::launcher::IconSource;

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

    /// Name of theme in which locate icon
    pub(super) theme: Option<Arc<str>>,

    /// Checks for a fallback if the icon was not found.
    #[setters(strip_option)]
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
    pub fn from_icon_source(value: IconSource, theme: Option<impl Into<Arc<str>>>) -> Self {
        match value {
            IconSource::Name(name) => Named::new(name, theme),
            IconSource::Mime(name) => {
                let name = name.replace('/', "-");
                Named::new(name, theme)
            }
        }
    }

    pub fn new(name: impl Into<Arc<str>>, theme: Option<impl Into<Arc<str>>>) -> Self {
        let name = name.into();
        let theme = theme.map(Into::into);

        Self {
            symbolic: name.ends_with("-symbolic"),
            name,
            fallback: Some(IconFallback::Default),
            size: None,
            scale: None,
            prefer_svg: true,
            theme,
        }
    }

    #[must_use]
    pub fn path(&self) -> Option<PathBuf> {
        let name = &*self.name;
        let theme = self.theme.as_deref().unwrap();
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

        let mut result = locate(theme, name);

        // On failure, attempt to locate fallback icon.
        if result.is_none() {
            let default_theme_gtk = freedesktop_icons::default_theme_gtk();
            if matches!(fallback, Some(IconFallback::Default)) {
                for new_name in name.rmatch_indices('-').map(|(pos, _)| &name[..pos]) {
                    result = default_theme_gtk.as_ref().and_then(|t| locate(t, new_name));
                    if result.is_some() {
                        break;
                    }
                }
            } else if let Some(IconFallback::Names(fallbacks)) = fallback {
                for fallback in fallbacks {
                    result = default_theme_gtk
                        .as_ref()
                        .and_then(|t| locate(t, fallback.as_ref()));
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
            data: self,
        }
    }

    pub fn icon(self, selected: bool) -> Icon {
        let size = self.size;

        let icon = super::icon(self.handle(), selected);

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
