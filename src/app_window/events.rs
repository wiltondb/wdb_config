
use super::*;

#[derive(Default)]
pub(super) struct AppWindowEvents {
    pub(super) events: Vec<ui::Event<AppWindow>>
}

impl ui::Events<AppWindowControls> for AppWindowEvents {
    fn build(&mut self, c: &AppWindowControls) -> Result<(), nwg::NwgError> {
        ui::event_builder()
            .control(&c.window)
            .event(nwg::Event::OnWindowClose)
            .handler(AppWindow::close)
            .build(&mut self.events)?;
        ui::event_builder()
            .control(&c.window)
            .event(nwg::Event::OnResizeEnd)
            .handler(AppWindow::on_resize)
            .build(&mut self.events)?;
        ui::event_builder()
            .control(&c.window)
            .event(nwg::Event::OnKeyEnter)
            .handler(AppWindow::on_filter_button)
            .build(&mut self.events)?;

        ui::event_builder()
            .control(&c.file_connect_menu_item)
            .event(nwg::Event::OnMenuItemSelected)
            .handler(AppWindow::open_connect_dialog)
            .build(&mut self.events)?;
        ui::event_builder()
            .control(&c.file_exit_menu_item)
            .event(nwg::Event::OnMenuItemSelected)
            .handler(AppWindow::close)
            .build(&mut self.events)?;

        ui::event_builder()
            .control(&c.help_about_menu_item)
            .event(nwg::Event::OnMenuItemSelected)
            .handler(AppWindow::open_about_dialog)
            .build(&mut self.events)?;
        ui::event_builder()
            .control(&c.help_website_menu_item)
            .event(nwg::Event::OnMenuItemSelected)
            .handler(AppWindow::open_website)
            .build(&mut self.events)?;

        ui::event_builder()
            .control(&c.filter_combo)
            .event(nwg::Event::OnComboxBoxSelection)
            .handler(AppWindow::on_filter_combo)
            .build(&mut self.events)?;

        ui::event_builder()
            .control(&c.filter_button)
            .event(nwg::Event::OnButtonClick)
            .handler(AppWindow::on_filter_button)
            .build(&mut self.events)?;

        ui::event_builder()
            .control(&c.settings_view)
            .event(nwg::Event::OnListViewColumnClick)
            .handler(AppWindow::on_settings_view_sort)
            .build(&mut self.events)?;
        ui::event_builder()
            .control(&c.settings_view)
            .event(nwg::Event::OnListViewDoubleClick)
            .handler(AppWindow::open_setting_dialog)
            .build(&mut self.events)?;

        ui::event_builder()
            .control(&c.reload_button)
            .event(nwg::Event::OnButtonClick)
            .handler(AppWindow::open_load_dialog)
            .build(&mut self.events)?;

        ui::event_builder()
            .control(&c.close_button)
            .event(nwg::Event::OnButtonClick)
            .handler(AppWindow::close)
            .build(&mut self.events)?;

        ui::event_builder()
            .control(&c.about_notice.notice)
            .event(nwg::Event::OnNotice)
            .handler(AppWindow::await_about_dialog)
            .build(&mut self.events)?;
        ui::event_builder()
            .control(&c.connect_notice.notice)
            .event(nwg::Event::OnNotice)
            .handler(AppWindow::await_connect_dialog)
            .build(&mut self.events)?;
        ui::event_builder()
            .control(&c.load_settings_notice.notice)
            .event(nwg::Event::OnNotice)
            .handler(AppWindow::await_load_dialog)
            .build(&mut self.events)?;
        ui::event_builder()
            .control(&c.setting_notice.notice)
            .event(nwg::Event::OnNotice)
            .handler(AppWindow::await_setting_dialog)
            .build(&mut self.events)?;

        Ok(())
    }
}
