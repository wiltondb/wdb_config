
mod args;
mod controls;
mod dialog;
mod events;
mod layout;
mod nui;
mod result;

use std::thread;

use nwg::NativeUi;

use crate::*;
use nwg_ui as ui;
use ui::Controls;
use ui::Events;
use ui::Layout;
use ui::PopupArgs;
use ui::PopupDialog;
use connect_check_dialog::ConnectCheckDialog;
use connect_check_dialog::ConnectCheckDialogArgs;
use connect_check_dialog::ConnectCheckDialogResult;

pub use args::ConnectDialogArgs;
use common::PgConnConfig;
pub(self) use controls::ConnectDialogControls;
pub use dialog::ConnectDialog;
use events::ConnectDialogEvents;
use layout::ConnectDialogLayout;
pub use result::ConnectDialogResult;
