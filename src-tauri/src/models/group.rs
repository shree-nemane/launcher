use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", default)]
pub struct ApplicationGroup {
    pub id: String,
    pub name: String,
    pub applications: Vec<String>,
    pub execution_order: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl Default for ApplicationGroup {
    fn default() -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            id: "".to_string(),
            name: "".to_string(),
            applications: Vec::new(),
            execution_order: Vec::new(),
            created_at: now.clone(),
            updated_at: now,
        }
    }
}
