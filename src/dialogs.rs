
//use std::thread;
use std::thread::JoinHandle;

use crate::*;
use notice::SyncNoticeSender;

pub trait PopupDialog<T: Send+Sync> {
    fn popup(notice_sender: SyncNoticeSender) -> JoinHandle<T>;

    fn result(&self) -> T;
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