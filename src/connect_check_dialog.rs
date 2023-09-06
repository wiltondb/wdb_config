
use std::cell::RefCell;
use std::fmt;
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
use dialogs::DialogJoiner;
use dialogs::DialogUi;
use dialogs::PopupDialog;
use dialogs::PopupDialogArgs;
use notice::SyncNotice;
use notice::SyncNoticeSender;
use connect_dialog::ConnectConfig;
use connect_check_dialog_ui::ConnectCheckDialogUi;

#[derive(Default)]
pub struct ConnectCheckDialogArgs {
    notice_sender:  RefCell<SyncNoticeSender>,
    config: ConnectConfig,
}

impl ConnectCheckDialogArgs {
    pub fn new(notice: &SyncNotice, config: ConnectConfig) -> Self {
        Self {
            notice_sender: RefCell::new(notice.sender()),
            config,
        }
    }

    pub fn send_notice(&self) {
        self.notice_sender.borrow().send()
    }
}

impl PopupDialogArgs for ConnectCheckDialogArgs {
    fn notify_parent(&self) {
        self.notice_sender.borrow().send()
    }
}

pub struct ConnectCheckDialogResult {
   pub value: Result<String, postgres::Error>
}

impl Default for ConnectCheckDialogResult {
    fn default() -> Self {
        Self {
            value: Ok(String::new())
        }
    }
}

#[derive(Debug)]
struct ConnectCheckDialogError {
    message: String
}

impl ConnectCheckDialogError {
    fn new<E: fmt::Display>(e: &E) -> Self {
        Self {
            message: format!("{}", e)
        }
    }
}

impl fmt::Display for ConnectCheckDialogError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<postgres::Error> for ConnectCheckDialogError {
    fn from(value: postgres::Error) -> Self {
        Self::new(&value)
    }
}

impl From<native_tls::Error> for ConnectCheckDialogError {
    fn from(value: native_tls::Error) -> Self {
        Self::new(&value)
    }
}

#[derive(Default)]
pub struct ConnectCheckDialog {
    args: ConnectCheckDialogArgs,
    ui: ConnectCheckDialogUi,
    check_joiner: DialogJoiner<ConnectCheckResult>,
}

impl ConnectCheckDialog {
    pub fn spawn_connection_check(&self) -> JoinHandle<ConnectCheckResult> {
        let sender = self.ui.check_notice().sender();
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
        self.ui.check_notice().receive();
        let res = self.check_joiner.await_result();
        self.ui.stop_progress_bar(res.success);
        let label = if res.success {
            "Connection successful"
        } else {
            "Connection failed"
        };
        self.ui.set_label_text(label);
        self.ui.set_details_text(&res.message);
    }

    pub fn copy_to_clipboard(&self) {
        let text = self.ui.details_text();
        let _ = set_clipboard(formats::Unicode, &text);
    }

    pub fn set_check_join_handle(&self, join_handle: JoinHandle<ConnectCheckResult>) {
        self.check_joiner.set_join_handle(join_handle);
    }
}

impl PopupDialog<ConnectCheckDialogUi, ConnectCheckDialogArgs, ConnectCheckDialogResult> for ConnectCheckDialog {
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

    fn close(&self) {
        self.args.send_notice();
        self.ui.hide_window();
        nwg::stop_thread_dispatch();
    }

    fn ui(&self) -> &ConnectCheckDialogUi {
        &self.ui
    }

    fn ui_mut(&mut self) -> &mut ConnectCheckDialogUi {
        &mut self.ui
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

