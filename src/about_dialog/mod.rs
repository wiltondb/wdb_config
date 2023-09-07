
mod args;
mod controls;
mod events;
mod layout;
mod nui;

use std::thread;
use std::thread::JoinHandle;

use nwg::NativeUi;

use crate::*;
use nwg_ui as ui;
use ui::Controls;
use ui::PopupArgs;

pub use args::AboutDialogArgs;
pub(self) use controls::AboutDialogControls;
use events::AboutDialogEvents;
use layout::AboutDialogLayout;

#[derive(Default)]
pub struct AboutDialog {
    pub(self) controls: AboutDialogControls,
    pub(self) layout: AboutDialogLayout,
    pub(self) events: AboutDialogEvents,

    args: AboutDialogArgs,
}

impl AboutDialog {
}

impl ui::PopupDialog<AboutDialogArgs, ()> for AboutDialog {
    fn popup(args: AboutDialogArgs) -> JoinHandle<()> {
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

    fn result(&self) -> () {
        ()
    }

    fn close(&self) {
        self.args.notify_parent();
        self.controls.hide_window();
        nwg::stop_thread_dispatch();
    }
}