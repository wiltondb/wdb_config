
use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;

use nwg::stretch::style::JustifyContent;
use nwg::stretch::style::FlexDirection;

use crate::*;
use dialogs::DialogUi;
use app_window::AppWindow;
use nwg::{NwgError, Window};

#[derive(Default)]
pub struct AppWindowUi {
    events: events::Events<AppWindow>,
    font_normal: nwg::Font,
    font_small: nwg::Font,

    window: nwg::Window,

    file_menu: nwg::Menu,
    file_connect_menu_item: nwg::MenuItem,
    file_exit_menu_item: nwg::MenuItem,
    help_menu: nwg::Menu,
    help_about_menu_item: nwg::MenuItem,
    help_website_menu_item: nwg::MenuItem,

    main_view: nwg::ListView,
    reload_button: nwg::Button,
    close_button: nwg::Button,
    status_bar: nwg::StatusBar,

    root_layout: nwg::FlexboxLayout,
    main_view_layout: nwg::FlexboxLayout,
    buttons_layout: nwg::FlexboxLayout,

    about_dialog_notice: notice::SyncNotice,
    connect_dialog_notice: notice::SyncNotice,
    load_settings_dialog_notice: notice::SyncNotice,
}

impl AppWindowUi {
    pub fn about_dialog_notice(&self) -> &notice::SyncNotice {
        &self.about_dialog_notice
    }

    pub fn connect_dialog_notice(&self) -> &notice::SyncNotice {
        &self.connect_dialog_notice
    }

    pub fn load_settings_dialog_notice(&self) -> &notice::SyncNotice {
        &self.load_settings_dialog_notice
    }

    pub fn set_status_bar_hostname(&self, hostname: &str) {
        self.status_bar.set_text(0, &format!("  DB host: {}", hostname));
    }
}

impl DialogUi for AppWindowUi {
    fn window(&self) -> &Window {
        &self.window
    }

    fn build_controls(&mut self) -> Result<(), NwgError> {
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
        events::builder()
            .control(&self.window)
            .event(nwg::Event::OnWindowClose)
            .handler(AppWindow::close)
            .build(&mut self.events)?;

        // menu

        nwg::Menu::builder()
            .parent(&self.window)
            .text("File")
            .build(&mut self.file_menu)?;
        nwg::MenuItem::builder()
            .parent(&self.file_menu)
            .text("Connect to DB")
            .build(&mut self.file_connect_menu_item)?;
        events::builder()
            .control(&self.file_connect_menu_item)
            .event(nwg::Event::OnMenuItemSelected)
            .handler(AppWindow::open_connect_dialog)
            .build(&mut self.events)?;
        nwg::MenuItem::builder()
            .parent(&self.file_menu)
            .text("Exit")
            .build(&mut self.file_exit_menu_item)?;
        events::builder()
            .control(&self.file_exit_menu_item)
            .event(nwg::Event::OnMenuItemSelected)
            .handler(AppWindow::close)
            .build(&mut self.events)?;

        nwg::Menu::builder()
            .parent(&self.window)
            .text("Help")
            .build(&mut self.help_menu)?;
        nwg::MenuItem::builder()
            .parent(&self.help_menu)
            .text("About")
            .build(&mut self.help_about_menu_item)?;
        events::builder()
            .control(&self.help_about_menu_item)
            .event(nwg::Event::OnMenuItemSelected)
            .handler(AppWindow::open_about_dialog)
            .build(&mut self.events)?;
        nwg::MenuItem::builder()
            .parent(&self.help_menu)
            .text("Website")
            .build(&mut self.help_website_menu_item)?;
        events::builder()
            .control(&self.help_website_menu_item)
            .event(nwg::Event::OnMenuItemSelected)
            .handler(AppWindow::open_website)
            .build(&mut self.events)?;

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
        events::builder()
            .control(&self.reload_button)
            .event(nwg::Event::OnButtonClick)
            .handler(AppWindow::open_load_dialog)
            .build(&mut self.events)?;

        nwg::Button::builder()
            .text("Close")
            .font(Some(&self.font_normal))
            .parent(&self.window)
            .build(&mut self.close_button)?;
        events::builder()
            .control(&self.close_button)
            .event(nwg::Event::OnButtonClick)
            .handler(AppWindow::close)
            .build(&mut self.events)?;

        // other

        nwg::StatusBar::builder()
            .parent(&self.window)
            .text("  DB host: none")
            .font(Some(&self.font_small))
            .build(&mut self.status_bar)?;

        notice::builder()
            .parent(&self.window)
            .build(&mut self.about_dialog_notice)?;
        events::builder()
            .control(&self.about_dialog_notice.notice)
            .event(nwg::Event::OnNotice)
            .handler(AppWindow::await_about_dialog)
            .build(&mut self.events)?;

        notice::builder()
            .parent(&self.window)
            .build(&mut self.connect_dialog_notice)?;
        events::builder()
            .control(&self.connect_dialog_notice.notice)
            .event(nwg::Event::OnNotice)
            .handler(AppWindow::await_connect_dialog)
            .build(&mut self.events)?;

        notice::builder()
            .parent(&self.window)
            .build(&mut self.load_settings_dialog_notice)?;
        events::builder()
            .control(&self.load_settings_dialog_notice.notice)
            .event(nwg::Event::OnNotice)
            .handler(AppWindow::await_load_dialog)
            .build(&mut self.events)?;

        Ok(())
    }

