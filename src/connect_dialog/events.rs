
use crate::*;
use nwg_ui as ui;
use ui::PopupDialog;
use super::*;

#[derive(Default)]
pub(super) struct ConnectDialogEvents {
    pub(super) events: Vec<ui::Event<ConnectDialog>>
}

impl ui::Events<ConnectDialogControls> for ConnectDialogEvents {
    fn build(&mut self, c: &ConnectDialogControls) -> Result<(), nwg::NwgError> {
        ui::event_builder()
            .control(&c.window)
            .event(nwg::Event::OnWindowClose)
            .handler(ConnectDialog::close)
            .build(&mut self.events)?;

        ui::event_builder()
            .control(&c.port_input)
            .event(nwg::Event::OnTextInput)
            .handler(ConnectDialog::on_port_input_changed)
            .build(&mut self.events)?;
        
        ui::event_builder()
            .control(&c.enable_tls_checkbox)
            .event(nwg::Event::OnButtonClick)
            .handler(ConnectDialog::on_enable_tls_checkbox_changed)
            .build(&mut self.events)?;
        
        ui::event_builder()
            .control(&c.test_button)
            .event(nwg::Event::OnButtonClick)
            .handler(ConnectDialog::open_check_dialog)
            .build(&mut self.events)?;

        ui::event_builder()
            .control(&c.close_button)
            .event(nwg::Event::OnButtonClick)
            .handler(ConnectDialog::close)
            .build(&mut self.events)?;

        ui::event_builder()
            .control(&c.check_notice.notice)
            .event(nwg::Event::OnNotice)
            .handler(ConnectDialog::await_check_dialog)
            .build(&mut self.events)?;

        Ok(())
    }
}