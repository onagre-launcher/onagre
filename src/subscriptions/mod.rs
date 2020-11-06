use iced_native::Subscription;
pub mod desktop_entries;

use async_std::fs;
use async_std::path::PathBuf as AsyncPathBuf;
use futures::future::{BoxFuture, FutureExt};
use iced_native::futures::stream::BoxStream;
use iced_native::futures::StreamExt;
use std::hash::Hash;

pub trait ToSubScription<T> {
    fn subscription() -> Subscription<T>;
}
