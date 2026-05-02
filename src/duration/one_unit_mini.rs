use super::{
    DAY, Duration, GIGA_YEAR, HOUR, KILO_YEAR, MEGA_YEAR, MINUTE, MONTH, MS, US, Unit, WEEK, YEAR,
};

macro_rules! fmt_mini {
    ($big:ty, $small:ty, $i:ident) => {
        fn $i(
            big: $big,
            small: $small,
            unit: &str,
            f: &mut core::fmt::Formatter,
        ) -> core::fmt::Result {
            if big < 10 {
                write!(f, "{}.{:01}{unit}", big, small / 100)
            } else {
                write!(f, "{}{unit}", big)
            }
        }
    };
}

fmt_mini!(u32, u32, fmt_mini_u32);
fmt_mini!(u64, u32, fmt_mini_u64_u32);

fn fmt_10(val: u64, div: u64, unit: &str, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let a = (val * 10) / div;
    let big = a / 10;
    let small = a % 10;
    if big < 10 {
        write!(f, "{big}.{small}{unit}")
    } else {
        write!(f, "{big}{unit}")
    }
}

impl Duration {
    pub(crate) fn fmt_one_unit_mini(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let secs = self.duration.as_secs();
        let ns = self.duration.subsec_nanos();
        let min = self.min_unit;
        let us_label = self.microsecond_label();

        if secs < 1 && min <= Unit::Millisecond {
            if ns < US && min <= Unit::Nanosecond {
                if ns == 0 {
                    write!(f, "0.0s")
                } else {
                    write!(f, "{ns}ns")
                }
            } else if ns < MS && min <= Unit::Microsecond {
                let us = ns / US;
                let ns = ns % US;
                fmt_mini_u32(us, ns, us_label, f)
            } else {
                let ms = ns / MS;
                let us = (ns % MS) / US;
                fmt_mini_u32(ms, us, "ms", f)
            }
        } else if secs < MINUTE && min <= Unit::Second {
            let ms = ns / 1_000_000;
            fmt_mini_u64_u32(secs, ms, "s", f)
        } else if secs < HOUR && min <= Unit::Minute {
            fmt_10(secs, MINUTE, "m", f)
        } else if secs < DAY && min <= Unit::Hour {
            fmt_10(secs, HOUR, "h", f)
        } else if secs < WEEK && min <= Unit::Day {
            fmt_10(secs, DAY, "d", f)
        } else if secs < MONTH && min <= Unit::Week {
            fmt_10(secs, WEEK, "w", f)
        } else if secs < YEAR && min <= Unit::Month {
            fmt_10(secs, MONTH, "mo", f)
        } else if secs < KILO_YEAR && min <= Unit::Year {
            fmt_10(secs, YEAR, "y", f)
        } else if secs < MEGA_YEAR && min <= Unit::KiloYear {
            fmt_10(secs, KILO_YEAR, "ky", f)
        } else if secs < GIGA_YEAR && min <= Unit::MegaYear {
            fmt_10(secs, MEGA_YEAR, "My", f)
        } else {
            let shift = 4;
            fmt_10(secs >> shift, GIGA_YEAR >> shift, "Gy", f)
        }
    }
}
