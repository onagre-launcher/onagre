use crate::app::plugin_matchers::Plugin;
use iced::futures::StreamExt;
use iced_native::futures::stream::BoxStream;
use iced_native::Subscription;

use crate::icons::IconPath;
use crate::THEME;
use std::hash::Hash;

pub struct PluginMatcherSubscription;

impl PluginMatcherSubscription {
    pub fn create() -> Subscription<Plugin> {
        Subscription::from_recipe(PluginMatcherSubscription)
    }
}

impl<H, I> iced_native::subscription::Recipe<H, I> for PluginMatcherSubscription
where
    H: std::hash::Hasher,
{
    type Output = Plugin;

    fn hash(&self, state: &mut H) {
        std::any::TypeId::of::<Self>().hash(state);
        "PluginMatcherSubscription".hash(state)
    }

    fn stream(self: Box<Self>, _: BoxStream<I>) -> BoxStream<Self::Output> {
        Box::pin(
            pop_launcher_toolkit::service::load::from_paths().map(|(path, config, regex)| {
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
            }),
        )
    }
}
