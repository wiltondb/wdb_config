
#[derive(Default, Debug, Clone)]
pub struct SettingRecord {
    pub name: String,
    pub setting: String,
    pub description: String,
}

#[derive(Default)]
pub struct LoadSettingsDialogResult {
    pub records: Vec<SettingRecord>
}

impl LoadSettingsDialogResult {
    pub fn new(records: Vec<SettingRecord>) -> Self {
        Self { records }
    }
}
