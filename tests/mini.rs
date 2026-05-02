mod common;

use common::*;
use core::time::Duration;
use folktime::Folktime;
use folktime::duration::{Style, Unit};

const STYLE: Style = Style::Mini;

#[test]
fn zero() {
    let d = Folktime::duration(Duration::ZERO).with_style(STYLE);
    assert_eq!(format!("{d}"), "0.0s");
}

#[test]
fn max() {
    let d = Folktime::duration(Duration::new(u64::MAX, 999_999_999)).with_style(STYLE);
    assert_eq!(format!("{d}"), "584Gy");
}

#[test]
fn nanoseconds() {
    let d = Folktime::duration(Duration::new(0, 999)).with_style(STYLE);
    assert_eq!(format!("{d}"), "999ns");
}

#[test]
fn microseconds_below_ten_keep_one_decimal() {
    let d = Folktime::duration(Duration::new(0, 1_234)).with_style(STYLE);
    assert_eq!(format!("{d}"), "1.2us");
}

#[test]
fn microseconds_above_ten_are_whole() {
    let d = Folktime::duration(Duration::new(0, 12_999)).with_style(STYLE);
    assert_eq!(format!("{d}"), "12us");
}

#[test]
fn microseconds_can_use_greek_mu() {
    let d = Folktime::duration(Duration::new(0, 1_234))
        .with_style(STYLE)
        .with_greek_mu();
    assert_eq!(format!("{d}"), "1.2μs");
}

#[test]
fn milliseconds_below_ten_keep_one_decimal() {
    let d = Folktime::duration(Duration::new(0, 1_234_000)).with_style(STYLE);
    assert_eq!(format!("{d}"), "1.2ms");
}

#[test]
fn milliseconds_above_ten_are_whole() {
    let d = Folktime::duration(Duration::new(0, 12_999_999)).with_style(STYLE);
    assert_eq!(format!("{d}"), "12ms");
}

#[test]
fn seconds_below_ten_keep_one_decimal() {
    let d = Folktime::duration(Duration::new(1, 123_456_789)).with_style(STYLE);
    assert_eq!(format!("{d}"), "1.1s");
}

#[test]
fn seconds_above_ten_are_whole() {
    let d = Folktime::duration(Duration::new(12, 999_999_999)).with_style(STYLE);
    assert_eq!(format!("{d}"), "12s");
}

#[test]
fn minutes_below_ten_keep_one_decimal() {
    let d = Folktime::duration(Duration::from_secs(123)).with_style(STYLE);
    assert_eq!(format!("{d}"), "2.0m");
}

#[test]
fn minutes_above_ten_are_whole() {
    let d = Folktime::duration(Duration::from_secs(10 * MINUTE)).with_style(STYLE);
    assert_eq!(format!("{d}"), "10m");
}

#[test]
fn larger_units() {
    let h = Folktime::duration(Duration::from_secs(3_660)).with_style(STYLE);
    let w = Folktime::duration(Duration::from_secs(777_600)).with_style(STYLE);
    let mo = Folktime::duration(Duration::from_secs(12_345_689)).with_style(STYLE);

    assert_eq!(format!("{h}"), "1.0h");
    assert_eq!(format!("{w}"), "1.2w");
    assert_eq!(format!("{mo}"), "4.6mo");
}

#[test]
fn min_unit_second() {
    let d = Folktime::duration(Duration::from_millis(500))
        .with_style(STYLE)
        .with_min_unit(Unit::Second);
    assert_eq!(format!("{d}"), "0.5s");
}

#[test]
fn min_unit_minute() {
    let a = Folktime::duration(Duration::from_millis(500))
        .with_style(STYLE)
        .with_min_unit(Unit::Minute);
    let b = Folktime::duration(Duration::from_secs(30))
        .with_style(STYLE)
        .with_min_unit(Unit::Minute);

    assert_eq!(format!("{a}"), "0.0m");
    assert_eq!(format!("{b}"), "0.5m");
}
