
// Imports
use std::fmt::Display;
use std::ops::{Add, Sub};
use bevy::prelude::*;
use mirth_engine_testing_tools::{TestColors};
use num_traits::{SaturatingAdd, SaturatingSub, Signed};

pub trait TickerType:
Copy
+ Ord
+ Display
+ SaturatingAdd
+ SaturatingSub
+ Signed
+ Add<Output = Self>
+ Sub<Output = Self>
{
    fn as_f32(self)         -> f32;
    fn as_i8(self)          -> i8;
    fn as_i64(self)         -> i64;
    fn from_f32(value: f32) -> Self;
    fn from_i32(val: i32)   -> Self;

}

impl TickerType for i8 {
    fn as_f32(self)         -> f32  { self as f32 }
    fn as_i8(self)          -> i8   { self }
    fn as_i64(self)         -> i64  { self as i64 }
    fn from_f32(value: f32) -> Self { value as i8 }
    fn from_i32(value: i32) -> Self { value as i8 }
}

impl TickerType for i16 {
    fn as_f32(self)         -> f32  { self as f32 }
    fn as_i8(self)          -> i8   { self as i8}
    fn as_i64(self)         -> i64  { self as i64 }
    fn from_f32(value: f32) -> Self { value as i16 }
    fn from_i32(value: i32) -> Self { value as i16 }
}

impl TickerType for i32 {
    fn as_f32(self)         -> f32  { self as f32 }
    fn as_i8(self)          -> i8   { self as i8 }
    fn as_i64(self)         -> i64  { self as i64 }
    fn from_f32(value: f32) -> Self { value as i32 }
    fn from_i32(value: i32) -> Self { value }
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
/// **If you're okay with values from [`i8::MIN`] to [`TICKER_MAX_VALUE`] for your timers, then feel free to go ham with Tickers.**
/// Otherwise, I recommend the Chronolog structure.
#[derive(Component, Reflect, Debug)]
pub struct Ticker<T: TickerType> {
    start_value:                T,
    current_value:              T,
    end_value:                  T,
    interval:                   f32,
    accrued_delta:              f32,
    is_paused:                  bool,
    is_looping:                 bool,
    is_ticking_up:              bool,
    is_handling_frame_spikes:   bool,
}

impl<T: TickerType> Default for Ticker<T> {
    fn default() -> Self {
        Self {
            start_value:                T::from_i32(0),
            current_value:              T::from_i32(0),
            end_value:                  T::from_i32(100),
            interval:                   1.0,
            accrued_delta:              0.0,
            is_paused:                  false,
            is_looping:                 true,
            is_ticking_up:              true,
            is_handling_frame_spikes:   true,
        }
    }
}

impl<T: TickerType> Ticker<T> {

    // ##################################### CONSTRUCTORS ######################################## //
    ///
    pub fn new(
        start_value: T,
        current_value: T,
        end_value: T,
        interval: f32,
        is_paused: bool,
        is_looping: bool,
        is_ticking_up: bool,
        is_handling_frame_spikes: bool,
    ) -> Self {

        let min = start_value.min(end_value);
        let max = start_value.max(end_value);

        if current_value < min || current_value > max {
            panic!(
                "[{}CURRENT_VALUE NOT IN RANGE{}] current_value must be between {} and {}.  It is currently being constructed with {}.",
                TestColors::FAIL, TestColors::DEFAULT, min, max, current_value
            );
        }

        Self {
            start_value,
            current_value,
            end_value,
            interval,
            accrued_delta: 0.0,
            is_paused,
            is_looping,
            is_ticking_up,
            is_handling_frame_spikes,
        }
    }

    ///
    pub fn new_onetime_with_frame_spike_handling(
        starting_value: T,
        end_value: T,
        interval: f32,
        is_ticking_up: bool,
    ) -> Self {
        Self {
            start_value:                starting_value,
            current_value:              starting_value,
            end_value,
            interval,
            accrued_delta:              0.0,
            is_paused:                  false,
            is_looping:                 false,
            is_ticking_up,
            is_handling_frame_spikes:   true,
        }
    }

