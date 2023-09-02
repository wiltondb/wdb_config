
use std::cell::RefCell;
use std::thread::JoinHandle;

use crate::*;
use notice::SyncNoticeSender;

pub trait PopupDialog<T: Send+Sync> {
    fn popup(notice_sender: SyncNoticeSender) -> JoinHandle<T>;

    fn result(&self) -> T;

    fn close(&self);
}

pub trait DialogUi {
    fn window(&self) -> &nwg::Window;

    fn build_controls(&mut self) -> Result<(), nwg::NwgError>;

    fn build_layout(&mut self) -> Result<(), nwg::NwgError>;

    fn shake_after_layout(&self) {
        // workaround for garbled text
        let (wx, wy) = self.window().size();
        self.window().set_size(wx + 1, wy + 1);
        self.window().set_size(wx, wy);
    }
}

#[derive(Default)]
pub struct DialogJoiner<T: Send+Sync> {
    cell: RefCell<Option<JoinHandle<T>>>
}

impl<T: Send+Sync> DialogJoiner<T> {
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




/*
#[derive(Default)]
pub struct ProgressBarDialog {
    notice_sender: Option<SyncNoticeSender>,

    progress_bar: nwg::ProgressBar,
}

impl ProgressBarDialog {

}

impl PopupDialog<()> for ProgressBarDialog {
    fn popup(notice_sender: SyncNoticeSender) -> JoinHandle<()> {
        thread::spawn(move || {
            let data = Self {
                notice_sender: Some(notice_sender),
                ..Default::default()
            };
            let dialog = Self::build_ui(data).expect("Failed to build UI");
            nwg::dispatch_thread_events();
            dialog.result()
        })
    }

    fn result(&self) -> () {
        ()
    }
}

 */