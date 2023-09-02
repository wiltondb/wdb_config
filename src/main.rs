#![windows_subsystem = "windows"]

mod about_dialog;
mod about_dialog_ui;
mod app_window;
mod app_window_ui;
mod connect_check_dialog;
mod connect_check_dialog_ui;
mod connect_dialog;
mod connect_dialog_ui;

mod dialogs;
mod events;
mod notice;
mod ui;

use nwg::NativeUi;


fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let data = app_window::AppWindow::new();
    let _app = app_window::AppWindow::build_ui(data).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}