    ///
    pub fn new_onetime_without_frame_spike_handling(
        starting_value: T,
        end_value: T,
        interval: f32,
        is_ticking_up: bool,
    ) -> Self {
        Self {
            start_value:                starting_value,
            current_value:              starting_value,
            end_value,
            interval,
            accrued_delta:              0.0,
            is_paused:                  false,
            is_looping:                 false,
            is_ticking_up,
            is_handling_frame_spikes:   false,
        }
    }

    ///
    pub fn new_looper_with_frame_spike_handling(
        starting_value: T,
        end_value: T,
        interval: f32,
        is_ticking_up: bool,
    ) -> Self {
        Self {
            start_value:                starting_value,
            current_value:              starting_value,
            end_value,
            interval,
            accrued_delta:              0.0,
            is_paused:                  false,
            is_looping:                 true,
            is_ticking_up,
            is_handling_frame_spikes:   true,
        }
    }

    ///
    pub fn new_looper_without_frame_spike_handling(
        starting_value: T,
        end_value: T,
        interval: f32,
        is_ticking_up: bool,
    ) -> Self {
        Self {
            start_value:                starting_value,
            current_value:              starting_value,
            end_value,
            interval,
            accrued_delta:              0.0,
            is_paused:                  false,
            is_looping:                 true,
            is_ticking_up,
            is_handling_frame_spikes:   false,
        }
    }
    // ######################################################################################## //



    // ##################################### GETTERS ########################################## //
    /// Returns the start_value of a Ticker.
    ///
    /// start_value can change through other methods, so don't treat it as a consistent value.
    #[inline]
    pub fn start_value(&self) -> T { self.start_value }

    /// Returns the current_value of a Ticker.
    #[inline]
    pub fn current_value(&self) -> T { self.current_value }

    /// Returns the end_value of a Ticker.
    ///
    /// end_value can change through other methods, so don't treat it as a consistent value.
    #[inline]
    pub fn end_value(&self) -> T { self.end_value }

    /// Returns the interval of a Ticker.
    ///
    /// The interval value can change through other methods, so don't treat it as a consistent value.
    /// Also, it's important to remember that the interval is what dictates how long in seconds that it takes
    /// for current_value to increase or decrease; direction depends on is_ticking_up.
    #[inline]
    pub fn interval(&self) -> f32 { self.interval }

    ///
    #[inline]
    pub fn is_paused(&self) -> bool { self.is_paused }

    ///
    #[inline]
    pub fn is_looping(&self) -> bool { self.is_looping }

    ///
    #[inline]
    pub fn is_ticking_up(&self) -> bool { self.is_ticking_up }

    ///
    #[inline]
    pub fn is_handling_frame_spikes(&self) -> bool { self.is_handling_frame_spikes }
    // ######################################################################################## //



    // ##################################### SETTERS ########################################## //
    /// Changes `start_value` to the passed value.
    #[inline]
    pub fn set_start_value(&mut self, value: T) { self.start_value = value; }

    /// Changes `current_value` to the passed value.
    ///
    /// `current_value` can NOT go out of the range that `start_value` and `end_value` create.  Attempting
    /// to set `current_value` outside the range will cause it to be clamped down.
    pub fn set_current_value(&mut self, value: T) {
        let min = self.start_value.min(self.end_value);
        let max = self.start_value.max(self.end_value);
        self.current_value = value.clamp(min, max);
    }

    /// Changes `end_value` to the passed value.
    #[inline]
    pub fn set_end_value(&mut self, value: T) { self.end_value = value; }

    /// Pauses a ticker's ticking.
    ///
    /// This prevents the .tick method from doing any calculations.
    #[inline]
    pub fn pause(&mut self) { self.is_paused = true; }

    /// Unpauses a ticker's ticking.
    ///
    /// This allows the .tick method to resume its calculations.
    #[inline]
    pub fn unpause(&mut self) { self.is_paused = false; }

    ///
    #[inline]
    pub fn start_looping(&mut self) { self.is_looping = true; }

    ///
    #[inline]
    pub fn stop_looping(&mut self) { self.is_looping = false; }

    /// Causes the ticker's current_value to count up.
    ///
    /// Will allow calculated ticks inside the .tick method to add to current_value, rather than subtract.
    #[inline]
    pub fn tick_up(&mut self) { self.is_ticking_up = true; }

