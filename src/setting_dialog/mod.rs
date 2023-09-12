
mod args;
mod controls;
mod dialog;
mod events;
mod layout;
mod nui;

use std::thread;

use nwg::NativeUi;

use crate::*;
use nwg_ui as ui;
use ui::Controls;
use ui::Events;
use ui::Layout;
use ui::PopupArgs;
use ui::PopupDialog;
use ui::PopupJoinHandle;

use common::PgConnConfig;
use common::SettingRecord;
use setting_change_dialog::SettingChangeDialog;
use setting_change_dialog::SettingChangeDialogArgs;
use setting_change_dialog::SettingChangeDialogResult;

pub(super) use args::SettingDialogArgs;
pub(super) use dialog::SettingDialog;
use controls::SettingDialogControls;
use events::SettingDialogEvents;
use layout::SettingDialogLayout;
