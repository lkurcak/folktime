use super::{
    DAY, Duration, GIGA_YEAR, HOUR, KILO_YEAR, MEGA_YEAR, MINUTE, MONTH, MS, US, Unit, WEEK, YEAR,
};

impl Duration {
    pub(crate) fn fmt_two_units_whole(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let secs = self.duration.as_secs();
        let ns = self.duration.subsec_nanos();
        let min = self.min_unit;
        let us_label = self.microsecond_label();

        if secs < 1 && min <= Unit::Millisecond {
            if ns < US && min <= Unit::Nanosecond {
                if ns == 0 {
                    write!(f, "0s 0ms")
                } else {
                    write!(f, "{ns}ns")
                }
            } else if ns < MS && min <= Unit::Microsecond {
                let us = ns / US;
                let ns = ns % US;
                write!(f, "{us}{us_label} {ns}ns")
            } else {
                let ms = ns / MS;
                let us = (ns % MS) / US;
                write!(f, "{ms}ms {us}{us_label}")
            }
        } else if secs < MINUTE && min <= Unit::Second {
            let ms = ns / 1_000_000;
            write!(f, "{secs}s {ms}ms")
        } else if secs < HOUR && min <= Unit::Minute {
            let mins = secs / MINUTE;
            let secs = secs % MINUTE;
            write!(f, "{mins}m {secs}s")
        } else if secs < DAY && min <= Unit::Hour {
            let hours = secs / HOUR;
            let mins = (secs % HOUR) / MINUTE;
            write!(f, "{hours}h {mins}m")
        } else if secs < WEEK && min <= Unit::Day {
            let days = secs / DAY;
            let hours = (secs % DAY) / HOUR;
            write!(f, "{days}d {hours}h")
        } else if secs < MONTH && min <= Unit::Week {
            let weeks = secs / WEEK;
            let days = (secs % WEEK) / DAY;
            write!(f, "{weeks}w {days}d")
        } else if secs < YEAR && min <= Unit::Month {
            let months = secs / MONTH;
            let days = (secs % MONTH) / DAY;
            write!(f, "{months}mo {days}d")
        } else if secs < KILO_YEAR && min <= Unit::Year {
            let years = secs / YEAR;
            let months = (secs % YEAR) / MONTH;
            write!(f, "{years}y {months}mo")
        } else if secs < MEGA_YEAR && min <= Unit::KiloYear {
            let kilo_years = secs / KILO_YEAR;
            let years = (secs % KILO_YEAR) / YEAR;
            write!(f, "{kilo_years}ky {years}y")
        } else if secs < GIGA_YEAR && min <= Unit::MegaYear {
            let mega_years = secs / MEGA_YEAR;
            let kilo_years = (secs % MEGA_YEAR) / KILO_YEAR;
            write!(f, "{mega_years}My {kilo_years}ky")
        } else {
            let giga_years = secs / GIGA_YEAR;
            let mega_years = (secs % GIGA_YEAR) / MEGA_YEAR;
            write!(f, "{giga_years}Gy {mega_years}My")
        }
    }
}