    /// Causes the ticker's current_value to count down.
    ///
    /// Will allow calculated ticks inside the .tick method to subtract from current_value, rather than add.
    #[inline]
    pub fn tick_down(&mut self) { self.is_ticking_up = false; }

    ///
    #[inline]
    pub fn start_handling_frame_spikes(&mut self) { self.is_handling_frame_spikes = true; }

    ///
    #[inline]
    pub fn stop_handling_frame_spikes(&mut self) {self.is_handling_frame_spikes = false; }
    // ######################################################################################## //



    // ################################### EQUAL METHODS ###################################### //
    /// Returns true if the current_value and the start_value are equal to one another, false otherwise.
    #[inline]
    pub fn is_current_equal_to_start(&self) -> bool { self.current_value == self.start_value }

    /// Returns true if the current_value and the end_value are equal to one another, false otherwise.
    #[inline]
    pub fn is_current_equal_to_end(&self) -> bool { self.current_value == self.end_value }

    /// Returns true if the start_value and the end_value are equal to one another, false otherwise.
    ///
    /// start_value and end_value can equal one another since their values can be changed or set to
    /// the same value at the creation of a Ticker instance.
    #[inline]
    pub fn is_start_equal_to_end(&self) -> bool { self.start_value == self.end_value }
    // ######################################################################################## //



    // ################################# DIFFERENCE METHODS ################################### //
    /// Returns the difference between current_value and start_value.
    ///
    /// Will only return positive numbers.
    pub fn difference_from_start(&self) -> i64 {
        let min: i64 = self.current_value.min(self.start_value).as_i64();
        let max: i64 = self.current_value.max(self.start_value).as_i64();
        max - min
    }

    /// Returns the difference between current_value and end_value.
    ///
    /// Will only return positive numbers.
    pub fn difference_from_end(&self) -> i64 {
        let min: i64 = self.current_value.min(self.end_value).as_i64();
        let max: i64 = self.current_value.max(self.end_value).as_i64();
        max - min
    }

    /// Returns the difference between start_value and end_value.
    ///
    /// Will only return positive numbers.
    pub fn difference_from_start_to_end(&self) -> i64 {
        let min: i64 = self.start_value.min(self.end_value).as_i64();
        let max: i64 = self.start_value.max(self.end_value).as_i64();
        max - min
    }
    // ######################################################################################## //



    // ################################# DIGIT METHODS ######################################## //
    /// Returns the digit in the ones-place of current_value.
    ///
    /// Will always return a positive value.
    ///
    /// #### Small Note
    /// It is not possible for the ones digit to be dropped, hence the reason why this method has no
    /// "with_drop_accounting" version - there is always a ones-place.
    ///
    /// #### No Conditional?
    /// This digit does not need to check current_value since all integer types can contain at least
    /// 3 digits.  0 will still be returned if the digit isn't being used.
    #[inline]
    pub fn digit_1(&self) -> i8 {
        (self.current_value.abs() % T::from_i32(10)).as_i8()
    }

    /// Returns the digit in the tens-place of current_value.
    ///
    /// Will always return a positive value.
    ///
    /// Will return a 0 if the digit doesn't exist.
    ///
    /// #### No Conditional?
    /// This digit does not need to check current_value since all integer types can contain at least
    /// 3 digits.  0 will still be returned if the digit isn't being used.
    #[inline]
    pub fn digit_2(&self) -> i8 {
        ((self.current_value.abs() / T::from_i32(10)) % T::from_i32(10)).as_i8()
    }

    /// Returns the digit in the hundreds-place of current_value.
    ///
    /// Will always return a positive value.
    ///
    /// Will return a 0 if the digit doesn't exist.
    ///
    /// #### No Conditional?
    /// This digit does not need to check current_value since all integer types can contain at least
    /// 3 digits.  0 will still be returned if the digit isn't being used.
    #[inline]
    pub fn digit_3(&self) -> i8 {
        ((self.current_value.abs() / T::from_i32(100)) % T::from_i32(10)).as_i8()
    }

