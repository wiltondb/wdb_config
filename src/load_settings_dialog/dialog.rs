
use super::*;

#[derive(Default)]
pub struct LoadSettingsDialog {
    pub(super) c: LoadSettingsDialogControls,

    loaded_settings: LoadSettingsDialogResult,
    args: LoadSettingsDialogArgs,
    load_join_handle: ui::PopupJoinHandle<LoadSettingsResult>,
}

impl LoadSettingsDialog {
    pub fn on_load_complete(&mut self, _: nwg::EventData) {
        self.c.load_notice.receive();
        let res = self.load_join_handle.join();
        self.stop_progress_bar(res.success);
        if res.success {
            self.loaded_settings = LoadSettingsDialogResult::new(res.records);
            self.close(nwg::EventData::NoData);
            return;
        }
        self.c.label.set_text("Load settings failed");
        self.c.details_box.set_text(&res.message);
        self.c.copy_clipboard_button.set_enabled(true);
        self.c.close_button.set_enabled(true);
    }

    pub fn copy_to_clipboard(&mut self, _: nwg::EventData) {
        let text = self.c.details_box.text();
        let _ = set_clipboard(formats::Unicode, &text);
    }

    pub fn stop_progress_bar(&self, success: bool) {
        self.c.progress_bar.set_marquee(false, 0);
        self.c.progress_bar.remove_flags(nwg::ProgressBarFlags::MARQUEE);
        self.c.progress_bar.set_pos(1);
        if !success {
            self.c.progress_bar.set_state(nwg::ProgressBarState::Error)
        }
    }

    fn load_settings_from_db(pg_conn_config: &PgConnConfig) -> Result<Vec<SettingRecord>, PgAccessError> {
        let mut client = pg_conn_config.open_connection()?;
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
}

impl ui::PopupDialog<LoadSettingsDialogArgs, LoadSettingsDialogResult> for LoadSettingsDialog {
    fn popup(args: LoadSettingsDialogArgs) -> ui::PopupJoinHandle<LoadSettingsDialogResult> {
        let join_handle = thread::spawn(move || {
            let data = Self {
                args,
                ..Default::default()
            };
            let mut dialog = Self::build_ui(data).expect("Failed to build UI");
            nwg::dispatch_thread_events();
            dialog.result()
        });
        ui::PopupJoinHandle::from(join_handle)
    }

    fn init(&mut self) {
        let sender = self.c.load_notice.sender();
        let pgconf = self.args.pg_conn_config.clone();
        let join_handle = thread::spawn(move || {
            let start = Instant::now();
            let res = match LoadSettingsDialog::load_settings_from_db(&pgconf) {
                Ok(records) => LoadSettingsResult::success(records),
                Err(e) => LoadSettingsResult::failure(format!("{}", e))
            };
            let remaining = 1000 - start.elapsed().as_millis() as i64;
            if remaining > 0 {
                thread::sleep(Duration::from_millis(remaining as u64));
            }
            sender.send();
            res
        });
        self.load_join_handle = ui::PopupJoinHandle::from(join_handle);
    }

    fn result(&mut self) -> LoadSettingsDialogResult {
        self.loaded_settings.clone()
    }

    fn close(&mut self, _: nwg::EventData) {
        self.args.notify_parent();
        self.c.window.set_visible(false);
        nwg::stop_thread_dispatch();
    }
}

#[derive(Default)]
struct LoadSettingsResult {
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
