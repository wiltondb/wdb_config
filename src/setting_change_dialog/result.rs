
#[derive(Default)]
pub(super) struct ChangeResult {
    pub(super) success: bool,
    pub(super) effective_value: String,
    pub(super) restart_pending: bool,
    pub(super) error: String,
}

impl ChangeResult {
    pub(super) fn success(effective_value: String, restart_pending: bool) -> Self {
        Self {
            success: true,
            effective_value,
            restart_pending,
            error: String::new()
        }
    }

    pub(super) fn failure(error: String) -> Self {
        Self {
            success: false,
            effective_value: String::new(),
            restart_pending: false,
            error
        }
    }
}

#[derive(Default, Clone)]
pub struct SettingChangeDialogResult {
    pub success: bool,
    pub effective_value: String,
}

impl SettingChangeDialogResult {
    pub(super) fn success(effective_value: String) -> Self {
        Self {
            success: true,
            effective_value
        }
    }

    pub(super) fn failure() -> Self {
        Self {
            success: false,
            effective_value: String::new()
        }
    }
}
