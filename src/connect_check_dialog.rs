
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use std::time::Instant;

use clipboard_win::formats;
use clipboard_win::set_clipboard;
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

fn check_postgres_conn() -> Result<String, postgres::Error> {
    let mut client = Client::connect("host=127.0.0.1 user=wilton password=xwilton connect_timeout=100", NoTls)?;
    let vec = client.query("select version()", &[])?;
    let row = &vec[0];
    let res: String = row.get("version");
    client.close()?;
    Ok(res)
}

impl ConnectCheckDialog {
    pub fn spawn_connection_check(&self) {
        let sender = self.ui.check_notice().sender();
        thread::spawn(move || {
            let start = Instant::now();
            let res = match check_postgres_conn() {
                Ok(version) => version,
                Err(e) => format!("{}", e)
            };
            let remaining = 1000 - start.elapsed().as_millis() as i64;
            if remaining > 0 {
                thread::sleep(Duration::from_millis(remaining as u64));
            }
            sender.send_result(res);
        });
    }

    pub fn on_connection_check_complete(&self) {
        let res = self.ui.check_notice().receive();

        self.ui.progress_bar.set_marquee(false, 0);
        self.ui.progress_bar.remove_flags(nwg::ProgressBarFlags::MARQUEE);
        self.ui.progress_bar.set_pos(1);

        self.ui.set_label_text("Connection successful");
        self.ui.set_details_text(&res);
    }

    pub fn copy_to_clipboard(&self) {
        let text = self.ui.details_text();
        let _ = set_clipboard(formats::Unicode, &text);
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
