pub mod date_getter {
    use chrono::{DateTime, Utc};

    pub fn get_date() -> DateTime<Utc> {
        Utc::now()
    }
}