    /// Returns the digit in the thousands-place of current_value.
    ///
    /// Will always return a positive value.
    ///
    /// Will return a 0 if the digit doesn't exist.
    #[inline]
    pub fn digit_4(&self) -> i8 {
        if self.current_value.abs() >= T::from_i32(1000) {
            ((self.current_value.abs() / T::from_i32(1000)) % T::from_i32(10)).as_i8()
        }
        else {
            0
        }
    }

    /// Returns the digit in the ten-thousands-place of current_value.
    ///
    /// Will always return a positive value.
    ///
    /// Will return a 0 if the digit doesn't exist.
    #[inline]
    pub fn digit_5(&self) -> i8 {
        if self.current_value.abs() >= T::from_i32(10000) {
            ((self.current_value.abs() / T::from_i32(10000)) % T::from_i32(10)).as_i8()
        }
        else {
            0
        }
    }

    /// Returns the digit in the hundred-thousands-place of current_value.
    ///
    /// Will always return a positive value.
    ///
    /// Will return a 0 if the digit doesn't exist.
    #[inline]
    pub fn digit_6(&self) -> i8 {
        if self.current_value.abs() >= T::from_i32(100000) {
            ((self.current_value.abs() / T::from_i32(100000)) % T::from_i32(10)).as_i8()
        }
        else {
            0
        }
    }

    /// Returns the digit in the millions-place of current_value.
    ///
    /// Will always return a positive value.
    ///
    /// Will return a 0 if the digit doesn't exist.
    #[inline]
    pub fn digit_7(&self) -> i8 {
        if self.current_value.abs() >= T::from_i32(1000000) {
            ((self.current_value.abs() / T::from_i32(1000000)) % T::from_i32(10)).as_i8()
        }
        else {
            0
        }
    }

    /// Returns the digit in the ten-millions-place of current_value.
    ///
    /// Will always return a positive value.
    ///
    /// Will return a 0 if the digit doesn't exist.
    #[inline]
    pub fn digit_8(&self) -> i8 {
        if self.current_value.abs() >= T::from_i32(10000000) {
            ((self.current_value.abs() / T::from_i32(10000000)) % T::from_i32(10)).as_i8()
        }
        else {
            0
        }
    }

    /// Returns the digit in the hundred-millions-place of current_value.
    ///
    /// Will always return a positive value.
    ///
    /// Will return a 0 if the digit doesn't exist.
    #[inline]
    pub fn digit_9(&self) -> i8 {
        if self.current_value.abs() >= T::from_i32(100000000) {
            ((self.current_value.abs() / T::from_i32(100000000)) % T::from_i32(10)).as_i8()
        }
        else {
            0
        }
    }

    /// Returns the digit in the tens-place of current_value.
    ///
    /// Will always return a positive value if the digit exists.
    ///
    /// Will return -1 if the digit is NOT being used.
    ///
    /// ### What is DDA?
    /// DDA stands for Digit Drop Accounting.  It can be used to determine if a digit has been dropped
    /// from a number or if a digit just happens to be 0.
    ///
    /// ### Example
    /// - If `current_value` is `6`, `digit_2_with_dda` returns `-1` — no tens-place exists.
    /// - If `current_value` is `63`, `digit_2_with_dda` returns `6` — the tens-place exists and is `6`.
    /// - If `current_value` is `103`, `digit_3_with_dda` returns `1` — the hundreds-place exists and is `1`.
    /// - If `current_value` is `1003`, `digit_3_with_dda` returns `0` — the hundreds-place exists but happens to be `0`.
    ///
    /// The `-1` sentinel allows you to differentiate between a digit that is absent and a digit that is simply `0`.
    #[inline]
    pub fn digit_2_with_dda(&self) -> i8 {
        if self.current_value.abs() >= T::from_i32(10) {
            ((self.current_value.abs() / T::from_i32(10)) % T::from_i32(10)).as_i8()
        }
        else {
            -1
        }
    }

