
use uuid::Uuid;

use super::*;

#[derive(Default)]
pub struct SettingChangeDialog {
    pub(super) c: SettingChangeDialogControls,

    args: SettingChangeDialogArgs,
    result: SettingChangeDialogResult,
    change_join_handle: ui::PopupJoinHandle<ChangeResult>,
}

impl SettingChangeDialog {
    pub(super) fn on_apply_change_complete(&mut self, _: nwg::EventData) {
        self.c.change_notice.receive();
        let res = self.change_join_handle.join();
        self.stop_progress_bar(res.success);
        if res.success {
            self.result = SettingChangeDialogResult::success(res.effective_value.clone());
            self.c.label.set_text("Setting change successful");
            let msg = format!("DB configuration reload requested,\
\r\napplied value: [{}],\
\r\neffective value: [{}],\
\r\nDB restart required: [{}].", &self.args.value, res.effective_value, res.restart_pending);
            self.c.details_box.set_text(&msg);
        } else {
            self.result = SettingChangeDialogResult::failure();
            self.c.label.set_text("Setting change failed");
            self.c.details_box.set_text(&res.error);
        }
    }

    pub(super) fn copy_to_clipboard(&mut self, _: nwg::EventData) {
        let text = self.c.details_box.text();
        let _ = set_clipboard(formats::Unicode, &text);
    }

    pub(super)

    fn stop_progress_bar(&self, success: bool) {
        self.c.progress_bar.set_marquee(false, 0);
        self.c.progress_bar.remove_flags(nwg::ProgressBarFlags::MARQUEE);
        self.c.progress_bar.set_pos(1);
        if !success {
            self.c.progress_bar.set_state(nwg::ProgressBarState::Error)
        }
    }

    fn apply_change(pg_conn_config: &PgConnConfig, name: &str, value: &str) -> Result<ChangeResult, PgConnError> {
        let mut client = pg_conn_config.open_connection()?;
        // syntax error at or near "$1"
        let uid = format!("_{}", Uuid::new_v4()).replace("-", "_");
        let sql = format!("alter system set {} = ${}${}${}$", name, uid, value, uid);
        client.execute(&sql, &[])?;
        client.execute("select pg_reload_conf()", &[])?;
        let vec_eff = client.query(&format!("show {}", name), &[])?;
        let sql_pending = format!("select pending_restart from pg_settings where name = $1");
        let vec_pending = client.query(&sql_pending, &[&name])?;
        client.close()?;
        if 0 == vec_eff.len() {
            return Err(PgConnError::from_string("Effective value fetch error".to_string()))
        }
        if 0 == vec_pending.len() {
            return Err(PgConnError::from_string("Restart pending flag fetch error".to_string()))
        }
        let pending: bool = vec_pending[0].get(0);
        let eff: String = vec_eff[0].get(0);
        Ok(ChangeResult::success(eff, pending))
    }
}

impl ui::PopupDialog<SettingChangeDialogArgs, SettingChangeDialogResult> for SettingChangeDialog {
    fn popup(args: SettingChangeDialogArgs) -> ui::PopupJoinHandle<SettingChangeDialogResult> {
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
        let sender = self.c.change_notice.sender();
        let pgconf = self.args.pg_conn_config.clone();
        let name = self.args.name.clone();
        let value = self.args.value.clone();
        let join_handle = thread::spawn(move || {
            let start = Instant::now();
            let res = match SettingChangeDialog::apply_change(&pgconf, &name, &value) {
                Ok(res) => res,
                Err(e) => ChangeResult::failure(format!("{}", e))
            };
            let remaining = 1000 - start.elapsed().as_millis() as i64;
            if remaining > 0 {
                thread::sleep(Duration::from_millis(remaining as u64));
            }
            sender.send();
            res
        });
        self.change_join_handle = ui::PopupJoinHandle::from(join_handle);
    }

    fn result(&mut self) -> SettingChangeDialogResult {
        self.result.clone()
    }

    fn close(&mut self, _: nwg::EventData) {
        self.args.send_notice();
        self.c.window.set_visible(false);
        nwg::stop_thread_dispatch();
    }
}
