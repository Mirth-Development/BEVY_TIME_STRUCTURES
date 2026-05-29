
// Imports
use bevy::prelude::*;

// Constants
const TICKER_MIN_VALUE: i8 = -100;
const TICKER_MAX_VALUE: i8 = 100;
const COUNTDOWN_MIN_VALUE: i8 = 1;
const COUNTDOWN_MAX_VALUE: i8 = TICKER_MAX_VALUE;
const LOOP_POINT: i8 = 101;

#[derive(Reflect, PartialEq, Debug)]
pub enum TickerStates {
    Ticking,
    Paused,
}

/// By themselves, tickers can be used to create simple timers.  Although they are best used in conjunction
/// as an inner element to a greater time structure to create some wicked tickety-tocking.
///
/// All fields of Ticker have getters, and only digit has no setter.
///
/// # TICKING LOOPS AT [`LOOP_POINT`]
/// Tickers don't stop ticking.  Once the next tick addition hits [`LOOP_POINT`] it will zero out current_value using to_zero().
/// **This is crucial to understand.** Not recognizing that ticking loops on these structures will make for poor usage of them.
/// Tickers are a building block element to making larger time structures or for highly compartmentalized timer usage.
/// **If you're okay with values from [`TICKER_MIN_VALUE`] to [`TICKER_MAX_VALUE`] for your timers, then feel free to go ham with Tickers.**
/// Otherwise, I recommend the Chronolog structure.
#[derive(Component, Reflect, Debug)]
pub struct Ticker {
    start_value: i8,
    current_value: i8,
    digit: i8,          // This is i8 and not u8 to avoid unnecessary typecasting inside the tick method.
    timer: Timer,
    state: TickerStates,
}

impl Default for Ticker {

    /// The default ticker counts up every second when its .tick method is used and all other fields start at 0.
    fn default() -> Self {
        Self {
            start_value:    0,
            current_value:  0,
            digit:          0,
            timer:          Timer::from_seconds(1.0, TimerMode::Repeating),
            state:          TickerStates::Ticking,
        }
    }
}

impl Ticker {

    /// Develops a new Ticker using a passed value for its start_value.
    ///
    /// Valid start values are [`TICKER_MIN_VALUE`] to [`TICKER_MAX_VALUE`] (inclusive).
    /// **Values outside this range will cause a panic.**
    ///
    /// When a second passes, the timer within the Ticker fires (increases current_value by 1 for each second that passes).
    pub fn new(starting_value: i8) -> Self {

        // Panic Evaluation
        check_value(starting_value, TICKER_MIN_VALUE, TICKER_MAX_VALUE);

        Self {
            start_value:    starting_value,
            current_value:  starting_value,
            digit:          starting_value.abs() % 10,
            timer:          Timer::from_seconds(1.0, TimerMode::Repeating),
            state:          TickerStates::Ticking,
        }
    }

    /// Develops a new Ticker using a passed value for its start_value.
    ///
    /// Valid start values are [`TICKER_MIN_VALUE`] to [`TICKER_MAX_VALUE`] (inclusive).
    /// **Values outside this range will cause a panic.**
    ///
    /// When the passed duration in second(s) passes, the timer within the Ticker fires (increases current_value by 1 for each duration that passes).
    pub fn new_with_duration(starting_value: i8, duration: f32) -> Self {

        // Panic Evaluation
        check_value(starting_value, TICKER_MIN_VALUE, TICKER_MAX_VALUE);

        Self {
            start_value:    starting_value,
            current_value:  starting_value,
            digit:          starting_value.abs() % 10,
            timer:          Timer::from_seconds(duration, TimerMode::Repeating),
            state:          TickerStates::Ticking,
        }
    }

    /// Creates a Ticker for countdown purposes.  Pass in the desired countdown duration as a number of seconds to pass.
    ///
    /// Valid countdown durations are [`COUNTDOWN_MIN_VALUE`] to [`COUNTDOWN_MAX_VALUE`] (pass 10 in for a 10-second countdown); inclusive range.
    /// **Values outside this range will cause a panic.**
    ///
    /// The start_value for Tickers that use this constructor is calculated by ([`LOOP_POINT`] - DURATION).
    pub fn new_with_countdown(duration: i8) -> Self {

        // Panic Evaluation
        check_value(duration, COUNTDOWN_MIN_VALUE, COUNTDOWN_MAX_VALUE);

        let starting_value = LOOP_POINT - duration;
        Self {
            start_value:    starting_value,
            current_value:  starting_value,
            digit:          starting_value.abs() % 10,
            timer:          Timer::from_seconds(1.0, TimerMode::Repeating),
            state:          TickerStates::Ticking,
        }
    }

