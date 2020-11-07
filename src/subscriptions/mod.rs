use iced_native::Subscription;
pub mod desktop_entries;
pub mod custom;

pub trait ToSubScription<T> {
    fn subscription() -> Subscription<T>;
}
