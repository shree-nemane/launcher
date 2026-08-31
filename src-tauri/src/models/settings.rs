use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", default)]
pub struct Settings {
    pub global_shortcut: String,
    pub default_application_group_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl Default for Settings {
    fn default() -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            global_shortcut: "Alt+Space".to_string(),
            default_application_group_id: None,
            created_at: now.clone(),
            updated_at: now,
        }
    }
}
