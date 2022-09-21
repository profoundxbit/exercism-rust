use std::ops::Rem;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    hours: Hours,
    minutes: Minutes,
}

#[derive(Debug, PartialEq, Eq)]
struct Hours(i32);

#[derive(Debug, PartialEq, Eq)]
struct Minutes(i32);

impl std::fmt::Display for Hours {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string_hours = self.0.to_string();

        if string_hours.len().eq(&1) {
            string_hours.insert(0, '0');
        }

        write!(f, "{}", string_hours)
    }
}

impl std::fmt::Display for Minutes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string_minutes = self.0.to_string();

        if string_minutes.len().eq(&1) {
            string_minutes.insert(0, '0');
        }
        write!(f, "{}", string_minutes)
    }
}

impl std::fmt::Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.hours, self.minutes)
    }
}

impl Hours {
    fn new(hours: i32) -> Self {
        let hours = hours.rem(24);
        match hours.is_negative() {
            true => Self(24 - hours.abs()),
            false => Self(hours),
        }
    }
}

impl Minutes {
    fn new(minutes: i32) -> Self {
        Self(minutes.rem(60))
        /*  match minutes {
            valid_minute @ 0..=59 => Self(valid_minute),
            60 => Self(0),
            overflow @ 60.. => Self(overflow.rem(60)),
            _ => unreachable!(),
        } */
    }
}

impl From<i32> for Hours {
    fn from(hours: i32) -> Self {
        Self::new(hours)
    }
}

impl From<i32> for Minutes {
    fn from(minutes: i32) -> Self {
        Self::new(minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut hours = hours;
        match minutes.is_negative() {
            true => {
                // First find out how many hours to go back
                hours += minutes / 60;
                if minutes.rem(60).abs() > 0 {
                    hours -= 1; // Negative minutes means atleast 1 hour back
                }

                // Then calculate minutes left after subtracting hours back
                let minutes = 60 + minutes.rem(60);
                Clock {
                    hours: hours.into(),
                    minutes: minutes.into(),
                }
            }
            false => {
                hours += minutes / 60;
                Clock {
                    hours: hours.into(),
                    minutes: minutes.into(),
                }
            }
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let Minutes(mut m) = self.minutes;
        m += minutes;
        let Hours(hours) = self.hours;
        Clock::new(hours, m)
    }
}
