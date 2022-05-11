use crate::ui::plugin_matchers::Plugin;
use iced::futures::StreamExt;
use iced_native::futures::stream::BoxStream;
use iced_native::Subscription;



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
            pop_launcher_toolkit::service::load::from_paths().map(|(_path, config, regex)| Plugin {
                name: config.name.to_string(),
                history: config.history,
                help: config.query.help.map(|h| h.to_string()),
                regex,
            }),
        )
    }
}
