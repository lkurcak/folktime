pub mod one_unit_frac;
pub mod one_unit_whole;
pub mod two_units_whole;

use core::fmt::Display;

const MIN: u64 = 60;
const HOUR: u64 = 60 * MIN;
const DAY: u64 = 24 * HOUR;
const WEEK: u64 = 7 * DAY;
const MONTH: u64 = 2_629_846; // = ceil(365.256363004*24*60*60/12)
const YEAR: u64 = 31_558_150; // = ceil(365.256363004*24*60*60)
const KILO_YEAR: u64 = 1_000 * YEAR;
const MEGA_YEAR: u64 = 1_000 * KILO_YEAR;
const GIGA_YEAR: u64 = 1_000 * MEGA_YEAR;

const US: u32 = 1_000;
const MS: u32 = 1_000 * US;

/// The unit of time to use as the minimum display unit.
///
/// When set via [`Duration::with_min_unit`], any duration below 1 of this unit
/// will be displayed as `"0"` followed by the unit label (e.g. `"0s"`, `"0m"`).
///
/// # Example
/// ```
/// use std::time::Duration;
/// use folktime::Folktime;
/// use folktime::duration::{Style, Unit};
///
/// let d = Folktime::duration(Duration::from_nanos(500))
///     .with_style(Style::OneUnitWhole)
///     .with_min_unit(Unit::Second);
/// assert_eq!(format!("{}", d), "0s");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Unit {
    Nanosecond,
    Microsecond,
    Millisecond,
    Second,
    Minute,
    Hour,
    Day,
    Week,
    Month,
    Year,
    KiloYear,
    MegaYear,
    GigaYear,
}

impl Unit {
    /// Returns the display label for this unit.
    pub fn label(&self) -> &'static str {
        match self {
            Unit::Nanosecond => "ns",
            Unit::Microsecond => "us",
            Unit::Millisecond => "ms",
            Unit::Second => "s",
            Unit::Minute => "m",
            Unit::Hour => "h",
            Unit::Day => "d",
            Unit::Week => "w",
            Unit::Month => "mo",
            Unit::Year => "y",
            Unit::KiloYear => "ky",
            Unit::MegaYear => "My",
            Unit::GigaYear => "Gy",
        }
    }

    /// Returns true if the given duration is below 1 of this unit.
    pub fn is_below(&self, secs: u64, ns: u32) -> bool {
        match self {
            Unit::Nanosecond => false,
            Unit::Microsecond => secs == 0 && ns < US,
            Unit::Millisecond => secs == 0 && ns < MS,
            Unit::Second => secs < 1,
            Unit::Minute => secs < MIN,
            Unit::Hour => secs < HOUR,
            Unit::Day => secs < DAY,
            Unit::Week => secs < WEEK,
            Unit::Month => secs < MONTH,
            Unit::Year => secs < YEAR,
            Unit::KiloYear => secs < KILO_YEAR,
            Unit::MegaYear => secs < MEGA_YEAR,
            Unit::GigaYear => secs < GIGA_YEAR,
        }
    }
}

/// Formatting style for [core::time::Duration].
pub enum Style {
    /// Format the duration in the largest possible unit with a fractional part with 3 significant digits.
    ///
    /// # Example
    /// ```
    /// use std::time::Duration;
    /// use folktime::Folktime;
    /// use folktime::duration::Style;
    ///
    /// let d = Folktime::duration(Duration::from_secs(123)).with_style(Style::OneUnitFrac);
    /// assert_eq!(format!("{}", d), "2.05m");
    OneUnitFrac,
    /// Format the duration in the largest possible unit with a whole number.
    ///
    /// # Example
    /// ```
    /// use std::time::Duration;
    /// use folktime::Folktime;
    /// use folktime::duration::Style;
    ///
    /// let d = Folktime::duration(Duration::from_secs(123)).with_style(Style::OneUnitWhole);
    /// assert_eq!(format!("{}", d), "2m");
    OneUnitWhole,
    /// Format the duration in the two largest possible units with whole numbers.
    ///
    /// # Example
    /// ```
    /// use std::time::Duration;
    /// use folktime::Folktime;
    /// use folktime::duration::Style;
    ///
    /// let d = Folktime::duration(Duration::from_secs(123)).with_style(Style::TwoUnitsWhole);
    /// assert_eq!(format!("{}", d), "2m 3s");
    /// ```
    TwoUnitsWhole,
}

pub struct Duration {
    pub field1: core::time::Duration,
    pub field2: Style,
    pub field3: Unit,
}

impl Duration {
    pub const fn new(d: core::time::Duration) -> Self {
        Self {
            field1: d,
            field2: Style::OneUnitFrac,
            field3: Unit::Nanosecond,
        }
    }

    /// Set the formatting style.
    ///
    /// # Example
    /// ```
    /// use std::time::Duration;
    /// use folktime::Folktime;
    /// use folktime::duration::Style;
    ///
    /// let d = Folktime::duration(Duration::from_secs(123)).with_style(Style::TwoUnitsWhole);
    /// assert_eq!(format!("{}", d), "2m 3s");
    /// ```
    pub fn with_style(self, style: Style) -> Self {
        Self {
            field1: self.field1,
            field2: style,
            field3: self.field3,
        }
    }

    /// Set the minimum unit to display.
    ///
    /// Any duration below 1 of this unit will be displayed as `"0"` followed by
    /// the unit label. This uses truncation (floor) consistently: for example,
    /// 999ms with `min_unit = Second` displays as `"0s"`.
    ///
    /// # Example
    /// ```
    /// use std::time::Duration;
    /// use folktime::Folktime;
    /// use folktime::duration::{Style, Unit};
    ///
    /// let d = Folktime::duration(Duration::from_millis(500))
    ///     .with_style(Style::OneUnitWhole)
    ///     .with_min_unit(Unit::Second);
    /// assert_eq!(format!("{}", d), "0s");
    ///
    /// let d = Folktime::duration(Duration::from_secs(2))
    ///     .with_style(Style::OneUnitWhole)
    ///     .with_min_unit(Unit::Second);
    /// assert_eq!(format!("{}", d), "2s");
    /// ```
    pub fn with_min_unit(self, unit: Unit) -> Self {
        Self {
            field1: self.field1,
            field2: self.field2,
            field3: unit,
        }
    }
}

impl Display for Duration {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.field2 {
            Style::OneUnitFrac => self.fmt_one_unit_frac(f),
            Style::OneUnitWhole => self.fmt_one_unit_whole(f),
            Style::TwoUnitsWhole => self.fmt_two_units_whole(f),
        }
    }
}

