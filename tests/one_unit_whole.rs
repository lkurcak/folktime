mod common;

use common::*;
use folktime::duration::Style;
use folktime::Folktime;
use std::time::Duration;

const STYLE: Style = Style::OneUnitWhole;

#[test]
fn zero() {
    let d = Folktime::duration(Duration::new(0, 0)).with_style(STYLE);
    assert_eq!(format!("{}", d), "0s");
}
#[test]
fn max() {
    let d = Folktime::duration(Duration::new(u64::MAX, 999_999_999)).with_style(STYLE);
    assert_eq!(format!("{}", d), "584Gy");
}
#[test]
fn test() {
    let d = Folktime::duration(Duration::from_secs(12345689)).with_style(STYLE);
    assert_eq!(format!("{}", d), "4mo");
}
#[test]
fn test2() {
    let d = Folktime::duration(Duration::from_secs(1234568)).with_style(STYLE);
    assert_eq!(format!("{}", d), "2w");
}

#[test]
fn ns_0() {
    let d = Folktime::duration(Duration::new(0, 1)).with_style(STYLE);
    assert_eq!(format!("{}", d), "1ns");
}
#[test]
fn ns_1() {
    let d = Folktime::duration(Duration::new(0, 999)).with_style(STYLE);
    assert_eq!(format!("{}", d), "999ns");
}

#[test]
fn us_0() {
    let d = Folktime::duration(Duration::new(0, 1_000)).with_style(STYLE);
    assert_eq!(format!("{}", d), "1us");
}
#[test]
fn us_1() {
    let d = Folktime::duration(Duration::new(0, 1_001)).with_style(STYLE);
    assert_eq!(format!("{}", d), "1us");
}
#[test]
fn us_2() {
    let d = Folktime::duration(Duration::new(0, 1_999)).with_style(STYLE);
    assert_eq!(format!("{}", d), "1us");
}
#[test]
fn us_3() {
    let d = Folktime::duration(Duration::new(0, 999_999)).with_style(STYLE);
    assert_eq!(format!("{}", d), "999us");
}

#[test]
fn ms_0() {
    let d = Folktime::duration(Duration::new(0, 1_000_000)).with_style(STYLE);
    assert_eq!(format!("{}", d), "1ms");
}
#[test]
fn ms_1() {
    let d = Folktime::duration(Duration::new(0, 1_000_001)).with_style(STYLE);
    assert_eq!(format!("{}", d), "1ms");
}
#[test]
fn ms_2() {
    let d = Folktime::duration(Duration::new(0, 1_000_999)).with_style(STYLE);
    assert_eq!(format!("{}", d), "1ms");
}
#[test]
fn ms_3() {
    let d = Folktime::duration(Duration::new(0, 1_001_000)).with_style(STYLE);
    assert_eq!(format!("{}", d), "1ms");
}
#[test]
fn ms_4() {
    let d = Folktime::duration(Duration::new(0, 1_999_999)).with_style(STYLE);
    assert_eq!(format!("{}", d), "1ms");
}
#[test]
fn ms_5() {
    let d = Folktime::duration(Duration::new(0, 999_999_999)).with_style(STYLE);
    assert_eq!(format!("{}", d), "999ms");
}

#[test]
fn s_0() {
    let d = Folktime::duration(Duration::new(1, 0)).with_style(STYLE);
    assert_eq!(format!("{}", d), "1s");
}
#[test]
fn s_1() {
    let d = Folktime::duration(Duration::new(1, 999_999)).with_style(STYLE);
    assert_eq!(format!("{}", d), "1s");
}
#[test]
fn s_2() {
    let d = Folktime::duration(Duration::new(1, 1_000_000)).with_style(STYLE);
    assert_eq!(format!("{}", d), "1s");
}
#[test]
fn s_3() {
    let d = Folktime::duration(Duration::new(59, 999_999_999)).with_style(STYLE);
    assert_eq!(format!("{}", d), "59s");
}

