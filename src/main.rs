#![windows_subsystem = "windows"]

mod about_dialog;
mod app_window;
mod connect_dialog;
mod connect_check_dialog;
mod load_settings_dialog;
mod nwg_ui;

use nwg::NativeUi;

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let data = app_window::AppWindow::new();
    let app = app_window::AppWindow::build_ui(data).expect("Failed to build UI");
    app.open_connect_dialog();

    nwg::dispatch_thread_events();
}