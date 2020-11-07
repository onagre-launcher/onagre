use iced_native::Subscription;
pub mod desktop_entries;

pub trait ToSubScription<T> {
    fn subscription() -> Subscription<T>;
}
