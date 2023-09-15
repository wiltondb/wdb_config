
use super::*;

#[derive(Default)]
pub(super) struct SettingChangeDialogEvents {
    pub(super) events: Vec<ui::Event<SettingChangeDialog>>
}

impl ui::Events<SettingChangeDialogControls> for SettingChangeDialogEvents {
    fn build(&mut self, c: &SettingChangeDialogControls) -> Result<(), nwg::NwgError> {
        ui::event_builder()
            .control(&c.window)
            .event(nwg::Event::OnWindowClose)
            .handler(SettingChangeDialog::close)
            .build(&mut self.events)?;
        ui::event_builder()
            .control(&c.window)
            .event(nwg::Event::OnResizeEnd)
            .handler(SettingChangeDialog::on_resize)
            .build(&mut self.events)?;

        ui::event_builder()
            .control(&c.copy_clipboard_button)
            .event(nwg::Event::OnButtonClick)
            .handler(SettingChangeDialog::copy_to_clipboard)
            .build(&mut self.events)?;
        ui::event_builder()
            .control(&c.close_button)
            .event(nwg::Event::OnButtonClick)
            .handler(SettingChangeDialog::close)
            .build(&mut self.events)?;
        ui::event_builder()
            .control(&c.change_notice.notice)
            .event(nwg::Event::OnNotice)
            .handler(SettingChangeDialog::on_apply_change_complete)
            .build(&mut self.events)?;

        Ok(())
    }
}
