pub mod date_getter {
    use chrono::Local;

    pub fn get_date() -> () {
        Local::now();
    }
}
