
mod args;
mod controls;
mod dialog;
mod events;
mod layout;
mod nui;
mod result;

use std::thread;
use std::time::Duration;
use std::time::Instant;

use clipboard_win::formats;
use clipboard_win::set_clipboard;
use nwg::NativeUi;

use crate::*;
use nwg_ui as ui;
use ui::Controls;
use ui::Events;
use ui::Layout;
use ui::PopupDialog;

pub use args::ConnectCheckDialogArgs;
use common::PgConnConfig;
use common::PgConnError;
pub(self) use controls::ConnectCheckDialogControls;
pub use dialog::ConnectCheckDialog;
use events::ConnectCheckDialogEvents;
use layout::ConnectCheckDialogLayout;
pub use result::ConnectCheckDialogResult;
