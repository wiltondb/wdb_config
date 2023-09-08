
mod args;
mod config;
mod controls;
mod dialog;
mod events;
mod layout;
mod nui;

use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use std::thread;
use std::thread::JoinHandle;

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

pub use config::ConnectConfig;
pub use args::ConnectDialogArgs;
pub(self) use controls::ConnectDialogControls;
pub use dialog::ConnectDialog;
use events::ConnectDialogEvents;
use layout::ConnectDialogLayout;
