
use std::cell::RefCell;
use std::thread;
use std::thread::JoinHandle;

use nwg::NativeUi;
use postgres::{Client, NoTls};

use crate::*;
use notice::SyncNoticeSender;

#[derive(Default)]
pub struct AboutDialog {
    notice_sender: Option<SyncNoticeSender>,

    pub response: RefCell<String>,

    pub window: nwg::Window,
    pub choice_yes: nwg::Button,
    pub connect_button: nwg::Button,

    pub events: events::Events<Self>,
}

impl AboutDialog {

    pub fn close(&self) {
        self.notice_sender.as_ref().expect("Notice sender not initialized").send();
        nwg::stop_thread_dispatch();
    }

    pub fn connect(&self) {
        //thread::spawn(move || {
            let mut client = Client::connect("host=127.0.0.1 user=wilton password=wilton", NoTls).expect("Connection failure");

            for row in client.query("show listen_addresses", &[]).expect("Query failure") {
                let val: String = row.get("listen_addresses");
                let mut data = self.response.borrow_mut();
                *data = val;
            }

            client.close().expect("Connection close error");
        //});
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

impl dialogs::PopupDialog<String> for AboutDialog {
    fn popup(notice_sender: SyncNoticeSender) -> JoinHandle<String> {
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

    fn result(&self) -> String {
        self.response.take()
    }
}