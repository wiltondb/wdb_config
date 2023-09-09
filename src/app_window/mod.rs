
mod controls;
mod events;
mod layout;
mod nui;
mod setting_groups;
mod window;

use std::collections::HashSet;

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
use load_settings_dialog::SettingRecord;
use load_settings_dialog::LoadSettingsDialogResult;

pub(self) use controls::AppWindowControls;
pub(self) use events::AppWindowEvents;
use layout::AppWindowLayout;
pub use window::AppWindow;
