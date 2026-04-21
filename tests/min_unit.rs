use folktime::duration::{Style, Unit};
use folktime::Folktime;
use std::time::Duration;

// --- OneUnitWhole with min_unit = Second ---

#[test]
fn one_unit_whole_min_s_zero() {
    let d = Folktime::duration(Duration::new(0, 0))
        .with_style(Style::OneUnitWhole)
        .with_min_unit(Unit::Second);
    assert_eq!(format!("{d}"), "0s");
}

#[test]
fn one_unit_whole_min_s_nanos() {
    let d = Folktime::duration(Duration::new(0, 123))
        .with_style(Style::OneUnitWhole)
        .with_min_unit(Unit::Second);
    assert_eq!(format!("{d}"), "0s");
}

#[test]
fn one_unit_whole_min_s_micros() {
    let d = Folktime::duration(Duration::new(0, 123_000))
        .with_style(Style::OneUnitWhole)
        .with_min_unit(Unit::Second);
    assert_eq!(format!("{d}"), "0s");
}

#[test]
fn one_unit_whole_min_s_millis() {
    let d = Folktime::duration(Duration::new(0, 123_000_000))
        .with_style(Style::OneUnitWhole)
        .with_min_unit(Unit::Second);
    assert_eq!(format!("{d}"), "0s");
}

#[test]
fn one_unit_whole_min_s_999ms() {
    let d = Folktime::duration(Duration::new(0, 999_999_999))
        .with_style(Style::OneUnitWhole)
        .with_min_unit(Unit::Second);
    assert_eq!(format!("{d}"), "0s");
}

#[test]
fn one_unit_whole_min_s_1s() {
    let d = Folktime::duration(Duration::new(1, 0))
        .with_style(Style::OneUnitWhole)
        .with_min_unit(Unit::Second);
    assert_eq!(format!("{d}"), "1s");
}

#[test]
fn one_unit_whole_min_s_1_23s() {
    let d = Folktime::duration(Duration::new(1, 230_000_000))
        .with_style(Style::OneUnitWhole)
        .with_min_unit(Unit::Second);
    assert_eq!(format!("{d}"), "1s");
}

#[test]
fn one_unit_whole_min_s_59s() {
    let d = Folktime::duration(Duration::new(59, 999_999_999))
        .with_style(Style::OneUnitWhole)
        .with_min_unit(Unit::Second);
    assert_eq!(format!("{d}"), "59s");
}

#[test]
fn one_unit_whole_min_s_60s() {
    // Above seconds, normal formatting picks minutes
    let d = Folktime::duration(Duration::new(60, 0))
        .with_style(Style::OneUnitWhole)
        .with_min_unit(Unit::Second);
    assert_eq!(format!("{d}"), "1m");
}

// --- OneUnitFrac with min_unit = Second ---

#[test]
fn one_unit_frac_min_s_zero() {
    let d = Folktime::duration(Duration::new(0, 0))
        .with_style(Style::OneUnitFrac)
        .with_min_unit(Unit::Second);
    assert_eq!(format!("{d}"), "0.00s");
}

#[test]
fn one_unit_frac_min_s_nanos() {
    let d = Folktime::duration(Duration::new(0, 500))
        .with_style(Style::OneUnitFrac)
        .with_min_unit(Unit::Second);
    assert_eq!(format!("{d}"), "0.00s");
}

#[test]
fn one_unit_frac_min_s_millis() {
    let d = Folktime::duration(Duration::new(0, 500_000_000))
        .with_style(Style::OneUnitFrac)
        .with_min_unit(Unit::Second);
    assert_eq!(format!("{d}"), "0.50s");
}

#[test]
fn one_unit_frac_min_s_1s() {
    // At or above 1s, normal frac formatting applies
    let d = Folktime::duration(Duration::new(1, 230_000_000))
        .with_style(Style::OneUnitFrac)
        .with_min_unit(Unit::Second);
    assert_eq!(format!("{d}"), "1.23s");
}

// --- TwoUnitsWhole with min_unit = Second ---

#[test]
fn two_units_whole_min_s_zero() {
    let d = Folktime::duration(Duration::new(0, 0))
        .with_style(Style::TwoUnitsWhole)
        .with_min_unit(Unit::Second);
    assert_eq!(format!("{d}"), "0s 0ms");
}

