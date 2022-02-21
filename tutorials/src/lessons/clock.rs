use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub struct Clock {
    minutes: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.minutes / 60, self.minutes % 60)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let day = 1440;
        return Self{minutes: (((hours * 60 + minutes) % day) + day) % day};
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        return Self::new(0, self.minutes + minutes);
    }
}
