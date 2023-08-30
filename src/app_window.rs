
use super::*;

#[derive(Default)]
pub struct AppWindow {
    pub window: nwg::Window,
    pub button1: nwg::Button,
    pub button2: nwg::Button,
    pub data_view: nwg::ListView,
    pub button5: nwg::Button,
    pub button6: nwg::Button,
    pub status_bar: nwg::StatusBar,

    pub file_menu: nwg::Menu,
    pub file_connect_menu_item: nwg::MenuItem,
    pub file_exit_menu_item: nwg::MenuItem,
    pub help_menu: nwg::Menu,
    pub help_about_menu_item: nwg::MenuItem,
    pub help_website_menu_item: nwg::MenuItem,

    pub small_font: nwg::Font,

    pub events: Vec<events::EventHandler<Self>>,

    pub dialog_notice: nwg::Notice,
}

impl AppWindow {

    pub fn new() -> Self {
        Default::default()
    }

    pub fn exit(&self) {
        nwg::stop_thread_dispatch();
    }

    pub fn load_data(&self) {
        let dv = &self.data_view;

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
        about_dialog::AboutDialog::popup(self.dialog_notice.sender());
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
