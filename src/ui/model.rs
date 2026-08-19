use chrono::{DateTime, Utc};

pub struct Model {
    pub value: DateTime<Utc>,
    pub running: bool,
    pub selection: Option<DateTime<Utc>>,
}
