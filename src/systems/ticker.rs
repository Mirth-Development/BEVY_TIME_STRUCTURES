
use bevy::prelude::*;
use crate::types::Ticker;

/// Will loop through queried tickers to initiate their ticking.
///
/// Also, seeing as how Tickers may be used in many potential future time structures, I'd like to
/// mention something important.  The Ticker and Chronolog are two different components, which means
/// we can not rely solely on this system to tick the tickers inside the Chronologs as well.
/// Tickers embedded inside a Chronolog are fields on the Chronolog struct, not separate ECS components.
/// This means the ticker_ticking system will never see them, it only queries for Ticker components
/// attached directly to entities. The chronolog_ticking system is responsible for ticking those internal
/// tickers separately.
///
/// I believe Bevy may be forced to do this due to how Rust interprets struct fields.  Which might be a
/// good thing since it allows for multiple implementations to be possible for a singular behavior
/// depending on the context that the behavior is operating within.
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TimeStructures {})
        .add_systems(Startup, (
            test_default,
            test_new,
            test_new_with_duration,
            test_new_with_countdown,
            test_getters,
            test_set_current_value,
            test_set_start_value,
            test_add_to_start,
            test_add_to_current,
            test_reset,
            test_zero_out,
            test_current_to_min,
            test_current_to_max,
            test_comparisons,
            test_get_distance_from_start,
            test_get_countdown_value,
            test_pause_unpause,
            test_tick_loop,
            test_new_panic_guard,
            test_countdown_panic_guard,
        ).chain())
        .run();
}

// ─── Helpers ────────────────────────────────────────────────────────────────

fn pass(test: &str) {
    println!("[PASS] {}", test);
}

fn fail(test: &str, reason: &str) {
    println!("[FAIL] {} — {}", test, reason);
}

fn check(test: &str, condition: bool, reason: &str) {
    if condition { pass(test); } else { fail(test, reason); }
}

// ─── Safety Note ─────────────────────────────────────────────────────────────
//
// ticker_ticking runs every frame and queries for Ticker components attached to
// entities via Query<&mut Ticker>.  Every test below constructs Tickers as plain
// local stack values — they are never spawned into the ECS — so ticker_ticking
// will never see or mutate them.  Tests that need to verify tick() behaviour call
// it directly with a known std::time::Duration instead of relying on real elapsed
// time, keeping results fully deterministic regardless of frame timing.
//
// ─────────────────────────────────────────────────────────────────────────────

fn test_default() {
    let t = Ticker::default();
    check("default::current_value is 0",  t.get_current_value() == 0,  "expected 0");
    check("default::start_value is 0",    t.get_start_value()   == 0,  "expected 0");
    check("default::digit is 0",          t.get_digit()         == 0,  "expected 0");
}

fn test_new() {
    let t = Ticker::new(42);
    check("new::current_value matches start", t.get_current_value() == 42, "expected 42");
    check("new::start_value set correctly",   t.get_start_value()   == 42, "expected 42");
    check("new::digit is ones-place of 42",   t.get_digit()         == 2,  "expected 2");

    let t_neg = Ticker::new(-37);
    check("new::negative start_value",        t_neg.get_current_value() == -37, "expected -37");
    check("new::digit of -37 is 7 (abs %10)", t_neg.get_digit()         == 7,   "expected 7");

    let t_zero = Ticker::new(0);
    check("new::zero start_value",            t_zero.get_current_value() == 0, "expected 0");
}

fn test_new_with_duration() {
    let t = Ticker::new_with_duration(10, 0.5);
    check("new_with_duration::current_value", t.get_current_value() == 10, "expected 10");
    check("new_with_duration::start_value",   t.get_start_value()   == 10, "expected 10");
    check("new_with_duration::digit",         t.get_digit()         == 0,  "expected 0");
    // Timer duration cannot be directly inspected without ticking, so we just
    // confirm construction succeeds and values are correct.
    pass("new_with_duration::constructed without panic");
}

fn test_new_with_countdown() {
    // LOOP_POINT = 101; duration = 10 => start_value = 91
    let t = Ticker::new_with_countdown(10);
    check("new_with_countdown::start_value is 101-10=91", t.get_start_value()   == 91, "expected 91");
    check("new_with_countdown::current_value is 91",      t.get_current_value() == 91, "expected 91");
    check("new_with_countdown::digit is 1 (91 % 10)",     t.get_digit()         == 1,  "expected 1");

    // countdown of 1 => start_value = 100
    let t2 = Ticker::new_with_countdown(1);
    check("new_with_countdown::min duration start_value=100", t2.get_start_value() == 100, "expected 100");
}

