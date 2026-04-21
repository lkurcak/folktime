use super::{
    Duration, Unit, DAY, GIGA_YEAR, HOUR, KILO_YEAR, MEGA_YEAR, MIN, MONTH, MS, US, WEEK, YEAR,
};

impl Duration {
    pub fn fmt_one_unit_whole(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let secs = self.duration.as_secs();
        let ns = self.duration.subsec_nanos();
        let min = self.min_unit;

        if secs < 1 && min <= Unit::Millisecond {
            if ns < US && min <= Unit::Nanosecond {
                if ns == 0 {
                    write!(f, "0s")
                } else {
                    write!(f, "{ns}ns")
                }
            } else if ns < MS && min <= Unit::Microsecond {
                let us = ns / US;
                write!(f, "{us}us")
            } else {
                let ms = ns / MS;
                write!(f, "{ms}ms")
            }
        } else if secs < MIN && min <= Unit::Second {
            write!(f, "{secs}s")
        } else if secs < HOUR && min <= Unit::Minute {
            let mins = secs / MIN;
            write!(f, "{mins}m")
        } else if secs < DAY && min <= Unit::Hour {
            let hours = secs / HOUR;
            write!(f, "{hours}h")
        } else if secs < WEEK && min <= Unit::Day {
            let days = secs / DAY;
            write!(f, "{days}d")
        } else if secs < MONTH && min <= Unit::Week {
            let weeks = secs / WEEK;
            write!(f, "{weeks}w")
        } else if secs < YEAR && min <= Unit::Month {
            let months = secs / MONTH;
            write!(f, "{months}mo")
        } else if secs < KILO_YEAR && min <= Unit::Year {
            let years = secs / YEAR;
            write!(f, "{years}y")
        } else if secs < MEGA_YEAR && min <= Unit::KiloYear {
            let kilo_years = secs / KILO_YEAR;
            write!(f, "{kilo_years}ky")
        } else if secs < GIGA_YEAR && min <= Unit::MegaYear {
            let mega_years = secs / MEGA_YEAR;
            write!(f, "{mega_years}My")
        } else {
            let giga_years = secs / GIGA_YEAR;
            write!(f, "{giga_years}Gy")
        }
    }
}

