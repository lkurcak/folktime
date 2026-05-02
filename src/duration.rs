mod one_unit_frac;
mod one_unit_mini;
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

/// Units used with [`Duration::with_min_unit`] to set the minimum primary unit.
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
    /// Microseconds (`us` by default, or `μs` with [`Duration::with_greek_mu`]).
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
#[non_exhaustive]
pub enum Style {
    /// Format in one unit using the shortest fractional representation.
    ///
    /// Values below `10` use one decimal digit. Larger values use whole units.
    ///
    /// # Example
    /// ```
    /// use core::time::Duration;
    /// use folktime::Folktime;
    /// use folktime::duration::Style;
    ///
    /// let d = Folktime::duration(Duration::from_secs(123)).with_style(Style::Mini);
    /// assert_eq!(format!("{d}"), "2.0m");
    /// ```
    Mini,
    /// Format in one unit with a compact fractional part.
    ///
    /// # Example
    /// ```
    /// use core::time::Duration;
    /// use folktime::Folktime;
    /// use folktime::duration::Style;
    ///
    /// let d = Folktime::duration(Duration::from_secs(123)).with_style(Style::Compact);
    /// assert_eq!(format!("{d}"), "2.05m");
    /// ```
    Compact,
    /// Format in one unit as a whole number.
    ///
    /// # Example
    /// ```
    /// use core::time::Duration;
    /// use folktime::Folktime;
    /// use folktime::duration::Style;
    ///
    /// let d = Folktime::duration(Duration::from_secs(123)).with_style(Style::Whole);
    /// assert_eq!(format!("{d}"), "2m");
    /// ```
    Whole,
    /// Format in two whole-number units.
    ///
    /// # Example
    /// ```
    /// use core::time::Duration;
    /// use folktime::Folktime;
    /// use folktime::duration::Style;
    ///
    /// let d = Folktime::duration(Duration::from_secs(123)).with_style(Style::Detailed);
    /// assert_eq!(format!("{d}"), "2m 3s");
    /// ```
    Detailed,
}

impl Style {
    /// Deprecated alias for [`Style::Compact`].
    #[allow(non_upper_case_globals)]
    #[deprecated(since = "0.5.0", note = "use Style::Compact instead")]
    #[doc(hidden)]
    pub const OneUnitFrac: Self = Self::Compact;

    /// Deprecated alias for [`Style::Whole`].
    #[allow(non_upper_case_globals)]
    #[deprecated(since = "0.5.0", note = "use Style::Whole instead")]
    #[doc(hidden)]
    pub const OneUnitWhole: Self = Self::Whole;

    /// Deprecated alias for [`Style::Detailed`].
    #[allow(non_upper_case_globals)]
    #[deprecated(since = "0.5.0", note = "use Style::Detailed instead")]
    #[doc(hidden)]
    pub const TwoUnitsWhole: Self = Self::Detailed;
}

/// Reusable configuration for duration formatting.
///
/// Use [`Format::duration`] to apply the same formatting options to multiple
/// [`core::time::Duration`] values.
///
/// # Example
/// ```
/// use core::time::Duration;
/// use folktime::duration::{Format, Style, Unit};
///
/// const FORMAT: Format = Format::new()
///     .with_style(Style::Detailed)
///     .with_min_unit(Unit::Microsecond)
///     .with_greek_mu();
///
/// let a = FORMAT.duration(Duration::from_nanos(12_034));
/// let b = FORMAT.duration(Duration::from_micros(1_012));
///
/// assert_eq!(format!("{a}"), "12μs 34ns");
/// assert_eq!(format!("{b}"), "1ms 12μs");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Format {
    style: Style,
    min_unit: Unit,
    greek_mu: bool,
}

impl Default for Format {
    fn default() -> Self {
        Self::new()
    }
}

impl Format {
    /// Create a reusable duration format with default options.
    ///
    /// Defaults to [`Style::Compact`], [`Unit::Nanosecond`], and ASCII
    /// microseconds (`us`).
    #[must_use]
    pub const fn new() -> Self {
        Self {
            style: Style::Compact,
            min_unit: Unit::Nanosecond,
            greek_mu: false,
        }
    }

