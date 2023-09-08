
use super::*;

#[derive(Default)]
pub struct LoadSettingsDialog {
    pub(super) c: LoadSettingsDialogControls,
    pub(super) layout: LoadSettingsDialogLayout,
    pub(super) events: LoadSettingsDialogEvents,

    loaded_settings: RefCell<LoadSettingsDialogResult>,
    args: LoadSettingsDialogArgs,
    load_joiner: ui::PopupJoiner<LoadSettingsResult>,
}

impl LoadSettingsDialog {
    pub fn on_load_complete(&self) {
        self.c.load_notice.receive();
        let res = self.load_joiner.await_result();
        self.stop_progress_bar(res.success);
        if res.success {
            self.loaded_settings.replace(LoadSettingsDialogResult::new(res.records));
            self.close();
            return;
        }
        self.c.label.set_text("Load settings failed");
        self.c.details_box.set_text(&res.message);
    }

    pub fn copy_to_clipboard(&self) {
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
}

impl ui::PopupDialog<LoadSettingsDialogArgs, LoadSettingsDialogResult> for LoadSettingsDialog {
    fn popup(args: LoadSettingsDialogArgs) -> JoinHandle<LoadSettingsDialogResult> {
        thread::spawn(move || {
            let data = Self {
                args,
                ..Default::default()
            };
            let dialog = Self::build_ui(data).expect("Failed to build UI");
            nwg::dispatch_thread_events();
            dialog.result()
        })
    }

    fn init(&self) {
        let sender = self.c.load_notice.sender();
        let config = self.args.config.clone();
        let join_handle = thread::spawn(move || {
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
        });
        self.load_joiner.set_join_handle(join_handle);
    }

    fn result(&self) -> LoadSettingsDialogResult {
        self.loaded_settings.take()
    }

    fn close(&self) {
        self.args.notify_parent();
        self.c.hide_window();
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
