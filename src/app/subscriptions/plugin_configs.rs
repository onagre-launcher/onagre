use std::hash::Hasher;

use iced::futures::stream::BoxStream;
use iced::futures::StreamExt;
use iced::Subscription;
use iced_core::event::Status;
use iced_runtime::futures::subscription::Recipe;

use crate::app::plugin_matchers::Plugin;
use crate::icons::IconPath;
use crate::THEME;

pub struct PluginMatcherSubscription;

impl PluginMatcherSubscription {
    pub fn create() -> Subscription<Plugin> {
        Subscription::from_recipe(PluginMatcherSubscription)
    }
}

impl Recipe for PluginMatcherSubscription {
    type Output = Plugin;

    fn hash(&self, state: &mut iced_core::Hasher) {
        state.write("PluginMatcherSubscription".as_bytes())
    }

    fn stream(self: Box<Self>, _: BoxStream<(iced::Event, Status)>) -> BoxStream<Self::Output> {
        Box::pin(onagre_launcher_toolkit::service::load::from_paths().map(
            |(path, config, regex)| {
                let icon: Option<IconPath> = THEME.icon_theme.as_ref().and_then(|theme| {
                    config
                        .icon
                        .as_ref()
                        .map(|source| (source, theme))
                        .and_then(|(source, theme)| IconPath::from_source(source, theme))
                });

                let name = path
                    .parent()
                    .expect("Plugin config should have a parent directory")
                    .file_name()
                    .expect("Plugin directory should have an utf8 filename")
                    .to_string_lossy()
                    .to_string();

                Plugin {
                    name,
                    icon,
                    history: config.history,
                    help: config.query.help.map(|h| h.to_string()),
                    regex,
                }
            },
        ))
    }
}