    fn build_layout(&mut self) -> Result<(), NwgError> {

        nwg::FlexboxLayout::builder()
            .parent(&self.window)
            .flex_direction(FlexDirection::Row)
            .child(&self.main_view)
            .child_flex_grow(1.0)
            .auto_spacing(None)
            .build_partial(&self.main_view_layout)?;

        nwg::FlexboxLayout::builder()
            .parent(&self.window)
            .flex_direction(FlexDirection::Row)
            .justify_content(JustifyContent::FlexEnd)
            .auto_spacing(None)
            .child(&self.reload_button)
            .child_size(ui::size_builder()
                .width_button_wide()
                .height_button()
                .build())
            .child_margin(ui::margin_builder()
                .bottom_pt(22)
                .build())
            .child(&self.close_button)
            .child_size(ui::size_builder()
                .width_button_normal()
                .height_button()
                .build())
            .child_margin(ui::margin_builder()
                .start_pt(5)
                .bottom_pt(22)
                .build())
            .build_partial(&self.buttons_layout)?;

        nwg::FlexboxLayout::builder()
            .parent(&self.window)
            .flex_direction(FlexDirection::Column)
            .child_layout(&self.main_view_layout)
            .child_flex_grow(1.0)
            .child_layout(&self.buttons_layout)
            .build(&self.root_layout)?;

        Ok(())
    }
}

pub struct AppWindowNwg {
    inner: Rc<AppWindow>,
    default_handler: RefCell<Option<nwg::EventHandler>>
}

impl AppWindowNwg {
    pub fn open_connect_dialog(&self) {
        self.inner.open_connect_dialog();
    }
}

impl nwg::NativeUi<AppWindowNwg> for AppWindow {
    fn build_ui(mut data: AppWindow) -> Result<AppWindowNwg, nwg::NwgError> {
        data.ui_mut().build_controls()?;
        data.ui_mut().build_layout()?;

        let wrapper = AppWindowNwg {
            inner:  Rc::new(data),
            default_handler: Default::default(),
        };

        let data_ref = Rc::downgrade(&wrapper.inner);
        let handle_events = move |evt, _evt_data, handle| {
            if let Some(evt_data) = data_ref.upgrade() {
                for eh in evt_data.ui().events.iter() {
                    if handle == eh.control_handle && evt == eh.event {
                        (eh.handler)(&evt_data);
                        break;
                    }
                }
            }
        };

        *wrapper.default_handler.borrow_mut() = Some(nwg::full_bind_event_handler(&wrapper.ui().window.handle, handle_events));

        return Ok(wrapper);
    }

}

impl Drop for AppWindowNwg {
    fn drop(&mut self) {
        let handler = self.default_handler.borrow();
        if handler.is_some() {
            nwg::unbind_event_handler(handler.as_ref().unwrap());
        }
    }
}

impl Deref for AppWindowNwg {
    type Target = AppWindow;

    fn deref(&self) -> &AppWindow {
        &self.inner
    }
}
