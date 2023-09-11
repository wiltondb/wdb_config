
use super::*;
use nwg::NwgError;

#[derive(Default)]
pub(super) struct SettingDialogEvents {
    pub(super) events: Vec<ui::Event<SettingDialog>>
}

impl ui::Events<SettingDialogControls> for SettingDialogEvents {
    fn build(&mut self, c: &SettingDialogControls) -> Result<(), NwgError> {
        ui::event_builder()
            .control(&c.close_button)
            .event(nwg::Event::OnButtonClick)
            .handler(SettingDialog::close)
            .build(&mut self.events)?;

        Ok(())
    }
}