    /// Returns the digit in the hundreds-place of current_value.
    ///
    /// Will always return a positive value if the digit exists.
    ///
    /// Will return -1 if the digit is NOT being used.
    ///
    /// ### What is DDA?
    /// DDA stands for Digit Drop Accounting.  It can be used to determine if a digit has been dropped
    /// from a number or if a digit just happens to be 0.
    ///
    /// ### Example
    /// - If `current_value` is `6`, `digit_2_with_dda` returns `-1` — no tens-place exists.
    /// - If `current_value` is `63`, `digit_2_with_dda` returns `6` — the tens-place exists and is `6`.
    /// - If `current_value` is `103`, `digit_3_with_dda` returns `1` — the hundreds-place exists and is `1`.
    /// - If `current_value` is `1003`, `digit_3_with_dda` returns `0` — the hundreds-place exists but happens to be `0`.
    ///
    /// The `-1` sentinel allows you to differentiate between a digit that is absent and a digit that is simply `0`.
    #[inline]
    pub fn digit_3_with_dda(&self) -> i8 {
        if self.current_value.abs() >= T::from_i32(100) {
            ((self.current_value.abs() / T::from_i32(100)) % T::from_i32(10)).as_i8()
        }
        else {
            -1
        }
    }

    /// Returns the digit in the thousands-place of current_value.
    ///
    /// Will always return a positive value if the digit exists.
    ///
    /// Will return -1 if the digit is NOT being used.
    ///
    /// ### What is DDA?
    /// DDA stands for Digit Drop Accounting.  It can be used to determine if a digit has been dropped
    /// from a number or if a digit just happens to be 0.
    ///
    /// ### Example
    /// - If `current_value` is `6`, `digit_2_with_dda` returns `-1` — no tens-place exists.
    /// - If `current_value` is `63`, `digit_2_with_dda` returns `6` — the tens-place exists and is `6`.
    /// - If `current_value` is `103`, `digit_3_with_dda` returns `1` — the hundreds-place exists and is `1`.
    /// - If `current_value` is `1003`, `digit_3_with_dda` returns `0` — the hundreds-place exists but happens to be `0`.
    ///
    /// The `-1` sentinel allows you to differentiate between a digit that is absent and a digit that is simply `0`.
    #[inline]
    pub fn digit_4_with_dda(&self) -> i8 {
        if self.current_value.abs() >= T::from_i32(1000) {
            ((self.current_value.abs() / T::from_i32(1000)) % T::from_i32(10)).as_i8()
        }
        else {
            -1
        }
    }

    /// Returns the digit in the ten-thousands-place of current_value.
    ///
    /// Will always return a positive value if the digit exists.
    ///
    /// Will return -1 if the digit is NOT being used.
    ///
    /// ### What is DDA?
    /// DDA stands for Digit Drop Accounting.  It can be used to determine if a digit has been dropped
    /// from a number or if a digit just happens to be 0.
    ///
    /// ### Example
    /// - If `current_value` is `6`, `digit_2_with_dda` returns `-1` — no tens-place exists.
    /// - If `current_value` is `63`, `digit_2_with_dda` returns `6` — the tens-place exists and is `6`.
    /// - If `current_value` is `103`, `digit_3_with_dda` returns `1` — the hundreds-place exists and is `1`.
    /// - If `current_value` is `1003`, `digit_3_with_dda` returns `0` — the hundreds-place exists but happens to be `0`.
    ///
    /// The `-1` sentinel allows you to differentiate between a digit that is absent and a digit that is simply `0`.
    #[inline]
    pub fn digit_5_with_dda(&self) -> i8 {
        if self.current_value.abs() >= T::from_i32(10000) {
            ((self.current_value.abs() / T::from_i32(10000)) % T::from_i32(10)).as_i8()
        }
        else {
            -1
        }
    }

    /// Returns the digit in the hundred-thousands-place of current_value.
    ///
    /// Will always return a positive value if the digit exists.
    ///
    /// Will return -1 if the digit is NOT being used.
    ///
    /// ### What is DDA?
    /// DDA stands for Digit Drop Accounting.  It can be used to determine if a digit has been dropped
    /// from a number or if a digit just happens to be 0.
    ///
    /// ### Example
    /// - If `current_value` is `6`, `digit_2_with_dda` returns `-1` — no tens-place exists.
    /// - If `current_value` is `63`, `digit_2_with_dda` returns `6` — the tens-place exists and is `6`.
    /// - If `current_value` is `103`, `digit_3_with_dda` returns `1` — the hundreds-place exists and is `1`.
    /// - If `current_value` is `1003`, `digit_3_with_dda` returns `0` — the hundreds-place exists but happens to be `0`.
    ///
    /// The `-1` sentinel allows you to differentiate between a digit that is absent and a digit that is simply `0`.
    #[inline]
    pub fn digit_6_with_dda(&self) -> i8 {
        if self.current_value.abs() >= T::from_i32(100000) {
            ((self.current_value.abs() / T::from_i32(100000)) % T::from_i32(10)).as_i8()
        }
        else {
            -1
        }
    }

