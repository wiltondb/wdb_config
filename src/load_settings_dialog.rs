
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
use load_settings_dialog_ui::LoadSettingsDialogUi;

#[derive(Default)]
pub struct LoadSettingsDialogArgs {
    notice_sender:  RefCell<SyncNoticeSender>,
    config: ConnectConfig,
}

impl LoadSettingsDialogArgs {
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

impl PopupDialogArgs for LoadSettingsDialogArgs {
    fn notify_parent(&self) {
        self.notice_sender.borrow().send()
    }
}

#[derive(Default, Debug, Clone)]
pub struct SettingRecord {
    pub name: String,
    pub setting: String,
    pub description: String,
}

#[derive(Default)]
pub struct LoadSettingsDialogResult {
    pub records: Vec<SettingRecord>
}

impl LoadSettingsDialogResult {
    pub fn new(records: Vec<SettingRecord>) -> Self {
        Self { records }
    }
}

#[derive(Debug)]
struct LoadSettingsDialogError {
    message: String
}

impl LoadSettingsDialogError {
    fn new<E: fmt::Display>(e: &E) -> Self {
        Self {
            message: format!("{}", e)
        }
    }
}

impl fmt::Display for LoadSettingsDialogError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<postgres::Error> for LoadSettingsDialogError {
    fn from(value: postgres::Error) -> Self {
        Self::new(&value)
    }
}

impl From<native_tls::Error> for LoadSettingsDialogError {
    fn from(value: native_tls::Error) -> Self {
        Self::new(&value)
    }
}

#[derive(Default)]
pub struct LoadSettingsDialog {
    args: LoadSettingsDialogArgs,
    ui: LoadSettingsDialogUi,
    loaded_settings: RefCell<LoadSettingsDialogResult>,
    load_joiner: DialogJoiner<LoadSettingsResult>,
}

impl LoadSettingsDialog {
    pub fn spawn_load(&self) -> JoinHandle<LoadSettingsResult> {
        let sender = self.ui.load_notice().sender();
        let config = self.args.config.clone();
        thread::spawn(move || {
            let start = Instant::now();
            let res = match load_settings_from_db(&config) {
                Ok(records) => LoadSettingsResult::success(records),
                Err(e) => LoadSettingsResult::failure(format!("{}", e))
            };
            let remaining = 1000 - start.elapsed().as_millis() as i64;
            if remaining > 0 {
                thread::sleep(Duration::from_millis(remaining as u64));
            }
            sender.send();
            res
        })
    }

    pub fn on_load_complete(&self) {
        self.ui.load_notice().receive();
        let res = self.load_joiner.await_result();
        self.ui.stop_progress_bar(res.success);
        if res.success {
            self.loaded_settings.replace(LoadSettingsDialogResult::new(res.records));
            self.close();
            return;
        }
        self.ui.set_label_text("Load settings failed");
        self.ui.set_details_text(&res.message);
    }

    pub fn copy_to_clipboard(&self) {
        let text = self.ui.details_text();
        let _ = set_clipboard(formats::Unicode, &text);
    }

    pub fn set_load_join_handle(&self, join_handle: JoinHandle<LoadSettingsResult>) {
        self.load_joiner.set_join_handle(join_handle);
    }
}

impl PopupDialog<LoadSettingsDialogUi, LoadSettingsDialogArgs, LoadSettingsDialogResult> for LoadSettingsDialog {
    fn popup(args: LoadSettingsDialogArgs) -> JoinHandle<LoadSettingsDialogResult> {
        thread::spawn(move || {
            let data = Self {
                args,
                ..Default::default()
            };
            let dialog = Self::build_ui(data).expect("Failed to build UI");
            let join_handle = dialog.inner.spawn_load();
            dialog.inner.set_load_join_handle(join_handle);
            nwg::dispatch_thread_events();
            dialog.result()
        })
    }

    fn close(&self) {
        self.args.send_notice();
        self.ui.hide_window();
        nwg::stop_thread_dispatch();
    }

    fn ui(&self) -> &LoadSettingsDialogUi {
        &self.ui
    }

    fn ui_mut(&mut self) -> &mut LoadSettingsDialogUi {
        &mut self.ui
    }

    fn result(&self) -> LoadSettingsDialogResult {
        self.loaded_settings.take()
    }
}

#[derive(Default)]
pub struct LoadSettingsResult {
    success: bool,
    message: String,
    records: Vec<SettingRecord>,
}

impl LoadSettingsResult {
    fn success(records: Vec<SettingRecord>) -> Self {
        Self {
            success: true,
            message: String::new(),
            records,
        }
    }

    fn failure(message: String) -> Self {
        Self {
            success: false,
            message,
            records: Vec::new(),
        }
    }
}

fn load_settings_from_db(config: &ConnectConfig) -> Result<Vec<SettingRecord>, LoadSettingsDialogError> {
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

    let vec = client.query("show all", &[])?;
    client.close()?;
    let res = vec.iter().map(|row| {
       SettingRecord {
           name: row.get("name"),
           setting: row.get("setting"),
           description: row.get("description"),
       }
    }).collect();
    Ok(res)
}
