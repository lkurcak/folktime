use super::{
    DAY, Duration, GIGA_YEAR, HOUR, KILO_YEAR, MEGA_YEAR, MINUTE, MONTH, MS, US, Unit, WEEK, YEAR,
};

macro_rules! fmt_three {
    ($big:ty, $small:ty, $i:ident) => {
        fn $i(
            big: $big,
            small: $small,
            unit: &str,
            f: &mut core::fmt::Formatter,
        ) -> core::fmt::Result {
            if big < 10 {
                write!(f, "{}.{:02}{unit}", big, small / 10)
            } else if big < 100 {
                write!(f, "{}.{:01}{unit}", big, small / 100)
            } else {
                write!(f, "{}{unit}", big)
            }
        }
    };
}

fmt_three!(u32, u32, fmt_three_u32);
fmt_three!(u64, u32, fmt_three_u64_u32);

fn fmt_100(val: u64, div: u64, unit: &str, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let a = (val * 100) / div;
    let big = a / 100;
    let small = a % 100;
    if big < 10 {
        write!(f, "{big}.{small:02}{unit}")
    } else if big < 100 {
        write!(f, "{}.{:01}{}", big, small / 10, unit)
    } else {
        write!(f, "{big}{unit}")
    }
}

impl Duration {
    pub(crate) fn fmt_one_unit_frac(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let secs = self.duration.as_secs();
        let ns = self.duration.subsec_nanos();
        let min = self.min_unit;

        if secs < 1 && min <= Unit::Millisecond {
            if ns < US && min <= Unit::Nanosecond {
                if ns == 0 {
                    write!(f, "0.00s")
                } else {
                    write!(f, "{ns}ns")
                }
            } else if ns < MS && min <= Unit::Microsecond {
                let us = ns / US;
                let ns = ns % US;
                fmt_three_u32(us, ns, "us", f)
            } else {
                let ms = ns / MS;
                let us = (ns % MS) / US;
                fmt_three_u32(ms, us, "ms", f)
            }
        } else if secs < MINUTE && min <= Unit::Second {
            let ms = ns / 1_000_000;
            fmt_three_u64_u32(secs, ms, "s", f)
        } else if secs < HOUR && min <= Unit::Minute {
            if secs < 10 * MINUTE {
                let hundredths = ns / 10_000_000;
                let val = secs * 100 + u64::from(hundredths);
                write!(f, "{}.{:02}m", val / 6000, (val / 60) % 100)
            } else {
                fmt_100(secs, MINUTE, "m", f)
            }
        } else if secs < DAY && min <= Unit::Hour {
            fmt_100(secs, HOUR, "h", f)
        } else if secs < WEEK && min <= Unit::Day {
            fmt_100(secs, DAY, "d", f)
        } else if secs < MONTH && min <= Unit::Week {
            fmt_100(secs, WEEK, "w", f)
        } else if secs < YEAR && min <= Unit::Month {
            fmt_100(secs, MONTH, "mo", f)
        } else if secs < KILO_YEAR && min <= Unit::Year {
            fmt_100(secs, YEAR, "y", f)
        } else if secs < MEGA_YEAR && min <= Unit::KiloYear {
            fmt_100(secs, KILO_YEAR, "ky", f)
        } else if secs < GIGA_YEAR && min <= Unit::MegaYear {
            // Right-shift both operands to prevent overflow in `val * 100` inside fmt_100.
            let shift = 1;
            fmt_100(secs >> shift, MEGA_YEAR >> shift, "My", f)
        } else {
            // Right-shift both operands to prevent overflow in `val * 100` inside fmt_100.
            // A larger shift is needed here because GIGA_YEAR * 100 exceeds u64::MAX.
            let shift = 8;
            fmt_100(secs >> shift, GIGA_YEAR >> shift, "Gy", f)
        }
    }
}
