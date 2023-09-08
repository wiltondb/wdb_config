
use super::*;

#[derive(Default)]
pub struct ConnectCheckDialog {
    pub(super) c: ConnectCheckDialogControls,
    pub(super) layout: ConnectCheckDialogLayout,
    pub(super) events: ConnectCheckDialogEvents,

    args: ConnectCheckDialogArgs,
    check_joiner: ui::PopupJoiner<ConnectCheckResult>,
}

impl ConnectCheckDialog {
    pub fn spawn_connection_check(&self) -> JoinHandle<ConnectCheckResult> {
        let sender = self.c.check_notice.sender();
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
        self.c.check_notice.receive();
        let res = self.check_joiner.await_result();
        self.stop_progress_bar(res.success);
        let label = if res.success {
            "Connection successful"
        } else {
            "Connection failed"
        };
        self.c.label.set_text(label);
        self.c.details_box.set_text(&res.message);
    }

    pub fn copy_to_clipboard(&self) {
        let text = self.c.details_box.text();
        let _ = set_clipboard(formats::Unicode, &text);
    }

    pub fn set_check_join_handle(&self, join_handle: JoinHandle<ConnectCheckResult>) {
        self.check_joiner.set_join_handle(join_handle);
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
        self.c.hide_window();
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
