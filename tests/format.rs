use core::time::Duration;

use folktime::Folktime;
use folktime::duration::{Format, Style, Unit};

const FORMAT: Format = Format::new()
    .with_style(Style::TwoUnitsWhole)
    .with_min_unit(Unit::Microsecond)
    .with_greek_mu();

#[test]
fn default_format_matches_folktime_duration() {
    let d = Duration::from_secs(123);

    assert_eq!(
        format!("{}", Format::new().duration(d)),
        format!("{}", Folktime::duration(d))
    );
}

#[test]
fn format_reuses_options() {
    let a = FORMAT.duration(Duration::from_nanos(12_034));
    let b = FORMAT.duration(Duration::from_millis(1));

    assert_eq!(format!("{a}"), "12μs 34ns");
    assert_eq!(format!("{b}"), "1ms 0μs");
}