fn test_getters() {
    let t = Ticker::new(55);
    check("get_current_value", t.get_current_value() == 55, "expected 55");
    check("get_start_value",   t.get_start_value()   == 55, "expected 55");
    check("get_digit",         t.get_digit()         == 5,  "expected 5");
    // get_timer: just confirm it returns without panic
    let _ = t.get_timer();
    pass("get_timer::returns reference without panic");
}

fn test_set_current_value() {
    let mut t = Ticker::new(0);
    t.set_current_value(77);
    check("set_current_value::value updated", t.get_current_value() == 77, "expected 77");

    t.set_current_value(-50);
    check("set_current_value::negative value", t.get_current_value() == -50, "expected -50");

    t.set_current_value(100);
    check("set_current_value::max boundary", t.get_current_value() == 100, "expected 100");

    t.set_current_value(-100);
    check("set_current_value::min boundary", t.get_current_value() == -100, "expected -100");
}

fn test_set_start_value() {
    let mut t = Ticker::new(0);
    t.set_start_value(33);
    check("set_start_value::value updated", t.get_start_value() == 33, "expected 33");

    t.set_start_value(-33);
    check("set_start_value::negative value", t.get_start_value() == -33, "expected -33");
}

fn test_add_to_start() {
    let mut t = Ticker::new(50);
    t.add_to_start(10);
    check("add_to_start::positive addition", t.get_start_value() == 60, "expected 60");

    t.add_to_start(-20);
    check("add_to_start::negative addition (subtraction)", t.get_start_value() == 40, "expected 40");

    // Clamp at max (100)
    t.add_to_start(100);
    check("add_to_start::clamps at TICKER_MAX_VALUE (100)", t.get_start_value() == 100, "expected 100");

    // Clamp at min (-100)
    t.add_to_start(-127);
    check("add_to_start::clamps at TICKER_MIN_VALUE (-100)", t.get_start_value() == -100, "expected -100");
}

fn test_add_to_current() {
    let mut t = Ticker::new(0);
    t.add_to_current(25);
    check("add_to_current::positive addition", t.get_current_value() == 25, "expected 25");

    t.add_to_current(-10);
    check("add_to_current::negative addition", t.get_current_value() == 15, "expected 15");

    // Clamp at max
    t.add_to_current(100);
    check("add_to_current::clamps at max (100)", t.get_current_value() == 100, "expected 100");

    // Clamp at min
    t.add_to_current(-127);
    check("add_to_current::clamps at min (-100)", t.get_current_value() == -100, "expected -100");
}

fn test_reset() {
    let mut t = Ticker::new(20);
    t.set_current_value(80);
    t.reset();
    check("reset::current_value returns to start_value", t.get_current_value() == 20, "expected 20");
    check("reset::digit reflects start_value ones-place",  t.get_digit()         == 0,  "expected 0");

    // Reset on negative start
    let mut t2 = Ticker::new(-37);
    t2.set_current_value(0);
    t2.reset();
    check("reset::negative start restored", t2.get_current_value() == -37, "expected -37");
    check("reset::digit of -37 is 7",       t2.get_digit()         == 7,   "expected 7");
}

fn test_zero_out() {
    let mut t = Ticker::new(99);
    t.zero_out();
    check("zero_out::current_value is 0", t.get_current_value() == 0, "expected 0");
    check("zero_out::digit is 0",         t.get_digit()         == 0, "expected 0");
    // start_value should be unaffected
    check("zero_out::start_value unchanged", t.get_start_value() == 99, "expected 99");
}

fn test_current_to_min() {
    let mut t = Ticker::new(50);
    t.current_to_min();
    check("current_to_min::current_value is -100", t.get_current_value() == -100, "expected -100");
    check("current_to_min::digit is 0 (100 % 10)", t.get_digit()         == 0,    "expected 0");
}

fn test_current_to_max() {
    let mut t = Ticker::new(0);
    t.current_to_max();
    check("current_to_max::current_value is 100", t.get_current_value() == 100, "expected 100");
    check("current_to_max::digit is 0 (100 % 10)", t.get_digit()        == 0,   "expected 0");
}

fn test_comparisons() {
    let mut t = Ticker::new(50);

    // current == start (both are 50)
    check("current_is_equal_to_start::true when equal",       t.current_is_equal_to_start(), "expected true");
    check("current_is_below_start::false when equal",         !t.current_is_below_start(),   "expected false");
    check("current_is_above_start::false when equal",         !t.current_is_above_start(),   "expected false");

    // current below start
    t.set_current_value(10);
    check("current_is_below_start::true when below",          t.current_is_below_start(),    "expected true");
    check("current_is_above_start::false when below",         !t.current_is_above_start(),   "expected false");
    check("current_is_equal_to_start::false when below",      !t.current_is_equal_to_start(),"expected false");

    // current above start
    t.set_current_value(75);
    check("current_is_above_start::true when above",          t.current_is_above_start(),    "expected true");
    check("current_is_below_start::false when above",         !t.current_is_below_start(),   "expected false");
    check("current_is_equal_to_start::false when above",      !t.current_is_equal_to_start(),"expected false");
}

