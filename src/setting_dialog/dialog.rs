
use super::*;

#[derive(Default)]
pub struct SettingDialog {
    pub(super) c: SettingDialogControls,

    args: SettingDialogArgs,
    change_join_handle: ui::PopupJoinHandle<()>,
}

impl ui::PopupDialog<SettingDialogArgs, ()> for SettingDialog {
    fn popup(args: SettingDialogArgs) -> PopupJoinHandle<()> {
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
        self.c.name_input.set_text(&self.args.setting.name);
        self.c.current_value_input.set_text(&self.args.setting.setting);
    }

    fn result(&mut self) -> () {
        ()
    }

    fn close(&mut self, _: nwg::EventData) {
        self.args.notify_parent();
        self.c.window.set_visible(false);
        nwg::stop_thread_dispatch();
    }
}
