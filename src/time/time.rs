use super::timezone::TimeZone;
use std::cmp::Ordering;
use std::time::{Duration, SystemTime};

const MAX_SECONDS: u64 = 315569520000;

pub(super) const UNIX_EPOCH: Time = Time {
    sec: 62167132800,
    nano: 0,
};

const SECS_PER_MINUTE: u64 = 60;
const SECS_PER_HOUR: u64 = 60 * SECS_PER_MINUTE;
const SECS_PER_DAY: u64 = 24 * SECS_PER_HOUR;
const DAYS_PER_400_YEARS: u32 = 365 * 400 + 97;
const DAYS_PER_100_YEARS: u32 = 365 * 100 + 24;
const DAYS_PER_4_YEARS: u32 = 365 * 4 + 1;
const DAYS_BEFORE: [u32; 13] = [
    0,
    31,
    31 + 28,
    31 + 28 + 31,
    31 + 28 + 31 + 30,
    31 + 28 + 31 + 30 + 31,
    31 + 28 + 31 + 30 + 31 + 30,
    31 + 28 + 31 + 30 + 31 + 30 + 31,
    31 + 28 + 31 + 30 + 31 + 30 + 31 + 31,
    31 + 28 + 31 + 30 + 31 + 30 + 31 + 31 + 30,
    31 + 28 + 31 + 30 + 31 + 30 + 31 + 31 + 30 + 31,
    31 + 28 + 31 + 30 + 31 + 30 + 31 + 31 + 30 + 31 + 30,
    31 + 28 + 31 + 30 + 31 + 30 + 31 + 31 + 30 + 31 + 30 + 31,
];

/// Represents a time in range [0000-01-01T00:00:00Z, 10000-01-01T00:00:00Z)
#[derive(Debug, Eq, PartialEq)]
pub struct Time {
    sec: u64,
    nano: u32,
}

impl Time {
    /// Represents `1970-01-01 00:00:00Z`
    pub const UNIX_EPOCH: Time = Time {
        sec: 62167132800,
        nano: 0,
    };

    /// Returns a Time with the given time tuple
    pub fn from_timetuple(
        year: u32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
        nano: u32,
        timezone: TimeZone,
    ) -> Option<Time> {
        if !in_range(year, 0, 10000)
            || !in_range(month, 1, 12)
            || !in_range(day, 1, 31)
            || !in_range(hour, 0, 23)
            || !in_range(minute, 0, 59)
            || !in_range(second, 0, 59)
            || !in_range(nano, 0, 1_000_000_000 - 1)
        {
            return None;
        }

        let is_leap = is_leap_year(year);

        if !is_day_validate(is_leap, month, day) {
            return None;
        }

        let mut d: u32 = 0;

        let mut y = year;

        let mut n: u32 = y / 400;
        y -= 400 * n;
        d += DAYS_PER_400_YEARS * n;

        n = y / 100;
        y -= n * 100;
        d += DAYS_PER_100_YEARS * n;

        n = y / 4;
        y -= n * 4;
        d += DAYS_PER_4_YEARS * n;

        n = y;
        d += 365 * n;

        d += DAYS_BEFORE[(month - 1) as usize];
        // already calculated in DAYS_PER_XX_YEARS
        if is_leap && month <= 2 {
            d -= 1;
        }

        d += day - 1;

        let mut sec: u64 = d as u64 * SECS_PER_DAY
            + hour as u64 * SECS_PER_HOUR
            + minute as u64 * SECS_PER_MINUTE
            + second as u64;

        let offset = timezone.offset();
        if offset >= 0 {
            let minus = offset as u64;
            if minus > sec {
                return None;
            }

            sec -= minus;
        } else {
            sec += (-offset) as u64;
        }

        if sec >= MAX_SECONDS {
            return None;
        }

        Some(Time {
            sec: sec,
            nano: nano,
        })
    }