fn test_get_distance_from_start() {
    let mut t = Ticker::new(50);
    check("get_distance_from_start::zero when equal", t.get_distance_from_start() == 0, "expected 0");

    t.set_current_value(80);
    check("get_distance_from_start::positive when above start", t.get_distance_from_start() == 30, "expected 30");

    t.set_current_value(20);
    check("get_distance_from_start::negative when below start", t.get_distance_from_start() == -30, "expected -30");

    // Boundary: start=-100, current=100; distance=200 (fits in i16)
    let mut t2 = Ticker::new(-100);
    t2.set_current_value(100);
    check("get_distance_from_start::large positive distance", t2.get_distance_from_start() == 200, "expected 200");
}

fn test_get_countdown_value() {
    // Made with new_with_countdown(10) => start=91, current=91
    let t = Ticker::new_with_countdown(10);
    check("get_countdown_value::full countdown remaining is 10", t.get_countdown_value() == 10, "expected 10");

    // Simulate partway through: current = 96 => 101-96 = 5 remaining
    let mut t2 = Ticker::new_with_countdown(10);
    t2.set_current_value(96);
    check("get_countdown_value::5 seconds remaining", t2.get_countdown_value() == 5, "expected 5");

    // Countdown complete: current >= LOOP_POINT is impossible, but current=0 or negative => returns 0
    let mut t3 = Ticker::new_with_countdown(10);
    t3.set_current_value(0);
    check("get_countdown_value::returns 0 when current_value is 0", t3.get_countdown_value() == 0, "expected 0");

    let mut t4 = Ticker::new_with_countdown(10);
    t4.set_current_value(-5);
    check("get_countdown_value::returns 0 when current_value is negative", t4.get_countdown_value() == 0, "expected 0");
}

fn test_pause_unpause() {
    let mut t = Ticker::default();

    t.pause();
    check("pause::timer is paused after pause()", t.get_timer().paused(), "expected paused == true");

    t.unpause();
    check("unpause::timer not paused after unpause()", !t.get_timer().paused(), "expected paused == false");
}

fn test_tick_loop() {
    // We can't rely on real elapsed time in a Startup system, so we test
    // tick() by feeding a duration that is guaranteed to fire the timer.
    // Default timer = 1.0s repeating; passing 1 second of duration fires it once.
    let one_second = std::time::Duration::from_secs(1);

    let mut t = Ticker::new(0);
    t.tick(one_second);
    check("tick::increments current_value by 1 after one second", t.get_current_value() == 1, "expected 1");
    check("tick::digit updated after tick",                        t.get_digit()         == 1, "expected 1");

    // Digit rollover: 9 -> 10, digit should become 0
    let mut t2 = Ticker::new(9);
    t2.tick(one_second);
    check("tick::digit rolls over 9->0 at tens boundary", t2.get_digit() == 0, "expected 0");

    // Loop: current = 100, one more tick should hit LOOP_POINT (101) and zero out
    let mut t3 = Ticker::new(100);
    t3.tick(one_second);
    check("tick::loops back to 0 at LOOP_POINT", t3.get_current_value() == 0, "expected 0");
    check("tick::digit is 0 after loop",         t3.get_digit()         == 0, "expected 0");

    // Paused ticker should NOT advance
    let mut t4 = Ticker::new(5);
    t4.pause();
    t4.tick(one_second);
    check("tick::paused ticker does not advance", t4.get_current_value() == 5, "expected 5");
}

/// Confirms new() panics on out-of-range values using std::panic::catch_unwind.
fn test_new_panic_guard() {
    let over  = std::panic::catch_unwind(|| Ticker::new(101));
    let under = std::panic::catch_unwind(|| Ticker::new(-101));
    check("new::panics on value > 100",  over.is_err(),  "expected panic");
    check("new::panics on value < -100", under.is_err(), "expected panic");
}

/// Confirms new_with_countdown() panics on out-of-range durations.
fn test_countdown_panic_guard() {
    let over = std::panic::catch_unwind(|| Ticker::new_with_countdown(101));
    let under = std::panic::catch_unwind(|| Ticker::new_with_countdown(0));
    check("new_with_countdown::panics on duration > 100", over.is_err(), "expected panic");
    check("new_with_countdown::panics on duration < 1", under.is_err(), "expected panic");
}
