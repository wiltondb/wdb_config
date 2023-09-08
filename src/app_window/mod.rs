
mod controls;
mod events;
mod layout;
mod nui;
mod window;

use std::cell::RefCell;
use std::os::windows::process::CommandExt;
use std::process::Command;
use std::process::Stdio;
use std::rc::Rc;

use crate::*;
use nwg_ui as ui;
use ui::Controls;
use ui::Events;
use ui::Layout;
use ui::PopupDialog;

use about_dialog::AboutDialog;
use about_dialog::AboutDialogArgs;
use connect_dialog::ConnectConfig;
use connect_dialog::ConnectDialog;
use connect_dialog::ConnectDialogArgs;
use load_settings_dialog::LoadSettingsDialog;
use load_settings_dialog::LoadSettingsDialogArgs;
use load_settings_dialog::LoadSettingsDialogResult;

pub(self) use controls::AppWindowControls;
pub(self) use events::AppWindowEvents;
use layout::AppWindowLayout;
pub use window::AppWindow;
