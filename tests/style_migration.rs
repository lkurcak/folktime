use core::time::Duration;

use folktime::Folktime;
use folktime::duration::Style;

#[allow(deprecated)]
#[test]
fn deprecated_style_names_match_new_styles() {
    let d = Duration::from_secs(123);

    assert_eq!(
        format!("{}", Folktime::duration(d).with_style(Style::OneUnitFrac)),
        format!("{}", Folktime::duration(d).with_style(Style::Compact))
    );
    assert_eq!(
        format!("{}", Folktime::duration(d).with_style(Style::OneUnitWhole)),
        format!("{}", Folktime::duration(d).with_style(Style::Whole))
    );
    assert_eq!(
        format!("{}", Folktime::duration(d).with_style(Style::TwoUnitsWhole)),
        format!("{}", Folktime::duration(d).with_style(Style::Detailed))
    );
}
