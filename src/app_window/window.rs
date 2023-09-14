
use std::env;
use std::os::windows::process::CommandExt;
use std::process::Command;
use std::process::Stdio;

use wildmatch::WildMatch;

use super::*;

#[derive(Default)]
pub struct AppWindow {
    pub(super) c: AppWindowControls,

    pg_conn_config: PgConnConfig,
    settings: Vec<SettingRecord>,

    networking_settings: HashSet<String>,
    logging_settings: HashSet<String>,
    memory_settings: HashSet<String>,
    escape_hatch_settings: HashSet<String>,

    about_dialog_join_handle: ui::PopupJoinHandle<()>,
    connect_dialog_join_handle: ui::PopupJoinHandle<ConnectDialogResult>,
    load_settings_dialog_join_handle: ui::PopupJoinHandle<LoadSettingsDialogResult>,
    setting_dialog_join_handle: ui::PopupJoinHandle<()>,
}

impl AppWindow {

    pub fn new() -> Self {
        Default::default()
    }

    pub(super) fn init(&mut self) {
        self.networking_settings = setting_groups::networking();
        self.logging_settings = setting_groups::logging();
        self.memory_settings = setting_groups::memory();
        self.escape_hatch_settings = setting_groups::escape_hatches();

        let cmd_args = Self::get_cmd_args();
        if 2 == cmd_args.len() && "--postinstall" == cmd_args[1] {
            self.pg_conn_config.hostname = String::from("localhost");
            self.pg_conn_config.port = 5432;
            self.pg_conn_config.username = String::from("wilton");
            self.pg_conn_config.password = String::from("wilton");
            self.pg_conn_config.enable_tls = true;
            self.pg_conn_config.accept_invalid_tls = true;
            self.set_status_bar_dbconn_label("localhost:5432");
            self.open_load_dialog(nwg::EventData::NoData);
        } else {
            self.pg_conn_config.hostname = String::from("localhost");
            self.pg_conn_config.port = 5432;
            self.set_status_bar_dbconn_label("none");
            self.open_connect_dialog(nwg::EventData::NoData);
        }
    }

    pub(super) fn close(&mut self, _: nwg::EventData) {
        self.c.window.set_visible(false);
        nwg::stop_thread_dispatch();
    }

    pub(super) fn open_about_dialog(&mut self, _: nwg::EventData) {
        self.c.window.set_enabled(false);
        let args = AboutDialogArgs::new(&self.c.about_notice);
        self.about_dialog_join_handle = AboutDialog::popup(args);
    }

    pub(super) fn await_about_dialog(&mut self, _: nwg::EventData) {
        self.c.window.set_enabled(true);
        self.c.about_notice.receive();
        let _ = self.about_dialog_join_handle.join();
        self.c.filter_input.set_enabled(true);
    }

    pub(super) fn open_connect_dialog(&mut self, _: nwg::EventData) {
        self.c.window.set_enabled(false);
        let args = ConnectDialogArgs::new(&self.c.connect_notice, self.pg_conn_config.clone());
        self.connect_dialog_join_handle = ConnectDialog::popup(args);
    }

    pub(super) fn await_connect_dialog(&mut self, _: nwg::EventData) {
        self.c.window.set_enabled(true);
        self.c.connect_notice.receive();
        let res = self.connect_dialog_join_handle.join();
        if res.load_settings_requested {
            self.pg_conn_config = res.pg_conn_config;
            let sbar_label = format!(
                "{}:{}", &self.pg_conn_config.hostname, &self.pg_conn_config.port);
            self.set_status_bar_dbconn_label(&sbar_label);
            self.open_load_dialog(nwg::EventData::NoData);
        } else {
            self.c.filter_input.set_enabled(true);
        }
    }

    pub(super) fn open_load_dialog(&mut self, _: nwg::EventData) {
        self.settings.truncate(0);
        self.reload_settings_view();
        self.c.window.set_enabled(false);
        let args = LoadSettingsDialogArgs::new(&self.c.load_settings_notice, self.pg_conn_config.clone());
        self.load_settings_dialog_join_handle = LoadSettingsDialog::popup(args);
    }

    pub(super) fn await_load_dialog(&mut self, _: nwg::EventData) {
        self.c.window.set_enabled(true);
        self.c.load_settings_notice.receive();
        let res = self.load_settings_dialog_join_handle.join();
        self.settings = res.records;
        self.reload_settings_view();
        self.c.filter_input.set_enabled(true);
    }

    pub(super) fn open_setting_dialog(&mut self, ed: nwg::EventData) {
        let row_idx = if let nwg::EventData::OnListViewItemIndex
        { row_index: row_idx, .. } = ed {
            row_idx
        } else {
            return;
        };
        let name = match self.c.settings_view.item(row_idx, 0, 1<<16) {
            Some(item) => item.text,
            None => return
        };
        let setting = match self.c.settings_view.item(row_idx, 1, 1<<16) {
            Some(item) => item.text,
            None => return
        };
        let description = match self.c.settings_view.item(row_idx, 2, 1<<16) {
            Some(item) => item.text,
            None => return
        };
        let st = SettingRecord {
            name, setting, description
        };
        self.c.window.set_enabled(false);
        let args = SettingDialogArgs::new(&self.c.setting_notice, self.pg_conn_config.clone(), st);
        self.setting_dialog_join_handle = SettingDialog::popup(args);
    }

