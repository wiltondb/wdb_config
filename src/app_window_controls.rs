
use super::*;
use app_window::AppWindow;

fn create_fonts(data: &mut AppWindow) -> Result<(), nwg::NwgError> {
    nwg::Font::builder()
        .size(14)
        //.weight(1000)
        .build(&mut data.small_font)?;

    Ok(())
}

fn create_window(data: &mut AppWindow) -> Result<(), nwg::NwgError> {
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

fn create_menu(data: &mut AppWindow) -> Result<(), nwg::NwgError> {
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

pub fn build(data: &mut AppWindow) -> Result<(), nwg::NwgError> {
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

    Ok(())
}