    /// Returns the current_value of a Ticker.
    pub fn get_current_value(&self) -> i8 {
        self.current_value
    }

    /// Returns the start_value of a Ticker.
    ///
    /// It's important to note that start_value can change through set_start_value(), so don't
    /// treat it as a consistent value.
    pub fn get_start_value(&self) -> i8 {
        self.start_value
    }

    /// Returns the difference between current_value and start_value, which does mean that negative values
    /// are a possibility for the result.
    ///
    /// A negative result indicates that the current_value is less than the start_value of the Ticker.
    ///
    /// A positive result indicates that the current_value is greater than the start_value of the Ticker.
    ///
    /// RESULT = CURRENT_VALUE - START_VALUE
    pub fn get_distance_from_start(&self) -> i16 {
        self.current_value as i16 - self.start_value as i16
    }

    /// Will return 0 once the countdown is complete, otherwise returns the number of seconds remaining.
    ///
    /// # WARNING
    /// Not advised to use with Tickers that were **NOT** made with new_with_countdown.  Use get_countdown_value
    /// at your own risk for non-countdown Tickers.
    pub fn get_countdown_value(&self) -> i8 {

        if self.current_value <= 0 {
            0
        }
        else {
            LOOP_POINT - self.current_value
        }
    }

    /// Returns the digit in the ones-place of the current_value.
    pub fn get_digit(&self) -> i8 {
        self.digit
    }

    /// Will return the Bevy timer being used in the Ticker.
    ///
    /// To my knowledge, this method is for the most part useless since Tickers are only assigned
    /// to repeating Bevy timers that use from_second with a value of 1.0.  BUT, in the case that I'm
    /// wrong, this method is around for anybody that needs to get the Timer inside a Ticker.
    pub fn get_timer(&self) -> &Timer {
        &self.timer
    }

    /// Returns the active state that the Ticker is in.
    pub fn get_state(&self) -> &TickerStates {
        &self.state
    }

    /// Allows manipulation of the current_value.  Passed value must be within acceptable range, if not a panic will occur.
    ///
    /// Both start_value and current_value have setters to allow for time manipulation shenanigans.  If an
    /// event were to occur and someone wanted to drastically alter how time worked then they can use the
    /// setters to make some interesting mechanics.
    pub fn set_current_value(&mut self, value: i8) {

        // Panic Evaluation
        check_value(value, TICKER_MIN_VALUE, TICKER_MAX_VALUE);

        self.current_value = value;
    }

    /// Allows manipulation of the start_value.  Passed value must be within acceptable range, if not a panic will occur.
    ///
    /// Both start_value and current_value have setters to allow for time manipulation shenanigans.  If an
    /// event were to occur and someone wanted to drastically alter how time worked then they can use the
    /// setters to make some interesting mechanics.
    pub fn set_start_value(&mut self, value: i8) {

        // Panic Evaluation
        check_value(value, TICKER_MIN_VALUE, TICKER_MAX_VALUE);

        self.start_value = value;
    }

    /// Pauses a timer within the ticker.
    pub fn pause(&mut self) {
        self.timer.pause();
        self.state = TickerStates::Paused;
    }

    /// Unpauses a timer within a ticker.
    pub fn unpause(&mut self) {
        self.timer.unpause();
        self.state = TickerStates::Ticking;
    }

    /// Will set the current_value to be equal to the start_value and the digit field of the Ticker
    /// will be changed according to the new ones-place value that is seen after current_value's reset.
    ///
    /// Digit is always to reflect current_value's ones-place.
    pub fn reset(&mut self) {
        self.current_value = self.start_value;
        self.digit = self.current_value.abs() % 10;
    }