#[test]
fn m_0() {
    let d = Folktime::duration(Duration::new(60, 0)).with_style(STYLE);
    assert_eq!(format!("{}", d), "1m");
}
#[test]
fn m_1() {
    let d = Folktime::duration(Duration::new(60, 999_999_999)).with_style(STYLE);
    assert_eq!(format!("{}", d), "1m");
}
#[test]
fn m_2() {
    let d = Folktime::duration(Duration::new(61, 0)).with_style(STYLE);
    assert_eq!(format!("{}", d), "1m");
}
#[test]
fn m_3() {
    let d = Folktime::duration(Duration::new(HOUR - 1, 999_999_999)).with_style(STYLE);
    assert_eq!(format!("{}", d), "59m");
}

#[test]
fn h_0() {
    let d = Folktime::duration(Duration::new(HOUR, 0)).with_style(STYLE);
    assert_eq!(format!("{}", d), "1h");
}
#[test]
fn h_1() {
    let d = Folktime::duration(Duration::new(HOUR + MIN - 1, 999_999_999)).with_style(STYLE);
    assert_eq!(format!("{}", d), "1h");
}
#[test]
fn h_2() {
    let d = Folktime::duration(Duration::new(HOUR + MIN, 0)).with_style(STYLE);
    assert_eq!(format!("{}", d), "1h");
}
#[test]
fn h_3() {
    let d = Folktime::duration(Duration::new(DAY - 1, 999_999_999)).with_style(STYLE);
    assert_eq!(format!("{}", d), "23h");
}

#[test]
fn d_0() {
    let d = Folktime::duration(Duration::new(DAY, 0)).with_style(STYLE);
    assert_eq!(format!("{}", d), "1d");
}
#[test]
fn d_1() {
    let d = Folktime::duration(Duration::new(DAY + HOUR - 1, 999_999_999)).with_style(STYLE);
    assert_eq!(format!("{}", d), "1d");
}
#[test]
fn d_2() {
    let d = Folktime::duration(Duration::new(DAY + HOUR, 0)).with_style(STYLE);
    assert_eq!(format!("{}", d), "1d");
}
#[test]
fn d_3() {
    let d = Folktime::duration(Duration::new(WEEK - 1, 999_999_999)).with_style(STYLE);
    assert_eq!(format!("{}", d), "6d");
}

#[test]
fn w_0() {
    let d = Folktime::duration(Duration::new(WEEK, 0)).with_style(STYLE);
    assert_eq!(format!("{}", d), "1w");
}
#[test]
fn w_1() {
    let d = Folktime::duration(Duration::new(WEEK + DAY - 1, 999_999_999)).with_style(STYLE);
    assert_eq!(format!("{}", d), "1w");
}
#[test]
fn w_2() {
    let d = Folktime::duration(Duration::new(WEEK + DAY, 0)).with_style(STYLE);
    assert_eq!(format!("{}", d), "1w");
}
#[test]
fn w_3() {
    let d = Folktime::duration(Duration::new(MONTH - 1, 999_999_999)).with_style(STYLE);
    assert_eq!(format!("{}", d), "4w");
}

#[test]
fn mo_0() {
    let d = Folktime::duration(Duration::new(MONTH, 0)).with_style(STYLE);
    assert_eq!(format!("{}", d), "1mo");
}
#[test]
fn mo_1() {
    let d = Folktime::duration(Duration::new(MONTH + DAY - 1, 999_999_999)).with_style(STYLE);
    assert_eq!(format!("{}", d), "1mo");
}
#[test]
fn mo_2() {
    let d = Folktime::duration(Duration::new(MONTH + DAY, 0)).with_style(STYLE);
    assert_eq!(format!("{}", d), "1mo");
}
#[test]
fn mo_3() {
    let d = Folktime::duration(Duration::new(YEAR - 1, 999_999_999)).with_style(STYLE);
    assert_eq!(format!("{}", d), "11mo");
}

