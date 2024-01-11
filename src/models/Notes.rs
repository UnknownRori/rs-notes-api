use chrono::prelude::*;

pub struct Notes {
    id: String,
    title: String,
    description: String,
    tags: Vec<String>,
    created_at: DateTime<Utc>,
}

impl Notes {}
