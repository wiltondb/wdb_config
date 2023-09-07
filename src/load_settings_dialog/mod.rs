
mod args;
mod controls;
mod error;
mod events;
mod layout;
mod nui;
mod result;

use std::cell::RefCell;
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
use ui::PopupDialog;
use connect_dialog::ConnectConfig;
pub use args::LoadSettingsDialogArgs;
pub(self) use controls::LoadSettingsDialogControls;
use events::LoadSettingsDialogEvents;
use error::LoadSettingsDialogError;
use layout::LoadSettingsDialogLayout;
pub use result::SettingRecord;
pub use result::LoadSettingsDialogResult;
use crate::nwg_ui::PopupArgs;


#[derive(Default)]
pub struct LoadSettingsDialog {
    pub(self) controls: LoadSettingsDialogControls,
    pub(self) layout: LoadSettingsDialogLayout,
    pub(self) events: LoadSettingsDialogEvents,

    loaded_settings: RefCell<LoadSettingsDialogResult>,
    args: LoadSettingsDialogArgs,
    load_joiner: ui::PopupJoiner<LoadSettingsResult>,
}

impl LoadSettingsDialog {
    pub fn spawn_load(&self) -> JoinHandle<LoadSettingsResult> {
        let sender = self.controls.load_notice.sender();
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
        self.controls.load_notice.receive();
        let res = self.load_joiner.await_result();
        self.stop_progress_bar(res.success);
        if res.success {
            self.loaded_settings.replace(LoadSettingsDialogResult::new(res.records));
            self.close();
            return;
        }
        self.controls.label.set_text("Load settings failed");
        self.controls.details_box.set_text(&res.message);
    }

    pub fn copy_to_clipboard(&self) {
        let text = self.controls.details_box.text();
        let _ = set_clipboard(formats::Unicode, &text);
    }

    pub fn set_load_join_handle(&self, join_handle: JoinHandle<LoadSettingsResult>) {
        self.load_joiner.set_join_handle(join_handle);
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

impl ui::PopupDialog<LoadSettingsDialogArgs, LoadSettingsDialogResult> for LoadSettingsDialog {
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
        self.args.notify_parent();
        self.controls.hide_window();
        nwg::stop_thread_dispatch();
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
