
mod args;
mod controls;
mod dialog;
mod error;
mod events;
mod layout;
mod nui;
mod result;

use std::cell::RefCell;
use std::fmt;
use std::ops::Deref;
use std::rc::Rc;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use std::time::Instant;

use clipboard_win::formats;
use clipboard_win::set_clipboard;
use native_tls::TlsConnector;
use nwg::NativeUi;
use postgres::config::Config;
use postgres::NoTls;
use postgres_native_tls::MakeTlsConnector;

use crate::*;
use nwg_ui as ui;
use ui::Controls;
use ui::Events;
use ui::Layout;
use ui::PopupDialog;
use connect_dialog::ConnectConfig;

pub use args::LoadSettingsDialogArgs;
pub(self) use controls::LoadSettingsDialogControls;
pub use dialog::LoadSettingsDialog;
use events::LoadSettingsDialogEvents;
use error::LoadSettingsDialogError;
use layout::LoadSettingsDialogLayout;
pub use result::SettingRecord;
pub use result::LoadSettingsDialogResult;
use crate::nwg_ui::PopupArgs;
