
mod args;
mod controls;
mod events;
mod error;
mod layout;
mod nui;
mod result;

use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use std::time::Instant;

use clipboard_win::formats;
use clipboard_win::set_clipboard;
use native_tls::TlsConnector;
use nwg::NativeUi;
use postgres::config::Config;
use postgres::NoTls;
use postgres_native_tls::MakeTlsConnector;

use crate::*;
use nwg_ui as ui;
use ui::Controls;
use connect_dialog::ConnectConfig;
pub use args::ConnectCheckDialogArgs;
pub(self) use controls::ConnectCheckDialogControls;
use events::ConnectCheckDialogEvents;
use error::ConnectCheckDialogError;
use layout::ConnectCheckDialogLayout;
pub use result::ConnectCheckDialogResult;

#[derive(Default)]
pub struct ConnectCheckDialog {
    pub(self) controls: ConnectCheckDialogControls,
    pub(self) layout: ConnectCheckDialogLayout,
    pub(self) events: ConnectCheckDialogEvents,

    args: ConnectCheckDialogArgs,
    check_joiner: ui::PopupJoiner<ConnectCheckResult>,
}

impl ConnectCheckDialog {
    pub fn spawn_connection_check(&self) -> JoinHandle<ConnectCheckResult> {
        let sender = self.controls.check_notice.sender();
        let config = self.args.config.clone();
        thread::spawn(move || {
            let start = Instant::now();
            let res = match check_postgres_conn(&config) {
                Ok(version) => ConnectCheckResult::success(version),
                Err(e) => ConnectCheckResult::failure(format!("{}", e))
            };
            let remaining = 1000 - start.elapsed().as_millis() as i64;
            if remaining > 0 {
                thread::sleep(Duration::from_millis(remaining as u64));
            }
            sender.send();
            res
        })
    }

    pub fn on_connection_check_complete(&self) {
        self.controls.check_notice.receive();
        let res = self.check_joiner.await_result();
        self.stop_progress_bar(res.success);
        let label = if res.success {
            "Connection successful"
        } else {
            "Connection failed"
        };
        self.controls.label.set_text(label);
        self.controls.details_box.set_text(&res.message);
    }

    pub fn copy_to_clipboard(&self) {
        let text = self.controls.details_box.text();
        let _ = set_clipboard(formats::Unicode, &text);
    }

    pub fn set_check_join_handle(&self, join_handle: JoinHandle<ConnectCheckResult>) {
        self.check_joiner.set_join_handle(join_handle);
    }

    pub fn stop_progress_bar(&self, success: bool) {
        self.controls.progress_bar.set_marquee(false, 0);
        self.controls.progress_bar.remove_flags(nwg::ProgressBarFlags::MARQUEE);
        self.controls.progress_bar.set_pos(1);
        if !success {
            self.controls.progress_bar.set_state(nwg::ProgressBarState::Error)
        }
    }
}

impl ui::PopupDialog<ConnectCheckDialogArgs, ConnectCheckDialogResult> for ConnectCheckDialog {
    fn popup(args: ConnectCheckDialogArgs) -> JoinHandle<ConnectCheckDialogResult> {
        thread::spawn(move || {
            let data = Self {
                args,
                ..Default::default()
            };
            let dialog = Self::build_ui(data).expect("Failed to build UI");
            let join_handle = dialog.inner.spawn_connection_check();
            dialog.inner.set_check_join_handle(join_handle);
            nwg::dispatch_thread_events();
            dialog.result()
        })
    }

    fn result(&self) -> ConnectCheckDialogResult {
        // todo
        Default::default()
    }

    fn close(&self) {
        self.args.send_notice();
        self.controls.hide_window();
        nwg::stop_thread_dispatch();
    }
}

#[derive(Default)]
pub struct ConnectCheckResult {
    success: bool,
    message: String,
}

impl ConnectCheckResult {
    fn success(message: String) -> Self {
        Self {
            success: true,
            message
        }
    }

    fn failure(message: String) -> Self {
        Self {
            success: false,
            message
        }
    }
}

fn check_postgres_conn(config: &ConnectConfig) -> Result<String, ConnectCheckDialogError> {
    let pgconf = Config::new()
        .host(&config.hostname)
        .port(config.port)
        .user(&config.username)
        .password(&config.password)
        .connect_timeout(Duration::from_secs(10))
        .clone();

    let mut client = if config.enable_tls {
        let connector = TlsConnector::builder()
            .danger_accept_invalid_certs(config.accept_invalid_tls)
            .danger_accept_invalid_hostnames(config.accept_invalid_tls)
            .build()?;
        let tls = MakeTlsConnector::new(connector);
        pgconf.connect(tls)?
    } else {
        pgconf.connect(NoTls)?
    };

    let vec = client.query("select version()", &[])?;
    let row = &vec[0];
    let res: String = row.get("version");
    client.close()?;
    Ok(res)
}

