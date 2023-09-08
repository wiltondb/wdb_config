
use super::*;

#[derive(Default)]
pub(super) struct ConnectCheckDialogEvents {
    pub(super) events: Vec<ui::Event<ConnectCheckDialog>>
}

impl ui::Events<ConnectCheckDialogControls> for ConnectCheckDialogEvents {
    fn build(&mut self, c: &ConnectCheckDialogControls) -> Result<(), nwg::NwgError> {
        ui::event_builder()
            .control(&c.window)
            .event(nwg::Event::OnWindowClose)
            .handler(ConnectCheckDialog::close)
            .build(&mut self.events)?;
        ui::event_builder()
            .control(&c.copy_clipboard_button)
            .event(nwg::Event::OnButtonClick)
            .handler(ConnectCheckDialog::copy_to_clipboard)
            .build(&mut self.events)?;
        ui::event_builder()
            .control(&c.close_button)
            .event(nwg::Event::OnButtonClick)
            .handler(ConnectCheckDialog::close)
            .build(&mut self.events)?;
        ui::event_builder()
            .control(&c.check_notice.notice)
            .event(nwg::Event::OnNotice)
            .handler(ConnectCheckDialog::on_connection_check_complete)
            .build(&mut self.events)?;

        Ok(())
    }
}