    pub(super) fn await_setting_dialog(&mut self, _: nwg::EventData) {
        self.c.window.set_enabled(true);
        self.c.setting_notice.receive();
        let _ = self.setting_dialog_join_handle.join();
    }

    pub(super) fn open_website(&mut self, _: nwg::EventData) {
        let create_no_window: u32 = 0x08000000;
        let _ = Command::new("cmd")
            .arg("/c")
            .arg("start")
            .arg("https://wiltondb.com")
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .creation_flags(create_no_window)
            .status();
    }

    pub(super) fn on_settings_view_sort(&mut self, ed: nwg::EventData) {
        let col_idx = if let nwg::EventData::OnListViewItemIndex
                { column_index: col_idx, .. } = ed {
            col_idx
        } else {
           return;
        };
        if 0 != col_idx {
            return;
        }
        let old_arrow = self.c.settings_view
            .column_sort_arrow(col_idx)
            .expect("Sort not initialized");
        let arrow = match old_arrow {
            nwg::ListViewColumnSortArrow::Up => nwg::ListViewColumnSortArrow::Down,
            nwg::ListViewColumnSortArrow::Down => nwg::ListViewColumnSortArrow::Up
        };
        let desc = match arrow {
            nwg::ListViewColumnSortArrow::Up => true,
            nwg::ListViewColumnSortArrow::Down => false
        };
        self.sort_settings(col_idx, desc);
        self.c.settings_view.set_column_sort_arrow(col_idx, Some(arrow));
        self.reload_settings_view();
    }

    pub(super) fn on_filter_button(&mut self, _: nwg::EventData) {
        self.reload_settings_view()
    }

    pub(super) fn on_filter_combo(&mut self, _: nwg::EventData) {
        self.reload_settings_view()
    }

    fn get_cmd_args() -> Vec<String> {
        let mut res = vec!();
        for aos in env::args_os() {
            match aos.into_string() {
                Ok(st) => res.push(st),
                Err(_) => {/* ignore */}
            }
        };
        res
    }

    fn set_status_bar_dbconn_label(&self, text: &str) {
        self.c.status_bar.set_text(0, &format!("  DB connection: {}", text));
    }

    fn setting_matches_filters(&self, name: &str) -> bool {
        let filter_group_names = self.c.filter_combo.collection();
        let filter_group_idx = match self.c.filter_combo.selection() {
            Some(idx) => idx,
            None => 0
        };
        if filter_group_idx > 0 {
            let group_name = &filter_group_names[filter_group_idx];
            let empty = HashSet::<String>::new();
            let group = if setting_groups::NETWORKING == group_name {
                &self.networking_settings
            } else if setting_groups::LOGGING == group_name {
                &self.logging_settings
            } else if setting_groups::MEMORY == group_name {
                &self.memory_settings
            } else if setting_groups::ESCAPE_HATCHES == group_name {
                &self.escape_hatch_settings
            } else {
                &empty
            };
            if !group.contains(name) {
                return false;
            }
        }

        let filter = self.c.filter_input.text();
        if 0 == filter.len() {
            return true;
        }
        if name.starts_with(&filter) {
            return true;
        }
        WildMatch::new(&filter).matches(name)
    }

    fn reload_settings_view(&self) {
        let sv = &self.c.settings_view;
        sv.set_redraw(false);
        loop {
            let removed = sv.remove_item(0);
            if !removed {
                break;
            }
        };
        let mut idx = 0 as i32;
        for rec in &self.settings {
            if self.setting_matches_filters(&rec.name) {
                sv.insert_item(nwg::InsertListViewItem {
                    index: Some(idx as i32),
                    column_index: 0,
                    text: Some(rec.name.clone()),
                    image: None
                });
                sv.insert_item(nwg::InsertListViewItem {
                    index: Some(idx as i32),
                    column_index: 1,
                    text: Some(rec.setting.clone()),
                    image: None
                });
                sv.insert_item(nwg::InsertListViewItem {
                    index: Some(idx as i32),
                    column_index: 2,
                    text: Some(rec.description.clone()),
                    image: None
                });
                idx += 1;
            }
        }
        sv.set_redraw(true);
    }

    fn sort_settings(&mut self, col_idx: usize, desc: bool) {
        if 0 != col_idx {
            return;
        }
        self.settings.sort_by(|a, b| {
            if desc {
                b.name.to_lowercase().cmp(&a.name.to_lowercase())
            } else {
                a.name.to_lowercase().cmp(&b.name.to_lowercase())
            }
        });
    }

}
