/*
 * Copyright 2023, WiltonDB Software
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

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
use common::PgConnConfig;
use common::SettingRecord;
use connect_dialog::ConnectDialog;
use connect_dialog::ConnectDialogArgs;
use connect_dialog::ConnectDialogResult;
use load_settings_dialog::LoadSettingsDialog;
use load_settings_dialog::LoadSettingsDialogArgs;
use load_settings_dialog::LoadSettingsDialogResult;
use setting_dialog::SettingDialog;
use setting_dialog::SettingDialogArgs;
use setting_dialog::SettingDialogResult;

pub(self) use controls::AppWindowControls;
pub(self) use events::AppWindowEvents;
use layout::AppWindowLayout;
pub use window::AppWindow;
