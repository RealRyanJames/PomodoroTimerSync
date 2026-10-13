use chrono::{DateTime, Utc};

pub struct Now<Utc> {
    pub now: Utc,
    pub hour: Utc,
    pub minutes: Utc,
    pub days: Utc,
    pub months: Utc,
    pub year: Utc,
}

trait NowData {
    fn get_current(&mut self) -> Self;
}

impl NowData for Now<DateTime<Utc>> {
    fn get_current(&mut self) -> Self {
        let now: Now<DateTime<Utc>> = Self {
            now: self.now,
            hour: self.hour,
            minutes: self.minutes,
            days: self.days,
            months: self.months,
            year: self.year,
        };

        now
    }
}