    /// Create a formatter wrapper around `d` using this format.
    #[must_use]
    pub const fn duration(self, d: core::time::Duration) -> Duration {
        Duration {
            duration: d,
            style: self.style,
            min_unit: self.min_unit,
            greek_mu: self.greek_mu,
        }
    }

    /// Set how durations are rendered.
    ///
    /// The default is [`Style::Compact`].
    #[must_use]
    pub const fn with_style(self, style: Style) -> Self {
        Self { style, ..self }
    }

    /// Set the minimum primary unit used to display durations.
    #[must_use]
    pub const fn with_min_unit(self, unit: Unit) -> Self {
        Self {
            min_unit: unit,
            ..self
        }
    }

    /// Render microseconds with Greek small letter mu (`μs`) instead of ASCII
    /// `us`.
    #[must_use]
    pub const fn with_greek_mu(self) -> Self {
        Self {
            greek_mu: true,
            ..self
        }
    }
}

/// A display wrapper for a [`core::time::Duration`].
///
/// Create it with [`crate::Folktime::duration`] or [`Format::duration`],
/// customize it with [`Duration::with_style`] or [`Duration::with_min_unit`],
/// and render it via [`core::fmt::Display`].
#[allow(clippy::struct_field_names)]
#[derive(Debug, Clone, Copy)]
pub struct Duration {
    pub(crate) duration: core::time::Duration,
    pub(crate) style: Style,
    pub(crate) min_unit: Unit,
    pub(crate) greek_mu: bool,
}

impl Duration {
    /// Set how the duration is rendered.
    ///
    /// The default is [`Style::Compact`].
    ///
    /// # Example
    /// ```
    /// use core::time::Duration;
    /// use folktime::Folktime;
    /// use folktime::duration::Style;
    ///
    /// let d = Folktime::duration(Duration::from_secs(123)).with_style(Style::Detailed);
    /// assert_eq!(format!("{d}"), "2m 3s");
    /// ```
    #[must_use]
    pub const fn with_style(self, style: Style) -> Self {
        Self { style, ..self }
    }

    /// Render microseconds with Greek small letter mu (`μs`) instead of ASCII
    /// `us`.
    ///
    /// The rendered prefix uses Greek small letter mu (`U+03BC`), not the
    /// legacy MICRO SIGN character (`U+00B5`).
    ///
    /// This only affects outputs that include microseconds.
    ///
    /// # Example
    /// ```
    /// use core::time::Duration;
    /// use folktime::Folktime;
    /// use folktime::duration::Style;
    ///
    /// let d = Folktime::duration(Duration::from_micros(12))
    ///     .with_style(Style::Whole)
    ///     .with_greek_mu();
    /// assert_eq!(format!("{d}"), "12μs");
    /// ```
    #[must_use]
    pub const fn with_greek_mu(self) -> Self {
        Self {
            greek_mu: true,
            ..self
        }
    }

    /// Set the minimum primary unit used to display the duration.
    ///
    /// This is useful when small durations should still be displayed in a
    /// larger unit. For example, `500ms` can be displayed in seconds instead of
    /// milliseconds.
    ///
    /// # Example
    /// ```
    /// use core::time::Duration;
    /// use folktime::Folktime;
    /// use folktime::duration::{Style, Unit};
    ///
    /// let d = Folktime::duration(Duration::from_millis(500)).with_min_unit(Unit::Second);
    /// assert_eq!(format!("{d}"), "0.50s");
    ///
    /// let whole = d.with_style(Style::Whole);
    /// assert_eq!(format!("{whole}"), "0s");
    ///
    /// let two = d.with_style(Style::Detailed);
    /// assert_eq!(format!("{two}"), "0s 500ms");
    /// ```
    #[must_use]
    pub const fn with_min_unit(self, unit: Unit) -> Self {
        Self {
            min_unit: unit,
            ..self
        }
    }

    pub(crate) const fn microsecond_label(&self) -> &'static str {
        if self.greek_mu { "μs" } else { "us" }
    }
}

impl Display for Duration {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.style {
            Style::Mini => self.fmt_one_unit_mini(f),
            Style::Compact => self.fmt_one_unit_frac(f),
            Style::Whole => self.fmt_one_unit_whole(f),
            Style::Detailed => self.fmt_two_units_whole(f),
        }
    }
}
