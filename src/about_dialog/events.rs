
use super::*;

#[derive(Default)]
pub(super) struct AboutDialogEvents {
   pub(super) events: Vec<ui::Event<AboutDialog>>
}

impl ui::Events<AboutDialogControls> for AboutDialogEvents {
    fn build(&mut self, c: &AboutDialogControls) -> Result<(), nwg::NwgError> {
        ui::event_builder()
            .control(&c.window)
            .event(nwg::Event::OnWindowClose)
            .handler(AboutDialog::close)
            .build(&mut self.events)?;
        ui::event_builder()
            .control(&c.window)
            .event(nwg::Event::OnResizeEnd)
            .handler(AboutDialog::on_resize)
            .build(&mut self.events)?;

        ui::event_builder()
            .control(&c.close_button)
            .event(nwg::Event::OnButtonClick)
            .handler(AboutDialog::close)
            .build(&mut self.events)?;

        Ok(())
    }
}