
use super::*;

#[derive(Default, Clone)]
pub struct LoadSettingsDialogResult {
    pub records: Vec<SettingRecord>
}

impl LoadSettingsDialogResult {
    pub fn new(records: Vec<SettingRecord>) -> Self {
        Self { records }
    }
}
