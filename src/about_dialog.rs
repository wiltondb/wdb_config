
use std::cell::RefCell;
use std::thread;

use nwg::NativeUi;

use super::*;
use crate::notice::SyncNoticeSender;

#[derive(Default)]
pub struct AboutDialog {
    notice_sender: Option<SyncNoticeSender>,

    pub response: RefCell<Option<String>>,

    //#[nwg_control(size: (300, 115), position: (650, 300), title: "A dialog", flags: "WINDOW|VISIBLE")]
    //#[nwg_events( OnWindowClose: [YesNoDialog::close] )]
    pub window: nwg::Window,

    //#[nwg_control(text: "YES", position: (10, 10), size: (130, 95))]
    //#[nwg_events( OnButtonClick: [YesNoDialog::choose(SELF, CTRL)] )]
    pub choice_yes: nwg::Button,

    //#[nwg_control(text: "NO", position: (160, 10), size: (130, 95), focus: true)]
    //#[nwg_events( OnButtonClick: [YesNoDialog::choose(SELF, CTRL)] )]
    pub choice_no: nwg::Button,

    pub events: Vec<events::EventHandler<Self>>,
}

impl AboutDialog {

    fn new(notice_sender: SyncNoticeSender) -> Self {
        Self {
            notice_sender: Some(notice_sender),
            ..Default::default()
        }

    }

    /// Create the dialog UI on a new thread. The dialog result will be returned by the thread handle.
    /// To alert the main GUI that the dialog completed, this function takes a notice sender object.
    pub fn popup(notice_sender: SyncNoticeSender) -> thread::JoinHandle<String> {
        thread::spawn(move || {
            // Create the UI just like in the main function
            let data = AboutDialog::new(notice_sender);
            let app = AboutDialog::build_ui(data).expect("Failed to build UI");
            nwg::dispatch_thread_events();

            // Return the dialog data
            app.response.take().unwrap_or("Cancelled!".to_owned())
        })
    }

    pub fn close(&self) {
        self.notice_sender.as_ref().expect("Notice sender not initialized").send();
        nwg::stop_thread_dispatch();
    }

    /*
    fn choose(&self, btn: &nwg::Button) {
        let mut data = self.response.borrow_mut();
        if btn == &self.choice_no {
            *data = Some("No!".to_string());
        } else if btn == &self.choice_yes {
            *data = Some("Yes!".to_string());
        }

        self.window.close();
    }
     */

}