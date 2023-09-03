
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use std::time::Instant;

use nwg::NativeUi;
use postgres::Client;
use postgres::NoTls;

use crate::*;
use dialogs::DialogUi;
use dialogs::PopupDialog;
use notice::SyncNoticeSender;
use connect_check_dialog_ui::ConnectCheckDialogUi;

#[derive(Default)]
pub struct ConnectCheckDialog {
    notice_sender: Option<SyncNoticeSender>,
    pub ui: ConnectCheckDialogUi,
}

impl ConnectCheckDialog {
    pub fn spawn_connection_check(&self) {
        let sender = self.ui.check_notice.sender();
        thread::spawn(move || {
            let start = Instant::now();
            let mut client = Client::connect("host=127.0.0.1 user=wilton password=wilton", NoTls).expect("Connection failure");

            let vec = client.query("show shared_preload_libraries", &[]).expect("Query failure");
            let row = &vec[0];
            let val: String = row.get("shared_preload_libraries");
            let res = "babelfishpg_tds" == val;

            client.close().expect("Connection close error");
            let remaining = 1000 - start.elapsed().as_millis() as i64;
            if remaining > 0 {
                thread::sleep(Duration::from_millis(remaining as u64));
            }
            sender.send_result(res);
        });
    }

    pub fn on_connection_check_complete(&self) {
        let res = self.ui.check_notice.receive();
        self.ui.label.set_text(&res.to_string());
        self.ui.progress_bar.set_marquee(false, 0);
        self.ui.progress_bar.remove_flags(nwg::ProgressBarFlags::MARQUEE);
        self.ui.progress_bar.set_pos(1);
    }
}

impl PopupDialog<bool> for ConnectCheckDialog {
    fn popup(notice_sender: SyncNoticeSender) -> JoinHandle<bool> {
        thread::spawn(move || {
            let data = Self {
                notice_sender: Some(notice_sender),
                ..Default::default()
            };
            let dialog = Self::build_ui(data).expect("Failed to build UI");
            dialog.inner.spawn_connection_check();
            nwg::dispatch_thread_events();
            dialog.result()
        })
    }

    fn close(&self) {
        self.notice_sender.as_ref().expect("Notice sender not initialized").send();
        self.ui.window().set_visible(false);
        nwg::stop_thread_dispatch();
    }
}
