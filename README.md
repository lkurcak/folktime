# folktime

[![CI](https://github.com/lkurcak/folktime/workflows/CI/badge.svg)](https://github.com/lkurcak/folktime/actions)
[![Crates.io](https://img.shields.io/crates/v/folktime.svg)](https://crates.io/crates/folktime)

Tiny `no_std`, zero-allocation formatter for [`core::time::Duration`](https://doc.rust-lang.org/stable/core/time/struct.Duration.html).

`folktime` is intentionally approximate. If you need an exact format, take a look at [`humantime`](https://crates.io/crates/humantime).

## Example

```rust
use core::time::Duration;
use folktime::Folktime;

let d = Folktime::duration(Duration::from_secs(123));
assert_eq!(format!("{d}"), "2.05m");
```

## Styles

Choose between three formatting styles:

- [`Style::OneUnitFrac`](https://docs.rs/folktime/latest/folktime/duration/enum.Style.html#variant.OneUnitFrac) (default): one unit with a fractional part
- [`Style::OneUnitWhole`](https://docs.rs/folktime/latest/folktime/duration/enum.Style.html#variant.OneUnitWhole): one whole-number unit
- [`Style::TwoUnitsWhole`](https://docs.rs/folktime/latest/folktime/duration/enum.Style.html#variant.TwoUnitsWhole): two whole-number units

```rust
use core::time::Duration;
use folktime::Folktime;
use folktime::duration::Style;

let a = Folktime::duration(Duration::new(0, 12_056_999));
let b = a.with_style(Style::OneUnitWhole);
let c = a.with_style(Style::TwoUnitsWhole);

assert_eq!(format!("{a}"), "12.0ms");
assert_eq!(format!("{b}"), "12ms");
assert_eq!(format!("{c}"), "12ms 56us");
```

Here's a comparison of styles:

| Duration     | [`Style::OneUnitFrac`](https://docs.rs/folktime/latest/folktime/duration/enum.Style.html#variant.OneUnitFrac) | [`Style::OneUnitWhole`](https://docs.rs/folktime/latest/folktime/duration/enum.Style.html#variant.OneUnitWhole) | [`Style::TwoUnitsWhole`](https://docs.rs/folktime/latest/folktime/duration/enum.Style.html#variant.TwoUnitsWhole) |
|-------------:|---------------------:|----------------------:|-----------------------:|
| `0s`         | `0.00s`              | `0s`                  | `0s 0ms`               |
| `0.123456s`  | `123ms`              | `123ms`               | `123ms 456us`          |
| `1.123456s`  | `1.12s`              | `1s`                  | `1s 123ms`             |
| `123s`       | `2.05m`              | `2m`                  | `2m 3s`                |
| `3660s`      | `1.01h`              | `1h`                  | `1h 1m`                |
| `777600s`    | `1.28w`              | `1w`                  | `1w 2d`                |
| `12345689s`  | `4.69mo`             | `4mo`                 | `4mo 21d`              |

## Minimum Unit

Use [`Duration::with_min_unit`](https://docs.rs/folktime/latest/folktime/duration/struct.Duration.html#method.with_min_unit) to set a floor on the displayed unit:

```rust
use core::time::Duration;
use folktime::Folktime;
use folktime::duration::{Style, Unit};

let a = Folktime::duration(Duration::from_millis(500));
let b = a.with_min_unit(Unit::Second);
let c = a.with_style(Style::TwoUnitsWhole).with_min_unit(Unit::Second);

assert_eq!(format!("{a}"), "500ms");
assert_eq!(format!("{b}"), "0.50s");
assert_eq!(format!("{c}"), "0s 500ms");
```

## Notes

- All styles support the full range of `core::time::Duration`.
- Month and year-based units use fixed Julian durations (365.25 days/year).
