
use std::cell::RefCell;
use std::thread;

use nwg::NativeUi;

use super::*;

#[derive(Default)]
pub struct AboutDialog {
    pub response: RefCell<Option<String>>,

    //#[nwg_control(size: (300, 115), position: (650, 300), title: "A dialog", flags: "WINDOW|VISIBLE")]
    //#[nwg_events( OnWindowClose: [YesNoDialog::close] )]
    pub window: nwg::Window,

    //#[nwg_control(text: "YES", position: (10, 10), size: (130, 95))]
    //#[nwg_events( OnButtonClick: [YesNoDialog::choose(SELF, CTRL)] )]
    pub choice_yes: nwg::Button,

    //#[nwg_control(text: "NO", position: (160, 10), size: (130, 95), focus: true)]
    //#[nwg_events( OnButtonClick: [YesNoDialog::choose(SELF, CTRL)] )]
    pub choice_no: nwg::Button,

    pub events: Vec<events::EventHandler<Self>>,
}

impl AboutDialog {

    /// Create the dialog UI on a new thread. The dialog result will be returned by the thread handle.
    /// To alert the main GUI that the dialog completed, this function takes a notice sender object.
    pub fn popup(sender: nwg::NoticeSender) -> thread::JoinHandle<String> {
        thread::spawn(move || {
            // Create the UI just like in the main function
            let app = AboutDialog::build_ui(Default::default()).expect("Failed to build UI");
            nwg::dispatch_thread_events();

            // Notice the main thread that the dialog completed
            sender.notice();

            // Return the dialog data
            app.response.take().unwrap_or("Cancelled!".to_owned())
        })
    }

    pub fn close(&self) {
        nwg::stop_thread_dispatch();
    }

    /*
    fn choose(&self, btn: &nwg::Button) {
        let mut data = self.response.borrow_mut();
        if btn == &self.choice_no {
            *data = Some("No!".to_string());
        } else if btn == &self.choice_yes {
            *data = Some("Yes!".to_string());
        }

        self.window.close();
    }
     */

}