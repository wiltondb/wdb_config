
use std::mem;
use std::thread::JoinHandle;

#[derive(Default)]
pub struct PopupJoinHandle<T> {
    handle: Option<JoinHandle<T>>
}

impl<T> PopupJoinHandle<T> {
    pub fn join(&mut self) -> T {
        mem::take(&mut self.handle)
            .expect("Join handle not set")
            .join()
            .expect("Join error")
    }
}

impl<T> From<JoinHandle<T>> for PopupJoinHandle<T> {
    fn from(value: JoinHandle<T>) -> Self {
        Self {
            handle: Some(value)
        }
    }
}
