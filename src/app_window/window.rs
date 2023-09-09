
use std::os::windows::process::CommandExt;
use std::process::Command;
use std::process::Stdio;

use super::*;

#[derive(Default)]
pub struct AppWindow {
    pub(super) c: AppWindowControls,

    config: ConnectConfig,
    settings: Vec<SettingRecord>,

    about_dialog_join_handle: ui::PopupJoinHandle<()>,
    connect_dialog_join_handle: ui::PopupJoinHandle<ConnectConfig>,
    load_settings_dialog_join_handle: ui::PopupJoinHandle<LoadSettingsDialogResult>,
}

impl AppWindow {

    pub fn new() -> Self {
        Default::default()
    }

    pub(super) fn init(&mut self) {
        self.config.hostname = String::from("localhost");
        self.config.port = 5432;
        self.config.username = String::from("wilton");
        // todo: removeme
        self.config.password = String::from("wilton");
        self.config.enable_tls = true;
        self.config.accept_invalid_tls = true;

        self.open_connect_dialog(nwg::EventData::NoData);
    }

    pub(super) fn close(&mut self, _: nwg::EventData) {
        self.c.hide_window();
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
    }

    pub(super) fn open_connect_dialog(&mut self, _: nwg::EventData) {
        self.c.window.set_enabled(false);
        let args = ConnectDialogArgs::new(&self.c.connect_notice, self.config.clone());
        self.connect_dialog_join_handle = ConnectDialog::popup(args);
    }

    pub(super) fn await_connect_dialog(&mut self, _: nwg::EventData) {
        self.c.window.set_enabled(true);
        self.c.connect_notice.receive();
        self.config = self.connect_dialog_join_handle.join();
        self.set_status_bar_hostname(&self.config.hostname);
    }

    pub(super) fn set_status_bar_hostname(&self, text: &str) {
        self.c.status_bar.set_text(0, &format!("  DB host: {}", text));
    }

    pub(super) fn open_load_dialog(&mut self, _: nwg::EventData) {
        self.settings.truncate(0);
        self.refresh_settings_view();
        self.c.window.set_enabled(false);
        let args = LoadSettingsDialogArgs::new(&self.c.load_settings_notice, self.config.clone());
        self.load_settings_dialog_join_handle = LoadSettingsDialog::popup(args);
    }

    pub(super) fn await_load_dialog(&mut self, _: nwg::EventData) {
        self.c.window.set_enabled(true);
        self.c.load_settings_notice.receive();
        let res = self.load_settings_dialog_join_handle.join();
        self.settings = res.records;
        self.refresh_settings_view();
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
        self.refresh_settings_view();
    }

    fn refresh_settings_view(&self) {
        let sv = &self.c.settings_view;
        sv.set_redraw(false);
        loop {
            let removed = sv.remove_item(0);
            if !removed {
                break;
            }
        };
        for idx in 0..self.settings.len() {
            let rec = &self.settings[idx];
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