    /// Returns the digit in the millions-place of current_value.
    ///
    /// Will always return a positive value if the digit exists.
    ///
    /// Will return -1 if the digit is NOT being used.
    ///
    /// ### What is DDA?
    /// DDA stands for Digit Drop Accounting.  It can be used to determine if a digit has been dropped
    /// from a number or if a digit just happens to be 0.
    ///
    /// ### Example
    /// - If `current_value` is `6`, `digit_2_with_dda` returns `-1` — no tens-place exists.
    /// - If `current_value` is `63`, `digit_2_with_dda` returns `6` — the tens-place exists and is `6`.
    /// - If `current_value` is `103`, `digit_3_with_dda` returns `1` — the hundreds-place exists and is `1`.
    /// - If `current_value` is `1003`, `digit_3_with_dda` returns `0` — the hundreds-place exists but happens to be `0`.
    ///
    /// The `-1` sentinel allows you to differentiate between a digit that is absent and a digit that is simply `0`.
    #[inline]
    pub fn digit_7_with_dda(&self) -> i8 {
        if self.current_value.abs() >= T::from_i32(1000000) {
            ((self.current_value.abs() / T::from_i32(1000000)) % T::from_i32(10)).as_i8()
        }
        else {
            -1
        }
    }

    /// Returns the digit in the ten-millions-place of current_value.
    ///
    /// Will always return a positive value if the digit exists.
    ///
    /// Will return -1 if the digit is NOT being used.
    ///
    /// ### What is DDA?
    /// DDA stands for Digit Drop Accounting.  It can be used to determine if a digit has been dropped
    /// from a number or if a digit just happens to be 0.
    ///
    /// ### Example
    /// - If `current_value` is `6`, `digit_2_with_dda` returns `-1` — no tens-place exists.
    /// - If `current_value` is `63`, `digit_2_with_dda` returns `6` — the tens-place exists and is `6`.
    /// - If `current_value` is `103`, `digit_3_with_dda` returns `1` — the hundreds-place exists and is `1`.
    /// - If `current_value` is `1003`, `digit_3_with_dda` returns `0` — the hundreds-place exists but happens to be `0`.
    ///
    /// The `-1` sentinel allows you to differentiate between a digit that is absent and a digit that is simply `0`.
    #[inline]
    pub fn digit_8_with_dda(&self) -> i8 {
        if self.current_value.abs() >= T::from_i32(10000000) {
            ((self.current_value.abs() / T::from_i32(10000000)) % T::from_i32(10)).as_i8()
        }
        else {
            -1
        }
    }

    /// Returns the digit in the hundred-millions-place of current_value.
    ///
    /// Will always return a positive value if the digit exists.
    ///
    /// Will return -1 if the digit is NOT being used.
    ///
    /// ### What is DDA?
    /// DDA stands for Digit Drop Accounting.  It can be used to determine if a digit has been dropped
    /// from a number or if a digit just happens to be 0.
    ///
    /// ### Example
    /// - If `current_value` is `6`, `digit_2_with_dda` returns `-1` — no tens-place exists.
    /// - If `current_value` is `63`, `digit_2_with_dda` returns `6` — the tens-place exists and is `6`.
    /// - If `current_value` is `103`, `digit_3_with_dda` returns `1` — the hundreds-place exists and is `1`.
    /// - If `current_value` is `1003`, `digit_3_with_dda` returns `0` — the hundreds-place exists but happens to be `0`.
    ///
    /// The `-1` sentinel allows you to differentiate between a digit that is absent and a digit that is simply `0`.
    #[inline]
    pub fn digit_9_with_dda(&self) -> i8 {
        if self.current_value.abs() >= T::from_i32(100000000) {
            ((self.current_value.abs() / T::from_i32(100000000)) % T::from_i32(10)).as_i8()
        }
        else {
            -1
        }
    }
    // ######################################################################################## //



