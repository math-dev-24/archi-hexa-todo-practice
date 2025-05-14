use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Todo {
    pub id: u64,
    pub title: String,
    pub completed: bool,
    pub description: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: DateTime<Utc>,
}

impl Todo {
    pub fn new(id: u64, title: String, description: String) -> Todo {
        Todo {
            id,
            title,
            completed: false,
            description,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn str(&self) -> String {
        let icon = if self.completed { "OK" } else { "NOK" };
        icon.to_string() + " - " + &self.title
    }
}