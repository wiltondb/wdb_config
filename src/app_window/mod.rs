
mod controls;
mod events;
mod layout;
mod nui;

use std::cell::RefCell;
use std::os::windows::process::CommandExt;
use std::process::Command;
use std::process::Stdio;

use crate::*;
use nwg_ui as ui;
use ui::Controls;
use ui::PopupDialog;

use about_dialog::AboutDialog;
use about_dialog::AboutDialogArgs;
use connect_dialog::ConnectConfig;
use connect_dialog::ConnectDialog;
use connect_dialog::ConnectDialogArgs;
use load_settings_dialog::LoadSettingsDialog;
use load_settings_dialog::LoadSettingsDialogArgs;
use load_settings_dialog::LoadSettingsDialogResult;
pub(self) use controls::AppWindowControls;
pub(self) use events::AppWindowEvents;
use layout::AppWindowLayout;

#[derive(Default)]
pub struct AppWindow {
    pub(self) controls: AppWindowControls,
    pub(self) layout: AppWindowLayout,
    pub(self) events: AppWindowEvents,

    config: RefCell<ConnectConfig>,

    about_dialog_joiner: ui::PopupJoiner<()>,
    connect_dialog_joiner: ui::PopupJoiner<ConnectConfig>,
    load_settings_dialog_joiner: ui::PopupJoiner<LoadSettingsDialogResult>,
}

impl AppWindow {

    pub fn new() -> Self {
        Default::default()
    }

    #[allow(dead_code)]
    pub fn load_data(&self) {
        /*
        let dv = &self.controls.data_view;

        dv.insert_column("Name");
        dv.set_column_sort_arrow(0, Some(nwg::ListViewColumnSortArrow::Down));
        dv.insert_column(nwg::InsertListViewColumn{
            index: Some(1),
            fmt: Some(nwg::ListViewColumnFlags::RIGHT),
            width: None, //Some(20),
            text: Some("test".into())
        });
        dv.set_headers_enabled(true);

// Passing a str to this method will automatically push the item at the end of the list in the first column
        dv.insert_item("Cat");
        dv.insert_item(nwg::InsertListViewItem {
            index: Some(0),
            column_index: 1,
            text: Some("Felis".into()),
            image: None
        });

// To insert a new row, use the index 0.
        dv.insert_item(nwg::InsertListViewItem {
            index: Some(0),
            column_index: 0,
            text: Some("Moose".into()),
            image: Some(1),
        });

        dv.insert_item(nwg::InsertListViewItem {
            index: Some(0),
            column_index: 1,
            text: Some("Alces".into()),
            image: None,
        });

// Insert multiple item on a single row.
        dv.insert_items_row(None, &["Dog", "Canis"]);

// Insert many item at one
        dv.insert_items(&["Duck", "Horse", "Boomalope"]);
        dv.insert_items(&[
            nwg::InsertListViewItem { index: Some(3), column_index: 1, text: Some("Anas".into()), image: None },
            nwg::InsertListViewItem { index: Some(4), column_index: 1, text: Some("Equus".into()), image: None },
        ]);

// Update items
        dv.update_item(2, nwg::InsertListViewItem { image: Some(1), ..Default::default() });
        dv.update_item(4, nwg::InsertListViewItem { image: Some(1), ..Default::default() });
         */
    }

    pub fn open_about_dialog(&self) {
        self.controls.window.set_enabled(false);
        let args = AboutDialogArgs::new(&self.controls.about_notice);
        let join_handle = AboutDialog::popup(args);
        self.about_dialog_joiner.set_join_handle(join_handle);
    }

    pub fn await_about_dialog(&self) {
        self.controls.window.set_enabled(true);
        self.controls.about_notice.receive();
        let _ = self.about_dialog_joiner.await_result();
    }

    pub fn open_connect_dialog(&self) {
        self.controls.window.set_enabled(false);
        let args = ConnectDialogArgs::new(&self.controls.connect_notice, self.config.borrow().clone());
        let join_handle = ConnectDialog::popup(args);
        self.connect_dialog_joiner.set_join_handle(join_handle);
    }

    pub fn await_connect_dialog(&self) {
        self.controls.window.set_enabled(true);
        self.controls.connect_notice.receive();
        let config = self.connect_dialog_joiner.await_result();
        self.config.replace(config);
        self.set_status_bar_hostname(&self.config.borrow().hostname);
    }

    pub fn set_status_bar_hostname(&self, text: &str) {
        self.controls.status_bar.set_text(0, &format!("  DB host: {}", text));
    }

    pub fn open_load_dialog(&self) {
        self.controls.window.set_enabled(false);
        let args = LoadSettingsDialogArgs::new(&self.controls.load_settings_notice, self.config.borrow().clone());
        let join_handle = LoadSettingsDialog::popup(args);
        self.load_settings_dialog_joiner.set_join_handle(join_handle);
    }

    pub fn await_load_dialog(&self) {
        self.controls.window.set_enabled(true);
        self.controls.load_settings_notice.receive();
        let res = self.load_settings_dialog_joiner.await_result();
        // todo
        self.set_status_bar_hostname(&res.records.len().to_string());
    }

    pub fn open_website(&self) {
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

    pub fn close(&self) {
        self.controls.hide_window();
        nwg::stop_thread_dispatch();
    }

}
