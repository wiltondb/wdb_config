
use std::cell::RefCell;
use std::thread::JoinHandle;

use crate::*;
use dialogs::PopupDialog;
use app_window_ui::AppWindowUi;

#[derive(Default)]
pub struct AppWindow {
    pub ui: AppWindowUi,

    dialog_data: RefCell<Option<JoinHandle<String>>>,
}

impl AppWindow {

    pub fn new() -> Self {
        Default::default()
    }

    pub fn exit(&self) {
        nwg::stop_thread_dispatch();
    }

    pub fn load_data(&self) {
        let dv = &self.ui.data_view;

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
    }

    pub fn open_connect_dialog(&self) {
        self.ui.window.set_enabled(false);
        *self.dialog_data.borrow_mut() = Some(about_dialog::AboutDialog::popup(self.ui.dialog_notice.sender()));
    }

    pub fn read_dialog_output(&self) {
        self.ui.window.set_enabled(true);
        self.ui.dialog_notice.receive();

        let data = self.dialog_data.borrow_mut().take();
        match data {
            Some(handle) => {
                let dialog_result = handle.join().unwrap();
                self.ui.status_bar.set_text(0, &dialog_result);
            },
            None => {}
        }
    }

    /*
    fn on_close(&self) {
        nwg::stop_thread_dispatch();
        std::process::exit(1);
    }
     */

    /*
    pub fn init_events(&mut self) {
        self.events = ui_events::Events::builder()
            .add(self.window.handle, nwg::Event::OnWindowClose, Self::exit)
            .add(self.window.handle, nwg::Event::OnWindowClose, Self::load_data)
            .build()
    }
     */
}