    // ################################### ADD METHODS ######################################## //
    /// Adds to the start_value of the ticker by the passed value.  Can take in negatives for subtraction.
    ///
    /// Will not let the result of summing cause overflow or wrapping; results will always be within [`i8::MIN`] to [`i8::MAX`] (inclusive).
    #[inline]
    pub fn add_to_start_value(&mut self, value: T) {
        self.start_value = self.start_value.saturating_add(&value);
    }

    /// Adds to the current_value of the ticker by the passed value.  Can take in negatives for subtraction.
    ///
    /// Will not let the result of summing cause overflow or wrapping; results will always be within start_value to end_value (inclusive).
    pub fn add_to_current_value(&mut self, value: T) {
        let min = self.start_value.min(self.end_value);
        let max = self.start_value.max(self.end_value);
        self.current_value = self.current_value.saturating_add(&value).clamp(min, max);
    }

    /// Adds to the end_value of the ticker by the passed value.  Can take in negatives for subtraction.
    ///
    /// Will not let the result of summing cause overflow or wrapping; results will always be within [`i8::MIN`] to [`i8::MAX`] (inclusive).
    #[inline]
    pub fn add_to_end_value(&mut self, value: T) {
        self.end_value = self.end_value.saturating_add(&value);
    }

    /// Adds to the interval of the ticker by the passed value.  Can take in negatives for subtraction.
    ///
    /// # IMPORTANT
    /// Interval can never be 0, a negative number, or go past f32::MAX; the reasoning for this is that it would cause the
    /// .tick method to create crazy values. If your goal is to slow time or slow an accumulation to the point that it reverses it,
    /// I suggest you flip the tick direction using .tick_up or .tick_down depending on which direction you want the counting to flip to.
    #[inline]
    pub fn add_to_interval(&mut self, value: f32) {
        self.interval = (self.interval + value).clamp(f32::MIN_POSITIVE, f32::MAX);
    }
    // ######################################################################################## //



    // ################################# MISCELLANEOUS METHODS ################################ //
    /// Returns the percentage of `current_value` relative to a specified boundary.
    /// - `"start"` measures distance from `start_value` toward `current_value`, all within the range of
    /// `start_value` to `end_value`.
    /// - `"end"` measures distance from `end_value` toward `current_value`, all within the range of
    /// `end_value` to `start_value`.
    ///
    /// Pass `"start"` to measure how far `current_value` is from `start_value`, or `"end"` to
    /// measure how far it is from `end_value`.  A return value of `0.0` means `current_value`
    /// is at the specified boundary, and `1.0` means it is at the opposite boundary.
    ///
    /// # Panics
    /// Panics if `boundary` is anything other than `"start"` or `"end"`.
    ///
    /// # Special Case
    /// Returns `-1.0` if `start_value` and `end_value` are equal, as no meaningful range exists.
    ///
    /// # Examples
    /// ```
    /// // "start" — how far current_value is from start_value
    /// // start_value = 0, current_value = 40, end_value = 100
    /// // Returns 0.4
    ///
    /// // "end" — how far current_value is from end_value
    /// // start_value = 0, current_value = 60, end_value = 100
    /// // Returns 0.4
    ///
    /// // "start" with negative start
    /// // start_value = -37, current_value = 40, end_value = 80
    /// // Returns ~0.6581
    ///
    /// // "end" with negative start
    /// // start_value = -37, current_value = 40, end_value = 80
    /// // Returns ~0.3419
    ///
    /// // Equal boundaries (special case)
    /// // start_value = 100, current_value = x, end_value = 100
    /// // Returns -1.0
    ///
    /// // Invalid boundary
    /// // boundary = "middle"
    /// // Panics: INVALID TICKER BOUNDARY: 'middle'. Expected "start" or "end".
    /// ```
    pub fn percentage_from_boundary(&self, boundary: &str) -> f32 {

        if self.start_value == self.end_value {
            return -1.0;
        }

        let start: f32 = self.start_value.as_f32();
        let current: f32 = self.current_value.as_f32();
        let end: f32 = self.end_value.as_f32();

        let range_reciprocal: f32 = 1.0 / (end - start);

        match boundary {
            "start" =>  { (current - start) * range_reciprocal }
            "end"   =>  { (end - current) * range_reciprocal }
            _       =>  panic!("[{}INVALID TICKER BOUNDARY{}]: '{}'. Expected \"start\" or \"end\".", TestColors::FAIL, TestColors::DEFAULT, boundary),
        }
    }

