use core::fmt::Write;
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use folktime::duration::{Style, Unit};
use folktime::Folktime;

// -- Counting allocator --

static ALLOC_COUNT: AtomicUsize = AtomicUsize::new(0);

struct CountingAllocator;

unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOC_COUNT.fetch_add(1, Ordering::SeqCst);
        unsafe { System.alloc(layout) }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }
}

#[global_allocator]
static A: CountingAllocator = CountingAllocator;

// -- Stack buffer that implements fmt::Write without allocating --

struct StackBuf {
    buf: [u8; 64],
    pos: usize,
}

impl StackBuf {
    fn new() -> Self {
        Self {
            buf: [0; 64],
            pos: 0,
        }
    }
}

impl Write for StackBuf {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let bytes = s.as_bytes();
        let end = self.pos + bytes.len();
        if end > self.buf.len() {
            return Err(core::fmt::Error);
        }
        self.buf[self.pos..end].copy_from_slice(bytes);
        self.pos = end;
        Ok(())
    }
}

// -- Helper --

fn assert_no_alloc(f: impl FnOnce(&mut StackBuf)) {
    let mut buf = StackBuf::new();
    let before = ALLOC_COUNT.load(Ordering::SeqCst);
    f(&mut buf);
    let after = ALLOC_COUNT.load(Ordering::SeqCst);
    assert_eq!(before, after, "unexpected allocation during formatting");
}

// -- Tests --

#[test]
fn one_unit_frac_does_not_allocate() {
    assert_no_alloc(|buf| write!(buf, "{}", Folktime::duration(Duration::ZERO)).unwrap());
    assert_no_alloc(|buf| write!(buf, "{}", Folktime::duration(Duration::from_nanos(1))).unwrap());
    assert_no_alloc(|buf| write!(buf, "{}", Folktime::duration(Duration::from_nanos(999))).unwrap());
    assert_no_alloc(|buf| write!(buf, "{}", Folktime::duration(Duration::from_micros(500))).unwrap());
    assert_no_alloc(|buf| write!(buf, "{}", Folktime::duration(Duration::from_millis(123))).unwrap());
    assert_no_alloc(|buf| write!(buf, "{}", Folktime::duration(Duration::from_secs(5))).unwrap());
    assert_no_alloc(|buf| write!(buf, "{}", Folktime::duration(Duration::from_secs(123))).unwrap());
    assert_no_alloc(|buf| write!(buf, "{}", Folktime::duration(Duration::from_secs(3_600))).unwrap());
    assert_no_alloc(|buf| write!(buf, "{}", Folktime::duration(Duration::from_secs(86_400))).unwrap());
    assert_no_alloc(|buf| write!(buf, "{}", Folktime::duration(Duration::from_secs(604_800))).unwrap());
    assert_no_alloc(|buf| write!(buf, "{}", Folktime::duration(Duration::from_secs(31_558_150))).unwrap());
    assert_no_alloc(|buf| write!(buf, "{}", Folktime::duration(Duration::new(u64::MAX, 999_999_999))).unwrap());
}

#[test]
fn one_unit_whole_does_not_allocate() {
    assert_no_alloc(|buf| write!(buf, "{}", Folktime::duration(Duration::ZERO).with_style(Style::OneUnitWhole)).unwrap());
    assert_no_alloc(|buf| write!(buf, "{}", Folktime::duration(Duration::from_nanos(42)).with_style(Style::OneUnitWhole)).unwrap());
    assert_no_alloc(|buf| write!(buf, "{}", Folktime::duration(Duration::from_millis(500)).with_style(Style::OneUnitWhole)).unwrap());
    assert_no_alloc(|buf| write!(buf, "{}", Folktime::duration(Duration::from_secs(59)).with_style(Style::OneUnitWhole)).unwrap());
    assert_no_alloc(|buf| write!(buf, "{}", Folktime::duration(Duration::from_secs(7_200)).with_style(Style::OneUnitWhole)).unwrap());
}

#[test]
fn two_units_whole_does_not_allocate() {
    assert_no_alloc(|buf| write!(buf, "{}", Folktime::duration(Duration::ZERO).with_style(Style::TwoUnitsWhole)).unwrap());
    assert_no_alloc(|buf| write!(buf, "{}", Folktime::duration(Duration::from_nanos(42)).with_style(Style::TwoUnitsWhole)).unwrap());
    assert_no_alloc(|buf| write!(buf, "{}", Folktime::duration(Duration::from_millis(500)).with_style(Style::TwoUnitsWhole)).unwrap());
    assert_no_alloc(|buf| write!(buf, "{}", Folktime::duration(Duration::from_secs(123)).with_style(Style::TwoUnitsWhole)).unwrap());
    assert_no_alloc(|buf| write!(buf, "{}", Folktime::duration(Duration::from_secs(90_061)).with_style(Style::TwoUnitsWhole)).unwrap());
}

#[test]
fn with_min_unit_does_not_allocate() {
    assert_no_alloc(|buf| {
        let d = Folktime::duration(Duration::from_millis(500)).with_min_unit(Unit::Second);
        write!(buf, "{}", d).unwrap();
    });
}
