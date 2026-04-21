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
    pub duration: core::time::Duration,
    pub style: Style,
    pub min_unit: Unit,
}

impl Duration {
    pub const fn new(d: core::time::Duration) -> Self {
        Self {
            duration: d,
            style: Style::OneUnitFrac,
            min_unit: Unit::Nanosecond,
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
            duration: self.duration,
            style,
            min_unit: self.min_unit,
        }
    }

    /// Set the minimum unit to display.
    ///
    /// Prevents the formatter from selecting any unit smaller than the
    /// specified one. Values below 1 of this unit are expressed in terms
    /// of that unit rather than a smaller one:
    ///
    /// - `OneUnitWhole`: shows `"0"` + label (e.g. `"0s"`)
    /// - `OneUnitFrac`: shows the fractional value (e.g. `"0.50s"`)
    /// - `TwoUnitsWhole`: shows `"0"` + label with remainder (e.g. `"0s 500ms"`)
    ///
    /// # Example
    /// ```
    /// use std::time::Duration;
    /// use folktime::Folktime;
    /// use folktime::duration::{Style, Unit};
    ///
    /// let d = Folktime::duration(Duration::from_millis(500))
    ///     .with_min_unit(Unit::Second);
    /// assert_eq!(format!("{}", d), "0.50s");
    ///
    /// let d = Folktime::duration(Duration::from_millis(500))
    ///     .with_style(Style::OneUnitWhole)
    ///     .with_min_unit(Unit::Second);
    /// assert_eq!(format!("{}", d), "0s");
    /// ```
    pub fn with_min_unit(self, unit: Unit) -> Self {
        Self {
            duration: self.duration,
            style: self.style,
            min_unit: unit,
        }
    }
}

impl Display for Duration {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.style {
            Style::OneUnitFrac => self.fmt_one_unit_frac(f),
            Style::OneUnitWhole => self.fmt_one_unit_whole(f),
            Style::TwoUnitsWhole => self.fmt_two_units_whole(f),
        }
    }
}

