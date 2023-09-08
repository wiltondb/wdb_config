
use super::*;

#[derive(Default)]
pub struct AboutDialog {
    pub(super) c: AboutDialogControls,
    pub(super) layout: AboutDialogLayout,
    pub(super) events: AboutDialogEvents,

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

    fn init(&self) {
        ()
    }

    fn result(&self) -> () {
        ()
    }

    fn close(&self) {
        self.args.notify_parent();
        self.c.hide_window();
        nwg::stop_thread_dispatch();
    }
}
