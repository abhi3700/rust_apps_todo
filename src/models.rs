use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub created_at: u32,
    pub title: String,
    pub assignee: Option<String>,
}