    /// Convert the time to SystemTime, returns None if the time is before unix epoch
    pub fn to_system_time(&self) -> Option<SystemTime> {
        if let Some(d) = self.since(&UNIX_EPOCH) {
            return Some(SystemTime::UNIX_EPOCH + d);
        }

        None
    }

    /// Returns the duration since an earlier time, and None if earlier is not before self.
    pub fn since(&self, earlier: &Time) -> Option<Duration> {
        if self < earlier {
            return None;
        }

        let mut sec = self.sec - earlier.sec;
        let mut nano = self.nano;
        if nano < earlier.nano {
            sec -= 1;
            nano += 1_000_000_000;
        }
        nano -= earlier.nano;

        Some(Duration::new(sec, nano))
    }
}

fn is_leap_year(y: u32) -> bool {
    return y % 4 == 0 && (y % 100 != 0 || y % 400 == 0);
}

fn in_range(n: u32, min: u32, max: u32) -> bool {
    return min <= n && n <= max;
}

fn is_day_validate(is_leap: bool, m: u32, d: u32) -> bool {
    match m {
        2 if is_leap => d <= 29,
        2 => d <= 28,
        4 | 6 | 9 | 11 => d <= 30,
        _ => d <= 31,
    }
}

impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Time) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Time {
    fn cmp(&self, other: &Time) -> Ordering {
        let ord = self.sec.cmp(&other.sec);
        match ord {
            Ordering::Equal => self.nano.cmp(&other.nano),
            _ => ord,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TimeZone;
    use super::{is_leap_year, Time, UNIX_EPOCH};
    use std::time::{Duration, SystemTime};

    #[test]
    fn test_timetuple() {
        struct Case {
            tuple: (u32, u32, u32, u32, u32, u32, u32, TimeZone),
            expect: Option<Duration>,
        }

        let cases = vec![
            Case {
                tuple: (2018, 1, 1, 0, 0, 0, 0, TimeZone::new(8).unwrap()),
                expect: Some(Duration::new(1514736000, 0)),
            },
            Case {
                tuple: (2018, 2, 1, 0, 0, 0, 0, TimeZone::new(8).unwrap()),
                expect: Some(Duration::new(1517414400, 0)),
            },
            Case {
                tuple: (2018, 3, 1, 0, 0, 0, 0, TimeZone::new(8).unwrap()),
                expect: Some(Duration::new(1519833600, 0)),
            },
            Case {
                tuple: (2018, 4, 1, 0, 0, 0, 0, TimeZone::new(8).unwrap()),
                expect: Some(Duration::new(1522512000, 0)),
            },
            Case {
                tuple: (2018, 5, 1, 0, 0, 0, 0, TimeZone::new(8).unwrap()),
                expect: Some(Duration::new(1525104000, 0)),
            },
            Case {
                tuple: (2018, 6, 1, 0, 0, 0, 0, TimeZone::new(8).unwrap()),
                expect: Some(Duration::new(1527782400, 0)),
            },
            Case {
                tuple: (2018, 7, 1, 0, 0, 0, 0, TimeZone::new(8).unwrap()),
                expect: Some(Duration::new(1530374400, 0)),
            },
            Case {
                tuple: (2018, 8, 1, 0, 0, 0, 0, TimeZone::new(8).unwrap()),
                expect: Some(Duration::new(1533052800, 0)),
            },
            Case {
                tuple: (2018, 9, 1, 0, 0, 0, 0, TimeZone::new(8).unwrap()),
                expect: Some(Duration::new(1535731200, 0)),
            },
            Case {
                tuple: (2018, 10, 1, 0, 0, 0, 0, TimeZone::new(8).unwrap()),
                expect: Some(Duration::new(1538323200, 0)),
            },
            Case {
                tuple: (2018, 11, 1, 0, 0, 0, 0, TimeZone::new(8).unwrap()),
                expect: Some(Duration::new(1541001600, 0)),
            },
            Case {
                tuple: (2018, 12, 1, 0, 0, 0, 0, TimeZone::new(8).unwrap()),
                expect: Some(Duration::new(1543593600, 0)),
            },
            Case {
                tuple: (
                    2018,
                    9,
                    21,
                    16,
                    56,
                    44,
                    234867232,
                    TimeZone::new(8).unwrap(),
                ),
                expect: Some(Duration::new(1537520204, 234867232)),
            },
            Case {
                tuple: (2000, 2, 1, 0, 0, 0, 0, TimeZone::new(8).unwrap()),
                expect: Some(Duration::new(949334400, 0)),
            },
            Case {
                tuple: (2000, 3, 1, 0, 0, 0, 0, TimeZone::new(8).unwrap()),
                expect: Some(Duration::new(951840000, 0)),
            },
            Case {
                tuple: (2100, 2, 1, 0, 0, 0, 0, TimeZone::new(8).unwrap()),
                expect: Some(Duration::new(4105094400, 0)),
            },
            Case {
                tuple: (2100, 3, 1, 0, 0, 0, 0, TimeZone::new(8).unwrap()),
                expect: Some(Duration::new(4107513600, 0)),
            },
            Case {
                tuple: (2104, 2, 1, 0, 0, 0, 0, TimeZone::new(8).unwrap()),
                expect: Some(Duration::new(4231238400, 0)),
            },
            Case {
                tuple: (2104, 3, 1, 0, 0, 0, 0, TimeZone::new(8).unwrap()),
                expect: Some(Duration::new(4233744000, 0)),
            },
            Case {
                tuple: (2105, 2, 1, 0, 0, 0, 0, TimeZone::new(8).unwrap()),
                expect: Some(Duration::new(4262860800, 0)),
            },
            Case {
                tuple: (2105, 3, 1, 0, 0, 0, 0, TimeZone::new(8).unwrap()),
                expect: Some(Duration::new(4265280000, 0)),
            },
        ];

        for c in cases {
            let d = Time::from_timetuple(
                c.tuple.0, c.tuple.1, c.tuple.2, c.tuple.3, c.tuple.4, c.tuple.5, c.tuple.6,
                c.tuple.7,
            ).unwrap();

            assert_eq!(d.since(&UNIX_EPOCH), c.expect);
        }
    }

    #[test]
    fn test_1970_to_9999() {
        let mut sec: u64 = 0; // +08:00

        for y in 1970..10000 {
            let year = y as u32;
            if year > 1970 {
                sec += 3600 * 24 * 365;
                if is_leap_year(year - 1) {
                    sec += 3600 * 24;
                }
            }

            assert!(
                Time::from_timetuple(year, 1, 1, 0, 0, 0, 0, TimeZone::new(0).unwrap())
                    .unwrap()
                    .to_system_time()
                    == Some(SystemTime::UNIX_EPOCH + Duration::new(sec.clone(), 0)),
                "{}-01-01",
                year,
            );

            let feb_1st_sec = sec + 3600 * 24 * 31;
            assert!(
                Time::from_timetuple(year, 2, 1, 0, 0, 0, 0, TimeZone::new(0).unwrap())
                    .unwrap()
                    .to_system_time()
                    == Some(SystemTime::UNIX_EPOCH + Duration::new(feb_1st_sec, 0)),
                "{}-02-01",
                year,
            );

            let mut march_1st_sec = sec + 3600 * 24 * (31 + 28);
            if is_leap_year(year) {
                march_1st_sec += 3600 * 24;
            }

            assert!(
                Time::from_timetuple(year, 3, 1, 0, 0, 0, 0, TimeZone::new(0).unwrap())
                    .unwrap()
                    .to_system_time()
                    == Some(SystemTime::UNIX_EPOCH + Duration::new(march_1st_sec, 0)),
                "{}-03-01",
                year,
            );
        }
    }
}
