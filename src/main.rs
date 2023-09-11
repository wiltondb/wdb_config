#![windows_subsystem = "windows"]

mod nwg_ui;
mod common;
mod about_dialog;
mod app_window;
mod connect_dialog;
mod connect_check_dialog;
mod load_settings_dialog;
mod setting_dialog;

use nwg::NativeUi;

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let data = app_window::AppWindow::new();
    let _app = app_window::AppWindow::build_ui(data).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}