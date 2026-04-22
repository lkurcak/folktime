//! Tiny `no_std`, zero-allocation library for approximate human-friendly duration formatting.

#![no_std]
#![warn(clippy::all, clippy::pedantic, clippy::mod_module_files)]

pub mod duration;

use duration::Duration;

#[cfg(doctest)]
#[doc = include_str!("../README.md")]
mod readme_doctests {}

/// Entry point for formatting values.
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
    /// Format a [`core::time::Duration`] in a human-friendly way.
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
    /// Formatting only shows the most significant digits:
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
    /// There are several styles for formatting:
    /// ```
    /// # use core::time::Duration;
    /// # use folktime::Folktime;
    /// use folktime::duration::Style;
    ///
    /// let a = Folktime::duration(Duration::new(0, 12_056_999));
    /// let b = Folktime::duration(Duration::new(0, 12_056_999)).with_style(Style::OneUnitWhole);
    /// let c = Folktime::duration(Duration::new(0, 12_056_999)).with_style(Style::TwoUnitsWhole);
    ///
    /// assert_eq!(format!("{a}"), "12.0ms");
    /// assert_eq!(format!("{b}"), "12ms");
    /// assert_eq!(format!("{c}"), "12ms 56us");
    /// ```
    ///
    /// # Minimum unit
    /// Use [`Duration::with_min_unit`] to set a floor on the displayed unit:
    /// ```
    /// # use core::time::Duration;
    /// # use folktime::Folktime;
    /// use folktime::duration::Unit;
    ///
    /// let d = Folktime::duration(Duration::from_millis(500)).with_min_unit(Unit::Second);
    /// assert_eq!(format!("{d}"), "0.50s");
    /// ```
    #[must_use]
    pub const fn duration(d: core::time::Duration) -> Duration {
        Duration::new(d)
    }
}