#[test]
fn two_units_whole_min_s_millis() {
    let d = Folktime::duration(Duration::new(0, 500_000_000))
        .with_style(Style::TwoUnitsWhole)
        .with_min_unit(Unit::Second);
    assert_eq!(format!("{d}"), "0s 500ms");
}

#[test]
fn two_units_whole_min_s_1s() {
    let d = Folktime::duration(Duration::new(1, 500_000_000))
        .with_style(Style::TwoUnitsWhole)
        .with_min_unit(Unit::Second);
    assert_eq!(format!("{d}"), "1s 500ms");
}

// --- min_unit = Minute ---

#[test]
fn one_unit_whole_min_m_30s() {
    let d = Folktime::duration(Duration::new(30, 0))
        .with_style(Style::OneUnitWhole)
        .with_min_unit(Unit::Minute);
    assert_eq!(format!("{d}"), "0m");
}

#[test]
fn one_unit_whole_min_m_60s() {
    let d = Folktime::duration(Duration::new(60, 0))
        .with_style(Style::OneUnitWhole)
        .with_min_unit(Unit::Minute);
    assert_eq!(format!("{d}"), "1m");
}

#[test]
fn one_unit_frac_min_m_30s() {
    let d = Folktime::duration(Duration::new(30, 0))
        .with_style(Style::OneUnitFrac)
        .with_min_unit(Unit::Minute);
    assert_eq!(format!("{d}"), "0.50m");
}

#[test]
fn one_unit_frac_min_m_500ms() {
    let d = Folktime::duration(Duration::new(0, 500_000_000))
        .with_style(Style::OneUnitFrac)
        .with_min_unit(Unit::Minute);
    assert_eq!(format!("{d}"), "0.00m");
}

#[test]
fn two_units_whole_min_m_30s() {
    let d = Folktime::duration(Duration::new(30, 0))
        .with_style(Style::TwoUnitsWhole)
        .with_min_unit(Unit::Minute);
    assert_eq!(format!("{d}"), "0m 30s");
}

// --- min_unit = Millisecond ---

#[test]
fn one_unit_whole_min_ms_500us() {
    let d = Folktime::duration(Duration::new(0, 500_000))
        .with_style(Style::OneUnitWhole)
        .with_min_unit(Unit::Millisecond);
    assert_eq!(format!("{d}"), "0ms");
}

#[test]
fn one_unit_whole_min_ms_1ms() {
    let d = Folktime::duration(Duration::new(0, 1_000_000))
        .with_style(Style::OneUnitWhole)
        .with_min_unit(Unit::Millisecond);
    assert_eq!(format!("{d}"), "1ms");
}

#[test]
fn one_unit_whole_min_ms_500ns() {
    let d = Folktime::duration(Duration::new(0, 500))
        .with_style(Style::OneUnitWhole)
        .with_min_unit(Unit::Millisecond);
    assert_eq!(format!("{d}"), "0ms");
}

#[test]
fn one_unit_frac_min_ms_500us() {
    let d = Folktime::duration(Duration::new(0, 500_000))
        .with_style(Style::OneUnitFrac)
        .with_min_unit(Unit::Millisecond);
    assert_eq!(format!("{d}"), "0.50ms");
}

#[test]
fn one_unit_frac_min_ms_50ns() {
    let d = Folktime::duration(Duration::new(0, 50))
        .with_style(Style::OneUnitFrac)
        .with_min_unit(Unit::Millisecond);
    assert_eq!(format!("{d}"), "0.00ms");
}

// --- min_unit = Nanosecond (no-op) ---

#[test]
fn one_unit_whole_min_ns_1ns() {
    // Nanosecond min_unit should be a no-op for non-zero values
    let d = Folktime::duration(Duration::new(0, 1))
        .with_style(Style::OneUnitWhole)
        .with_min_unit(Unit::Nanosecond);
    assert_eq!(format!("{d}"), "1ns");
}

// --- no min_unit (existing behavior preserved) ---

#[test]
fn no_min_unit_preserves_behavior() {
    let d = Folktime::duration(Duration::new(0, 123)).with_style(Style::OneUnitWhole);
    assert_eq!(format!("{d}"), "123ns");
}

#[test]
fn no_min_unit_frac_preserves_behavior() {
    let d = Folktime::duration(Duration::new(0, 0)).with_style(Style::OneUnitFrac);
    assert_eq!(format!("{d}"), "0.00s");
}