#[test]
fn y_0() {
    let d = Folktime::duration(Duration::new(YEAR, 0)).with_style(STYLE);
    assert_eq!(format!("{}", d), "1y");
}
#[test]
fn y_1() {
    let d = Folktime::duration(Duration::new(YEAR + MONTH - 1, 999_999_999)).with_style(STYLE);
    assert_eq!(format!("{}", d), "1y");
}
#[test]
fn y_2() {
    let d = Folktime::duration(Duration::new(YEAR + MONTH, 0)).with_style(STYLE);
    assert_eq!(format!("{}", d), "1y");
}
#[test]
fn y_3() {
    let d = Folktime::duration(Duration::new(2 * YEAR - 1, 999_999_999)).with_style(STYLE);
    assert_eq!(format!("{}", d), "1y");
}
#[test]
fn y_4() {
    let d = Folktime::duration(Duration::new(2 * YEAR, 0)).with_style(STYLE);
    assert_eq!(format!("{}", d), "2y");
}
#[test]
fn y_5() {
    let d = Folktime::duration(Duration::new(1000 * YEAR - 1, 999_999_999)).with_style(STYLE);
    assert_eq!(format!("{}", d), "999y");
}

#[test]
fn ky_0() {
    let d = Folktime::duration(Duration::new(1000 * YEAR, 0)).with_style(STYLE);
    assert_eq!(format!("{}", d), "1ky");
}
#[test]
fn ky_1() {
    let d =
        Folktime::duration(Duration::new(1000 * YEAR + YEAR - 1, 999_999_999)).with_style(STYLE);
    assert_eq!(format!("{}", d), "1ky");
}
#[test]
fn ky_2() {
    let d = Folktime::duration(Duration::new(1000 * YEAR + YEAR, 0)).with_style(STYLE);
    assert_eq!(format!("{}", d), "1ky");
}
#[test]
fn ky_3() {
    let d = Folktime::duration(Duration::new(MEGA_YEAR - 1, 999_999_999)).with_style(STYLE);
    assert_eq!(format!("{}", d), "999ky");
}

#[test]
fn my_0() {
    let d = Folktime::duration(Duration::new(MEGA_YEAR, 0)).with_style(STYLE);
    assert_eq!(format!("{}", d), "1My");
}
#[test]
fn my_1() {
    let d = Folktime::duration(Duration::new(MEGA_YEAR + 1000 * YEAR - 1, 999_999_999))
        .with_style(STYLE);
    assert_eq!(format!("{}", d), "1My");
}
#[test]
fn my_2() {
    let d = Folktime::duration(Duration::new(MEGA_YEAR + 1000 * YEAR, 0)).with_style(STYLE);
    assert_eq!(format!("{}", d), "1My");
}
#[test]
fn my_3() {
    let d = Folktime::duration(Duration::new(GIGA_YEAR - 1, 999_999_999)).with_style(STYLE);
    assert_eq!(format!("{}", d), "999My");
}

#[test]
fn gy_0() {
    let d = Folktime::duration(Duration::new(GIGA_YEAR, 0)).with_style(STYLE);
    assert_eq!(format!("{}", d), "1Gy");
}
#[test]
fn gy_1() {
    let d = Folktime::duration(Duration::new(GIGA_YEAR + MEGA_YEAR - 1, 999_999_999))
        .with_style(STYLE);
    assert_eq!(format!("{}", d), "1Gy");
}
#[test]
fn gy_2() {
    let d = Folktime::duration(Duration::new(GIGA_YEAR + MEGA_YEAR, 0)).with_style(STYLE);
    assert_eq!(format!("{}", d), "1Gy");
}
#[test]
fn gy_3() {
    let d = Folktime::duration(Duration::new(500 * GIGA_YEAR, 0)).with_style(STYLE);
    assert_eq!(format!("{}", d), "500Gy");
}
