//! Tiny `no_std`, zero-allocation formatter for `core::time::Duration`.

#![no_std]
#![warn(clippy::all, clippy::pedantic, clippy::mod_module_files)]

/// Configuration types for duration formatting.
pub mod duration;

use duration::Duration;

#[cfg(doctest)]
#[doc = include_str!("../README.md")]
mod readme_doctests {}

/// Entry point for constructing duration formatters.
///
/// # Example
/// ```
/// use core::time::Duration;
/// use folktime::Folktime;
///
/// let d = Folktime::duration(Duration::from_secs(5));
/// assert_eq!(format!("{d}"), "5.00s");
/// ```
pub struct Folktime;

impl Folktime {
    /// Create a formatter wrapper around `d`.
    ///
    /// # Example
    /// ```
    /// use core::time::Duration;
    /// use folktime::Folktime;
    ///
    /// let d = Folktime::duration(Duration::from_secs(5));
    /// assert_eq!(format!("{d}"), "5.00s");
    /// ```
    ///
    /// # Precision
    ///
    /// Formatting is intentionally approximate and keeps only the most significant digits:
    /// ```
    /// # use core::time::Duration;
    /// # use folktime::Folktime;
    /// #
    /// let a = Folktime::duration(Duration::new(0, 123_456_789));
    /// let b = Folktime::duration(Duration::new(1, 123_456_789));
    /// let c = Folktime::duration(Duration::new(12, 123_456_789));
    /// let d = Folktime::duration(Duration::new(123, 123_456_789));
    ///
    /// assert_eq!(format!("{a}"), "123ms");
    /// assert_eq!(format!("{b}"), "1.12s");
    /// assert_eq!(format!("{c}"), "12.1s");
    /// assert_eq!(format!("{d}"), "2.05m");
    /// ```
    ///
    /// # Formatting styles
    /// Choose between the available formatting styles:
    /// ```
    /// # use core::time::Duration;
    /// # use folktime::Folktime;
    /// use folktime::duration::Style;
    ///
    /// let a = Folktime::duration(Duration::new(0, 12_056_999));
    /// let b = a.with_style(Style::OneUnitWhole);
    /// let c = a.with_style(Style::TwoUnitsWhole);
    ///
    /// assert_eq!(format!("{a}"), "12.0ms");
    /// assert_eq!(format!("{b}"), "12ms");
    /// assert_eq!(format!("{c}"), "12ms 56us");
    /// ```
    ///
    /// # Minimum unit
    /// Use [`Duration::with_min_unit`] to prevent the formatter from choosing a
    /// smaller unit:
    /// ```
    /// # use core::time::Duration;
    /// # use folktime::Folktime;
    /// use folktime::duration::Unit;
    ///
    /// let a = Folktime::duration(Duration::from_millis(500));
    /// let b = a.with_min_unit(Unit::Second);
    ///
    /// assert_eq!(format!("{a}"), "500ms");
    /// assert_eq!(format!("{b}"), "0.50s");
    /// ```
    #[must_use]
    pub const fn duration(d: core::time::Duration) -> Duration {
        Duration::new(d)
    }
}
