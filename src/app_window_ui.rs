
use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;

use nwg::stretch::geometry::Size;
use nwg::stretch::style::FlexDirection;

use crate::*;
use dialogs::DialogUi;
use app_window::AppWindow;
use nwg::{NwgError, Window};

#[derive(Default)]
pub struct AppWindowUi {
    events: events::Events<AppWindow>,

    window: nwg::Window,
    button1: nwg::Button,
    button2: nwg::Button,
    pub data_view: nwg::ListView,
    button5: nwg::Button,
    button6: nwg::Button,
    pub status_bar: nwg::StatusBar,

    file_menu: nwg::Menu,
    file_connect_menu_item: nwg::MenuItem,
    file_exit_menu_item: nwg::MenuItem,
    help_menu: nwg::Menu,
    help_about_menu_item: nwg::MenuItem,
    help_website_menu_item: nwg::MenuItem,

    root_layout: nwg::FlexboxLayout,
    row1_layout: nwg::FlexboxLayout,
    row2_layout: nwg::FlexboxLayout,
    row3_layout: nwg::FlexboxLayout,

    small_font: nwg::Font,

    pub about_dialog_notice: notice::SyncNotice,
    pub connect_dialog_notice: notice::SyncNotice,
}

impl DialogUi for AppWindowUi {
    fn window(&self) -> &Window {
        &self.window
    }

    fn build_controls(&mut self) -> Result<(), NwgError> {
        // font
        nwg::Font::builder()
            .size(14)
            //.weight(1000)
            .build(&mut self.small_font)?;

        // window

        nwg::Window::builder()
            .size((640, 480))
            .center(true)
            .title("WiltonDB Configuration Tool")
            .build(&mut self.window)?;
        events::builder()
            .control(&self.window)
            .event(nwg::Event::OnWindowClose)
            .handler(AppWindow::exit)
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
            .handler(AppWindow::exit)
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

        // buttons

        nwg::Button::builder()
            .text("Btn 1")
            .parent(&self.window)
            .focus(true)
            .build(&mut self.button1)?;

        nwg::Button::builder()
            .text("Btn 2")
            .parent(&self.window)
            .build(&mut self.button2)?;
        events::builder()
            .control(&self.button2)
            .event(nwg::Event::OnButtonClick)
            .handler(AppWindow::load_data)
            .build(&mut self.events)?;

        nwg::Button::builder()
            .text("Btn 5")
            .parent(&self.window)
            .build(&mut self.button5)?;

        nwg::Button::builder()
            .text("Btn 6")
            .parent(&self.window)
            .build(&mut self.button6)?;

        // other

        nwg::ListView::builder()
            .parent(&self.window)
            .item_count(10)
            .list_style(nwg::ListViewStyle::Detailed)
            .focus(true)
            .ex_flags(nwg::ListViewExFlags::GRID | nwg::ListViewExFlags::FULL_ROW_SELECT)
            .build(&mut self.data_view)?;

        nwg::StatusBar::builder()
            .parent(&self.window)
            .text("Ready for tests")
            .font(Some(&self.small_font))
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

        Ok(())
    }

    fn build_layout(&mut self) -> Result<(), NwgError> {
        nwg::FlexboxLayout::builder()
            .parent(&self.window)
            .flex_direction(FlexDirection::Row)
            .child(&self.button1)
            .child_size(ui::size_builder()
                .width_button_normal()
                .height_button()
                .build())
            .child(&self.button2)
            .child_size(ui::size_builder()
                .width_button_normal()
                .height_button()
                .build())
            .child_flex_grow(1.0)
            .build_partial(&self.row1_layout)?;

        nwg::FlexboxLayout::builder()
            .parent(&self.window)
            .flex_direction(FlexDirection::Row)
            .child(&self.data_view)
            .child_flex_grow(1.0)
            .build_partial(&self.row2_layout)?;

        nwg::FlexboxLayout::builder()
            .parent(&self.window)
            .flex_direction(FlexDirection::Row)
            .child(&self.button5)
            .child_size(ui::size_builder()
                .width_button_normal()
                .height_button()
                .build())
            .child(&self.button6)
            .child_size(ui::size_builder()
                .width_button_normal()
                .height_button()
                .build())
            .child_flex_grow(1.0)
            .build_partial(&self.row3_layout)?;

        nwg::FlexboxLayout::builder()
            .parent(&self.window)
            .flex_direction(FlexDirection::Column)
            .child_layout(&self.row1_layout)
            .child_layout(&self.row2_layout)
            .child_flex_grow(1.0)
            .child_layout(&self.row3_layout)
            .build(&self.root_layout)?;

        Ok(())
    }
}

pub struct AppWindowNwg {
    inner: Rc<AppWindow>,
    default_handler: RefCell<Option<nwg::EventHandler>>
}

impl nwg::NativeUi<AppWindowNwg> for AppWindow {
    fn build_ui(mut data: AppWindow) -> Result<AppWindowNwg, nwg::NwgError> {
        data.ui.build_controls()?;
        data.ui.build_layout()?;
        data.ui.shake_after_layout();

        let wrapper = AppWindowNwg {
            inner:  Rc::new(data),
            default_handler: Default::default(),
        };

        let data_ref = Rc::downgrade(&wrapper.inner);
        let handle_events = move |evt, _evt_data, handle| {
            if let Some(evt_data) = data_ref.upgrade() {
                for eh in evt_data.ui.events.iter() {
                    if handle == eh.control_handle && evt == eh.event {
                        (eh.handler)(&evt_data);
                        break;
                    }
                }
            }
        };

        *wrapper.default_handler.borrow_mut() = Some(nwg::full_bind_event_handler(&wrapper.ui.window.handle, handle_events));

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
