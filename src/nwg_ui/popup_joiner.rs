
use std::cell::RefCell;
use std::thread::JoinHandle;

#[derive(Default)]
pub struct PopupJoiner<T: Send+Sync> {
    cell: RefCell<Option<JoinHandle<T>>>
}

impl<T: Send+Sync> PopupJoiner<T> {
    pub fn set_join_handle(&self, join_handle: JoinHandle<T>) {
        *self.cell.borrow_mut() = Some(join_handle);
    }

    pub fn await_result(&self) -> T {
        match self.cell.borrow_mut().take() {
            Some(handle) => handle.join().expect("Joiner error"),
            None => panic!("Join handle not set")
        }
    }
}
