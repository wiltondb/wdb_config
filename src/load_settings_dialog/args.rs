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

use super::*;

#[derive(Default)]
pub struct LoadSettingsDialogArgs {
    notice_sender:  ui::SyncNoticeSender,
    pub(super) pg_conn_config: PgConnConfig,
}

impl LoadSettingsDialogArgs {
    pub fn new(notice: &ui::SyncNotice, pg_conn_config: PgConnConfig) -> Self {
        Self {
            notice_sender: notice.sender(),
            pg_conn_config,
        }
    }
}

impl ui::PopupArgs for LoadSettingsDialogArgs {
    fn notify_parent(&self) {
        self.notice_sender.send()
    }
}
