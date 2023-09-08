
use super::*;

#[derive(Default)]
pub(super) struct AppWindowControls {
    pub(super) font_normal: nwg::Font,
    pub(super) font_small: nwg::Font,

    pub(super) window: nwg::Window,

    pub(super) file_menu: nwg::Menu,
    pub(super) file_connect_menu_item: nwg::MenuItem,
    pub(super) file_exit_menu_item: nwg::MenuItem,
    pub(super) help_menu: nwg::Menu,
    pub(super) help_about_menu_item: nwg::MenuItem,
    pub(super) help_website_menu_item: nwg::MenuItem,

    pub(super) main_view: nwg::ListView,
    pub(super) reload_button: nwg::Button,
    pub(super) close_button: nwg::Button,
    pub(super) status_bar: nwg::StatusBar,

    pub(super) about_notice: ui::SyncNotice,
    pub(super) connect_notice: ui::SyncNotice,
    pub(super) load_settings_notice: ui::SyncNotice,
}

impl ui::Controls for AppWindowControls {
    fn build(&mut self) -> Result<(), nwg::NwgError> {
        // fonts
        nwg::Font::builder()
            .size(ui::font_size_builder()
                .normal()
                .build())
            .build(&mut self.font_normal)?;
        nwg::Font::builder()
            .size(ui::font_size_builder()
                .small()
                .build())
            .build(&mut self.font_small)?;

        // window

        nwg::Window::builder()
            .size((640, 480))
            .center(true)
            .title("WiltonDB Configuration Tool")
            .build(&mut self.window)?;

        // menu

        nwg::Menu::builder()
            .parent(&self.window)
            .text("File")
            .build(&mut self.file_menu)?;
        nwg::MenuItem::builder()
            .parent(&self.file_menu)
            .text("Connect to DB")
            .build(&mut self.file_connect_menu_item)?;
        nwg::MenuItem::builder()
            .parent(&self.file_menu)
            .text("Exit")
            .build(&mut self.file_exit_menu_item)?;

        nwg::Menu::builder()
            .parent(&self.window)
            .text("Help")
            .build(&mut self.help_menu)?;
        nwg::MenuItem::builder()
            .parent(&self.help_menu)
            .text("About")
            .build(&mut self.help_about_menu_item)?;
        nwg::MenuItem::builder()
            .parent(&self.help_menu)
            .text("Website")
            .build(&mut self.help_website_menu_item)?;

        // main view

        nwg::ListView::builder()
            .parent(&self.window)
            .item_count(10)
            .list_style(nwg::ListViewStyle::Detailed)
            .focus(true)
            .ex_flags(nwg::ListViewExFlags::GRID | nwg::ListViewExFlags::FULL_ROW_SELECT)
            .build(&mut self.main_view)?;

        // buttons

        nwg::Button::builder()
            .text("Load settings")
            .font(Some(&self.font_normal))
            .parent(&self.window)
            .build(&mut self.reload_button)?;
        nwg::Button::builder()
            .text("Close")
            .font(Some(&self.font_normal))
            .parent(&self.window)
            .build(&mut self.close_button)?;

        // other

        nwg::StatusBar::builder()
            .parent(&self.window)
            .text("  DB host: none")
            .font(Some(&self.font_small))
            .build(&mut self.status_bar)?;

        ui::notice_builder()
            .parent(&self.window)
            .build(&mut self.about_notice)?;
        ui::notice_builder()
            .parent(&self.window)
            .build(&mut self.connect_notice)?;
        ui::notice_builder()
            .parent(&self.window)
            .build(&mut self.load_settings_notice)?;


        Ok(())

    }

    fn window(&self) -> &nwg::Window {
        &self.window
    }
}
