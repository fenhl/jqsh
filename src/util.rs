use {
    std::{
        fmt,
        ops::{
            Deref,
            DerefMut
        },
        sync::Arc
    },
    crate::lang::{
        channel::{
            Sender,
            Receiver
        },
        filter::Filter
    }
};

#[derive(Clone)]
pub struct Labeled<T> {
    label: String,
    value: T
}

impl<T> Labeled<T> {
    pub fn new<S: Into<String>>(label: S, value: T) -> Labeled<T> {
        Labeled {
            label: label.into(),
            value: value
        }
    }
}

impl<T> From<T> for Labeled<T> {
    fn from(value: T) -> Labeled<T> {
        Labeled::new("", value)
    }
}

impl<T> Deref for Labeled<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.value
    }
}

impl<T> DerefMut for Labeled<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

impl<T> fmt::Debug for Labeled<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.label, f)
    }
}

pub type FilterFn = Labeled<Arc<dyn Fn(&[Filter], Receiver, Sender) + Send + Sync>>;
