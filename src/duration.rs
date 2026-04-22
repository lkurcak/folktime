mod one_unit_frac;
mod one_unit_whole;
mod two_units_whole;

use core::fmt::Display;

pub(crate) const MINUTE: u64 = 60;
pub(crate) const HOUR: u64 = 60 * MINUTE;
pub(crate) const DAY: u64 = 24 * HOUR;
pub(crate) const WEEK: u64 = 7 * DAY;
// Fixed Julian units used for approximate formatting.
pub(crate) const MONTH: u64 = 2_629_800; // = 365.25*24*60*60/12
pub(crate) const YEAR: u64 = 31_557_600; // = 365.25*24*60*60
pub(crate) const KILO_YEAR: u64 = 1_000 * YEAR;
pub(crate) const MEGA_YEAR: u64 = 1_000 * KILO_YEAR;
pub(crate) const GIGA_YEAR: u64 = 1_000 * MEGA_YEAR;

const US: u32 = 1_000;
const MS: u32 = 1_000 * US;

/// Units used with [`Duration::with_min_unit`] to keep the formatter from
/// choosing anything smaller.
///
/// # Example
/// ```
/// use core::time::Duration;
/// use folktime::Folktime;
/// use folktime::duration::Unit;
///
/// let a = Folktime::duration(Duration::from_millis(500));
/// let b = a.with_min_unit(Unit::Second);
///
/// assert_eq!(format!("{a}"), "500ms");
/// assert_eq!(format!("{b}"), "0.50s");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Unit {
    /// Nanoseconds (`ns`).
    Nanosecond,
    /// Microseconds (`us`).
    Microsecond,
    /// Milliseconds (`ms`).
    Millisecond,
    /// Seconds (`s`).
    Second,
    /// Minutes (`m`).
    Minute,
    /// Hours (`h`).
    Hour,
    /// Days (`d`).
    Day,
    /// Weeks (`w`).
    Week,
    /// Months (`mo`).
    Month,
    /// Years (`y`).
    Year,
    /// Thousand years (`ky`).
    KiloYear,
    /// Million years (`My`).
    MegaYear,
    /// Billion years (`Gy`).
    GigaYear,
}

/// How a [`Duration`] is rendered.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Style {
    /// Format in a single unit with a fractional part.
    ///
    /// # Example
    /// ```
    /// use core::time::Duration;
    /// use folktime::Folktime;
    /// use folktime::duration::Style;
    ///
    /// let d = Folktime::duration(Duration::from_secs(123)).with_style(Style::OneUnitFrac);
    /// assert_eq!(format!("{d}"), "2.05m");
    /// ```
    OneUnitFrac,
    /// Format in a single unit as a whole number.
    ///
    /// # Example
    /// ```
    /// use core::time::Duration;
    /// use folktime::Folktime;
    /// use folktime::duration::Style;
    ///
    /// let d = Folktime::duration(Duration::from_secs(123)).with_style(Style::OneUnitWhole);
    /// assert_eq!(format!("{d}"), "2m");
    /// ```
    OneUnitWhole,
    /// Format in two whole-number units.
    ///
    /// # Example
    /// ```
    /// use core::time::Duration;
    /// use folktime::Folktime;
    /// use folktime::duration::Style;
    ///
    /// let d = Folktime::duration(Duration::from_secs(123)).with_style(Style::TwoUnitsWhole);
    /// assert_eq!(format!("{d}"), "2m 3s");
    /// ```
    TwoUnitsWhole,
}

/// A configured formatter for a [`core::time::Duration`].
///
/// Create it with [`crate::Folktime::duration`], customize it with
/// [`Duration::with_style`] or [`Duration::with_min_unit`], and render it via
/// [`core::fmt::Display`].
#[allow(clippy::struct_field_names)]
#[derive(Debug, Clone, Copy)]
pub struct Duration {
    pub(crate) duration: core::time::Duration,
    pub(crate) style: Style,
    pub(crate) min_unit: Unit,
}

impl Duration {
    #[must_use]
    pub(crate) const fn new(d: core::time::Duration) -> Self {
        Self {
            duration: d,
            style: Style::OneUnitFrac,
            min_unit: Unit::Nanosecond,
        }
    }

    /// Set how the duration is rendered.
    ///
    /// The default is [`Style::OneUnitFrac`].
    ///
    /// # Example
    /// ```
    /// use core::time::Duration;
    /// use folktime::Folktime;
    /// use folktime::duration::Style;
    ///
    /// let d = Folktime::duration(Duration::from_secs(123)).with_style(Style::TwoUnitsWhole);
    /// assert_eq!(format!("{d}"), "2m 3s");
    /// ```
    #[must_use]
    pub const fn with_style(self, style: Style) -> Self {
        Self { style, ..self }
    }

    /// Set the smallest unit the formatter may choose.
    ///
    /// Prevents the formatter from switching to a smaller unit. Values below
    /// `1` of this unit are still expressed in terms of that unit rather than a
    /// smaller one:
    ///
    /// - `OneUnitWhole`: shows `"0"` + label (e.g. `"0s"`)
    /// - `OneUnitFrac`: shows the fractional value (e.g. `"0.50s"`)
    /// - `TwoUnitsWhole`: shows `"0"` + label with remainder (e.g. `"0s 500ms"`)
    ///
    /// # Example
    /// ```
    /// use core::time::Duration;
    /// use folktime::Folktime;
    /// use folktime::duration::{Style, Unit};
    ///
    /// let d = Folktime::duration(Duration::from_millis(500))
    ///     .with_min_unit(Unit::Second);
    /// assert_eq!(format!("{d}"), "0.50s");
    ///
    /// let d = Folktime::duration(Duration::from_millis(500))
    ///     .with_style(Style::OneUnitWhole)
    ///     .with_min_unit(Unit::Second);
    /// assert_eq!(format!("{d}"), "0s");
    /// ```
    #[must_use]
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