    /// Will set the current_value to be equal to the start_value and the digit field of the Ticker
    /// will be changed according to the new ones-place value that is seen after current_value's reset.
    ///
    /// Digit is always to reflect current_value's ones-place.
    #[inline]
    pub fn reset(&mut self) {
        self.current_value = self.start_value;
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
    pub fn tick(&mut self, delta: f32) {

        // PAUSE STATUS
        // If paused, go no further as we don't need to calculate the new current_value since the Ticker is frozen.
        if self.is_paused {
            return;
        }

        // DELTA ACCUMULATION
        // Add to the accrued delta so that we can later determine if we've gone over the interval value and need to fire another tick.
        self.accrued_delta += delta;

        // TICK COLLECTION (TC)
        // Acquiring the amount of tick fires that occurred within the given frame based on if
        // the Ticker is set to handle frame spikes.
        let ticks = match self.is_handling_frame_spikes {

            // TC FOR HANDLING FRAME SPIKES
            // When frame spike handling is active, all ticks that accumulated during a large
            // delta are collected at once.  The remainder after division is kept in accrued_delta
            // so that partial progress toward the next tick is not lost between frames.
            true => {
                let tocks = T::from_f32(self.accrued_delta / self.interval);
                self.accrued_delta %= self.interval;    // Carrying remainder over to keep ticking accuracy.
                tocks
            },

            // TC FOR ~NOT~ HANDLING FRAME SPIKES
            // When frame spike handling is inactive, only one tick is allowed to fire per call
            // regardless of how large the delta was.  One interval is subtracted from accrued_delta
            // rather than resetting to zero so that the timer remains accurate over time — any
            // leftover time beyond the single tick carries into the next frame naturally.
            false => match self.accrued_delta >= self.interval {
                true => {
                    self.accrued_delta -= self.interval;
                    T::from_i32(1)
                },
                false => T::from_i32(0),
            },
        };

        // TICK FIRE TO CHANGE CURRENT_VALUE
        // Will only ever tick fire if the accrued delta pushed ticks beyond the interval value.
        // This check ensures we aren't needlessly firing for every frame, rather we are firing
        // based on if we've passed over the interval threshold using our constant accrual.
        if ticks > T::from_i32(0) {

            // TICK FIRE DIRECTION
            // Increase or decrease current_value's new host based on if the Ticker is counting up or down.
            let new_value = match self.is_ticking_up {
                true  => self.current_value.saturating_add(&ticks),
                false => self.current_value.saturating_sub(&ticks),
            };

            // DETERMINING CURRENT_VALUE'S BOUNDARIES
            // Since start_value and end_value can be either negative or positive at any given moment,
            // we must throw both values against one another to determine whose greater/lesser than
            // the other so that we can properly clamp down current_value to its allowed range.
            let min = self.start_value.min(self.end_value);
            let max = self.start_value.max(self.end_value);

            // RESET DETERMINATION + CURRENT_VALUE ASSIGNMENT
            // Will change current_value's assignment using new_value based on if the Ticker is set to loop or not.
            match self.is_looping {

                // LOOPING IS ACTIVE
                // Assign current_value to its new host and then reset it to the Ticker's start_value
                // if either of its boundaries -- start_value and end_value -- are hit.
                true => {
                    self.current_value = new_value;
                    if self.current_value <= min || self.current_value >= max {
                        self.reset();
                    }
                },

                // LOOPING IS INACTIVE
                // current_value can assume its new host after new_value has been clamped to the allowed range.
                false => {
                    self.current_value = new_value.clamp(min, max);
                },
            };
        }
    }
    // ######################################################################################## //
}
