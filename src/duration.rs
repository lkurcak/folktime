mod one_unit_frac;
mod one_unit_whole;
mod two_units_whole;

use core::fmt::Display;

/// Seconds in one minute.
pub const MIN: u64 = 60;
/// Seconds in one hour.
pub const HOUR: u64 = 60 * MIN;
/// Seconds in one day.
pub const DAY: u64 = 24 * HOUR;
/// Seconds in one week.
pub const WEEK: u64 = 7 * DAY;
/// Seconds in one month (average Gregorian month).
pub const MONTH: u64 = 2_629_846; // = ceil(365.256363004*24*60*60/12)
/// Seconds in one year (average Gregorian year).
pub const YEAR: u64 = 31_558_150; // = ceil(365.256363004*24*60*60)
/// Seconds in one thousand years.
pub const KILO_YEAR: u64 = 1_000 * YEAR;
/// Seconds in one million years.
pub const MEGA_YEAR: u64 = 1_000 * KILO_YEAR;
/// Seconds in one billion years.
pub const GIGA_YEAR: u64 = 1_000 * MEGA_YEAR;

const US: u32 = 1_000;
const MS: u32 = 1_000 * US;

/// A unit of time, used with [`Duration::with_min_unit`] to set a floor
/// on which unit the formatter may select.
///
/// # Example
/// ```
/// use std::time::Duration;
/// use folktime::Folktime;
/// use folktime::duration::Unit;
///
/// let d = Folktime::duration(Duration::from_millis(500))
///     .with_min_unit(Unit::Second);
/// assert_eq!(format!("{}", d), "0.50s");
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

/// Formatting style for [core::time::Duration].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    /// ```
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
    /// ```
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
    pub(crate) duration: core::time::Duration,
    pub(crate) style: Style,
    pub(crate) min_unit: Unit,
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
    pub const fn with_style(self, style: Style) -> Self {
        Self { style, ..self }
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
    pub const fn with_min_unit(self, unit: Unit) -> Self {
        Self {
            min_unit: unit,
            ..self
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
