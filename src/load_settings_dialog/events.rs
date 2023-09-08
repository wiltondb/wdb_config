
use super::*;

#[derive(Default)]
pub(super) struct LoadSettingsDialogEvents {
    pub(super) events: Vec<ui::Event<LoadSettingsDialog>>
}

impl ui::Events<LoadSettingsDialogControls> for LoadSettingsDialogEvents {
    fn build(&mut self, c: &LoadSettingsDialogControls) -> Result<(), nwg::NwgError> {
        ui::event_builder()
            .control(&c.window)
            .event(nwg::Event::OnWindowClose)
            .handler(LoadSettingsDialog::close)
            .build(&mut self.events)?;
        
        ui::event_builder()
            .control(&c.copy_clipboard_button)
            .event(nwg::Event::OnButtonClick)
            .handler(LoadSettingsDialog::copy_to_clipboard)
            .build(&mut self.events)?;

        ui::event_builder()
            .control(&c.close_button)
            .event(nwg::Event::OnButtonClick)
            .handler(LoadSettingsDialog::close)
            .build(&mut self.events)?;

        ui::event_builder()
            .control(&c.load_notice.notice)
            .event(nwg::Event::OnNotice)
            .handler(LoadSettingsDialog::on_load_complete)
            .build(&mut self.events)?;

        Ok(())
    }
}
