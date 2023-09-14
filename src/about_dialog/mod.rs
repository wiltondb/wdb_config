
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
use common::labels;

pub use args::AboutDialogArgs;
pub(self) use controls::AboutDialogControls;
pub use dialog::AboutDialog;
use events::AboutDialogEvents;
use layout::AboutDialogLayout;
