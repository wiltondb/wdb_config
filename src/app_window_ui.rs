
//use nwg::stretch::geometry::Rect;
use nwg::stretch::geometry::Size;
use nwg::stretch::style::Dimension;
use nwg::stretch::style::FlexDirection;
//use nwg::stretch::style::AlignSelf;

use crate::*;
use app_window::AppWindow;

//const FIFTY_PC: D = D::Percent(0.5);
const PT_50: Dimension = Dimension::Points(50.0);
//const PADDING: Rect<D> = Rect{ start: PT_10, end: PT_10, top: PT_10, bottom: PT_10 };
//const MARGIN: Rect<D> = Rect{ start: PT_5, end: PT_5, top: PT_5, bottom: PT_5 };

#[derive(Default)]
pub struct AppWindowUi {
    pub events: events::Events<AppWindow>,

    pub window: nwg::Window,
    pub button1: nwg::Button,
    pub button2: nwg::Button,
    pub data_view: nwg::ListView,
    pub button5: nwg::Button,
    pub button6: nwg::Button,
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

    pub dialog_notice: notice::SyncNotice,
}

impl AppWindowUi {
    pub fn build(&mut self) -> Result<(), nwg::NwgError> {
        let data = self;
        create_fonts(data)?;
        create_window(data)?;
        create_menu(data)?;

        nwg::Button::builder()
            .text("Btn 1")
            .parent(&data.window)
            .focus(true)
            .build(&mut data.button1)?;

        nwg::Button::builder()
            .text("Btn 2")
            .parent(&data.window)
            .build(&mut data.button2)?;
        events::builder()
            .control(&data.button2)
            .event(nwg::Event::OnButtonClick)
            .handler(AppWindow::load_data)
            .build(&mut data.events)?;

        nwg::ListView::builder()
            .parent(&data.window)
            .item_count(10)
            .list_style(nwg::ListViewStyle::Detailed)
            .focus(true)
            .ex_flags(nwg::ListViewExFlags::GRID | nwg::ListViewExFlags::FULL_ROW_SELECT)
            .build(&mut data.data_view)?;

        nwg::Button::builder()
            .text("Btn 5")
            .parent(&data.window)
            .build(&mut data.button5)?;

        nwg::Button::builder()
            .text("Btn 6")
            .parent(&data.window)
            .build(&mut data.button6)?;

        nwg::StatusBar::builder()
            .parent(&data.window)
            .text("Ready for tests")
            .font(Some(&data.small_font))
            .build(&mut data.status_bar)?;

        notice::SyncNotice::builder()
            .parent(&data.window)
            .build(&mut data.dialog_notice)?;
        events::builder()
            .control(&data.dialog_notice.notice)
            .event(nwg::Event::OnNotice)
            .handler(AppWindow::read_dialog_output)
            .build(&mut data.events)?;

        create_layout(data)?;

        Ok(())
    }
}

fn create_fonts(data: &mut AppWindowUi) -> Result<(), nwg::NwgError> {
    nwg::Font::builder()
        .size(14)
        //.weight(1000)
        .build(&mut data.small_font)?;

    Ok(())
}

fn create_window(data: &mut AppWindowUi) -> Result<(), nwg::NwgError> {
    nwg::Window::builder()
        .size((640, 480))
        .center(true)
        .title("WiltonDB Configuration Tool")
        .build(&mut data.window)?;
    events::builder()
        .control(&data.window)
        .event(nwg::Event::OnWindowClose)
        .handler(AppWindow::exit)
        .build(&mut data.events)?;

    Ok(())
}

fn create_menu(data: &mut AppWindowUi) -> Result<(), nwg::NwgError> {
    nwg::Menu::builder()
        .parent(&data.window)
        .text("File")
        .build(&mut data.file_menu)?;
    nwg::MenuItem::builder()
        .parent(&data.file_menu)
        .text("Connect to DB")
        .build(&mut data.file_connect_menu_item)?;
    events::builder()
        .control(&data.file_connect_menu_item)
        .event(nwg::Event::OnMenuItemSelected)
        .handler(AppWindow::open_connect_dialog)
        .build(&mut data.events)?;
    nwg::MenuItem::builder()
        .parent(&data.file_menu)
        .text("Exit")
        .build(&mut data.file_exit_menu_item)?;
    events::builder()
        .control(&data.file_exit_menu_item)
        .event(nwg::Event::OnMenuItemSelected)
        .handler(AppWindow::exit)
        .build(&mut data.events)?;

    nwg::Menu::builder()
        .parent(&data.window)
        .text("Help")
        .build(&mut data.help_menu)?;
    nwg::MenuItem::builder()
        .parent(&data.help_menu)
        .text("About")
        .build(&mut data.help_about_menu_item)?;
    nwg::MenuItem::builder()
        .parent(&data.help_menu)
        .text("Website")
        .build(&mut data.help_website_menu_item)?;

    Ok(())
}

pub fn create_layout(data: &mut AppWindowUi) -> Result<(), nwg::NwgError> {

    nwg::FlexboxLayout::builder()
        .parent(&data.window)
        .flex_direction(FlexDirection::Row)
        .child(&data.button1)
        .child_size(Size { width: PT_50, height: PT_50 })
        .child(&data.button2)
        .child_size(Size { width: PT_50, height: PT_50 })
        .child_flex_grow(1.0)
        .build_partial(&data.row1_layout)?;

    nwg::FlexboxLayout::builder()
        .parent(&data.window)
        .flex_direction(FlexDirection::Row)
        .child(&data.data_view)
        .child_flex_grow(1.0)
        .build_partial(&data.row2_layout)?;

    nwg::FlexboxLayout::builder()
        .parent(&data.window)
        .flex_direction(FlexDirection::Row)
        .child(&data.button5)
        .child_size(Size { width: PT_50, height: PT_50 })
        .child(&data.button6)
        .child_size(Size { width: PT_50, height: PT_50 })
        .child_flex_grow(1.0)
        .build_partial(&data.row3_layout)?;

    nwg::FlexboxLayout::builder()
        .parent(&data.window)
        .flex_direction(FlexDirection::Column)
        .child_layout(&data.row1_layout)
        .child_layout(&data.row2_layout)
        .child_flex_grow(1.0)
        .child_layout(&data.row3_layout)
        .build(&data.root_layout)?;

    Ok(())
}
