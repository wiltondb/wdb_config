
use super::*;
use nwg::NwgError;

#[derive(Default)]
pub(super) struct SettingDialogEvents {
    pub(super) events: Vec<ui::Event<SettingDialog>>
}

impl ui::Events<SettingDialogControls> for SettingDialogEvents {
    fn build(&mut self, c: &SettingDialogControls) -> Result<(), NwgError> {
        ui::event_builder()
            .control(&c.window)
            .event(nwg::Event::OnWindowClose)
            .handler(SettingDialog::close)
            .build(&mut self.events)?;
        ui::event_builder()
            .control(&c.window)
            .event(nwg::Event::OnResizeEnd)
            .handler(SettingDialog::on_resize)
            .build(&mut self.events)?;

        ui::event_builder()
            .control(&c.change_button)
            .event(nwg::Event::OnButtonClick)
            .handler(SettingDialog::open_change_dialog)
            .build(&mut self.events)?;
        ui::event_builder()
            .control(&c.close_button)
            .event(nwg::Event::OnButtonClick)
            .handler(SettingDialog::close)
            .build(&mut self.events)?;

        ui::event_builder()
            .control(&c.new_value_input)
            .event(nwg::Event::OnTextInput)
            .handler(SettingDialog::on_new_value_change)
            .build(&mut self.events)?;

        ui::event_builder()
            .control(&c.change_notice.notice)
            .event(nwg::Event::OnNotice)
            .handler(SettingDialog::await_change_dialog)
            .build(&mut self.events)?;

        Ok(())
    }
}