    /// Adds to the start_value of the ticker by the passed value.  Can take in negatives for subtraction.
    ///
    /// Will not let the result of summing cause overflow or wrapping; results will always be within [`TICKER_MIN_VALUE`] to [`TICKER_MAX_VALUE`] (inclusive).
    pub fn add_to_start(&mut self, value: i8) {
        self.start_value = self.start_value.saturating_add(value).clamp(TICKER_MIN_VALUE, TICKER_MAX_VALUE);
    }

    /// Adds to the current_value of the ticker by the passed value.  Can take in negatives for subtraction.
    ///
    /// Will not let the result of summing cause overflow or wrapping; results will always be within [`TICKER_MIN_VALUE`] to [`TICKER_MAX_VALUE`] (inclusive).
    pub fn add_to_current(&mut self, value: i8) {
        self.current_value = self.current_value.saturating_add(value).clamp(TICKER_MIN_VALUE, TICKER_MAX_VALUE);
    }

    /// Returns true if the current_value of the Ticker is below its start_value, false otherwise.
    pub fn current_is_below_start(&self) -> bool {
        self.current_value < self.start_value
    }

    /// Returns true if the current_value of the Ticker is above its start_value, false otherwise.
    pub fn current_is_above_start(&self) -> bool {
        self.current_value > self.start_value
    }

    /// Returns true if the current_value and the start_value are equal to one another, false otherwise.
    ///
    /// When relying solely on frames, I think this would be rather difficult to trigger.  However,
    /// using the reset method and setters may allow for this to return true often depending on
    /// how said methods are used.
    pub fn current_is_equal_to_start(&self) -> bool {
        self.current_value == self.start_value
    }

    /// Will make the current_value and digit be set to zero.
    ///
    /// Zero is a special number which is why it gets its own method.  Never let anybody tell you that
    /// zero isn't special - it's the almighty equalizer, destroyer, and splitter.
    pub fn zero_out(&mut self) {
        self.current_value = 0;
        self.digit = 0;
    }

    /// Sets current_value to its minimum value (will alter the digit field to reflect this change).
    pub fn current_to_min(&mut self) {
        self.current_value = TICKER_MIN_VALUE;
        self.digit = self.current_value.abs() % 10;
    }

    /// Sets current_value to its maximum value (will alter the digit field to reflect this change).
    pub fn current_to_max(&mut self) {
        self.current_value = TICKER_MAX_VALUE;
        self.digit = self.current_value.abs() % 10;
    }

    /// Used to advance a ticker.  Takes in a time.delta() call off the time resource (Res<Time>) that Bevy provides.
    ///
    /// If you're making a custom ticking system and have stripped out the ticking systems provided
    /// in the systems of this plugin, then please note that you must run this each frame for time to move normally.
    ///
    /// # TICKING LOOPS AT [`LOOP_POINT`]
    /// Tickers don't stop ticking.  Once the next tick addition hits [`LOOP_POINT`] it will zero out current_value.
    /// **This is crucial to understand.** Not recognizing that ticking loops on these structures will make for poor usage of them.
    /// Tickers are a building block element to making larger time structures or for highly compartmentalized timer usage.
    /// **If you're okay with values from [`TICKER_MIN_VALUE`] to [`TICKER_MAX_VALUE`] for your timers, then feel free to go ham with Tickers.**
    /// Otherwise, I recommend the Chronolog structure.
    pub fn tick(&mut self, delta: std::time::Duration) {

        // Will only tick a Ticker if its set to the Ticking state.
        if self.state == TickerStates::Ticking {

            self.timer.tick(delta);
            let ticks: u32 = self.timer.times_finished_this_tick();

            if ticks > 0 {

                let new_ticks: i8 = ticks as i8;

                // Saturating add is present in case the amount of ticks received could cause for the addition
                // on current_value to go beyond the i8::MAX.
                if self.current_value.saturating_add(new_ticks) >= LOOP_POINT {
                    self.zero_out();
                }
                else {
                    self.current_value = self.current_value.saturating_add(new_ticks);
                    self.digit = self.current_value.abs() % 10;
                }
            }
        }
    }
}

/// Determines if the value is within the acceptable range that is passed in.  Will cause a panic if the value is out of the range.
fn check_value(value: i8, minimum: i8, maximum: i8) {
    assert!(value >= minimum && value <= maximum, "TICKER PANIC: Value must be between {} and {} (inclusive). Got {}.", minimum, maximum, value);
}
