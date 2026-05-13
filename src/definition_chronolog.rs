
// Imports
use bevy::ecs::component::Component;
use bevy::reflect::Reflect;
use bevy::time::{Timer, TimerMode};
use crate::definition_ticker::Ticker;

/// Used to create timers that can optionally store digits from the hundreds place to the thousandths
/// place.  Digits can be preset by assigning values to digit properties and timers for each digit
/// can be assigned independently for all the insanity that comes with declaring fancy clocks.
///
/// Digits are declared with the datatype u32 and not u8 because the tick system is set up
/// to handle frame spikes.  Frame spikes with timers inside Bevy can be dealt with using the times_finished_this_tick()
/// method on a timer and counting the number of ticks that occurred during the time it took to pass through the
/// delta happening mid-spike.  I'm being a bit insane by applying them to all digits.  Realistically speaking,
/// applying u32 to the thousandth and hundredth place is absurd over going with the just_finished() option that Bevy provides on timers
/// (nobody is gonna have a 10+ or 100+ second frame spike).  But uh...  CODE SYMMETRY (AKA OCD) CALLS FOR INEFFICIENCY!
///
/// By default, Tickers within a Chronolog have nothing in them.
/// By using the new method, Tickers will count up and repeat once they hit their max value.
/// You can still do countdown logic with tickers moving up in time, just takes a little more work.  But
/// if you'd like you can give a default Ticker custom timers to do countdown effects more intuitively.
#[derive(Component, Reflect, Debug)]
pub struct Chronolog {
    pub start_value: Option<f32>,
    pub ticker_for_hundreds: Option<Ticker>,
    pub ticker_for_tens: Option<Ticker>,
    pub ticker_for_ones: Option<Ticker>,
    pub ticker_for_tenths: Option<Ticker>,
    pub ticker_for_hundredths: Option<Ticker>,
    pub ticker_for_thousandths: Option<Ticker>,
}

impl Default for Chronolog {
    fn default() -> Self {
        Self {
            start_value: Some(0.0),
            ticker_for_hundreds: Some(Ticker::default()),
            ticker_for_tens: Some(Ticker::default()),
            ticker_for_ones: Some(Ticker::default()),
            ticker_for_tenths: Some(Ticker::default()),
            ticker_for_hundredths: Some(Ticker::default()),
            ticker_for_thousandths: Some(Ticker::default()),
        }
    }
}

impl Chronolog {

    /// Requires an Option to be thrown into it for usage, the Option may be filled or None.
    ///
    /// Passing in None will set the start_value to 0.0.
    /// Passing in Some(INSERT_FLOATING_POINT_HERE) will set the start_value to INSERT_FLOATING_POINT_HERE.
    ///
    /// Does not work with negatives, so don't even try unless you'd like to cry like I have.
    pub fn new(starting_value: Option<f32>) -> Self {
        Self {

            start_value: Some(starting_value.unwrap_or(0.0)),

            ticker_for_hundreds: Some(Ticker{
                number: Some(0),
                timer: Some(Timer::from_seconds(100.0, TimerMode::Repeating)),
            }),

            ticker_for_tens: Some(Ticker{
                number: Some(0),
                timer: Some(Timer::from_seconds(10.0, TimerMode::Repeating)),
            }),

            ticker_for_ones: Some(Ticker{
                number: Some(0),
                timer: Some(Timer::from_seconds(1.0, TimerMode::Repeating)),
            }),

            ticker_for_tenths: Some(Ticker{
                number: Some(0),
                timer: Some(Timer::from_seconds(0.1, TimerMode::Repeating)),
            }),

            ticker_for_hundredths: Some(Ticker{
                number: Some(0),
                timer: Some(Timer::from_seconds(0.01, TimerMode::Repeating)),
            }),

            ticker_for_thousandths: Some(Ticker{
                number: Some(0),
                timer: Some(Timer::from_seconds(0.001, TimerMode::Repeating)),
            }),
        }
    }

    /// This type of reset will cause for the Chronolog to continue ticking immediately after reset
    /// in the same way that a Chronolog will tick when created using the "new" method.
    pub fn reset(&mut self) {
        *self = Chronolog::new(None);
    }

    /// Will wipe out all the tickers in the Chronolog.  Can be used to create a blank slate to add new
    /// tickers onto a Chronolog if you want to.
    pub fn blank(&mut self) {
        *self = Chronolog::default();
    }

    /// Returns how long a Chronolog has until it hits the zero value.
    ///
    /// The start_value of a Chronolog dictates the top of the countdown and the constant increasing
    /// values of the Chronolog are reversed through subtraction in this method to create a countdown effect.
    pub fn get_countdown_number(&self) -> f32 {

        let start = self.start_value.unwrap_or(0.0);
        let elapsed = start - self.get_number();

        // Prevents the countdown number from returning a negative value.
        // Will return the elapsed time if it's greater than 1.0.
        // Will return 0.0 if the elapsed time comes out as negative.
        if elapsed > 0.0 {
            elapsed
        }
        else {
            0.0
        }
    }

    /// Returns a string for the current countdown value, the number of digits is based on how many
    /// whole places is desired and how many floating places is desired.
    ///
    /// It's important to note that a decimal is added into the string and should be accounted for
    /// if you're gonna try and convert the string into an indexable structure.
    pub fn get_countdown_string(
        &self,
        number_of_whole_places: usize,
        number_of_floating_places: usize
    ) -> String {

        let countdown = self.get_countdown_number();
        let character_count = number_of_whole_places + number_of_floating_places + 1;

        // Left side of printout is dictated implicitly by (number_of_characters - floating).
        format!("{:0>number_of_characters$.floating$}",
                countdown,
                number_of_characters = character_count,
                floating = number_of_floating_places
        )
    }

    /// Returns the number that's in the hundreds' ticker.  Will return 0 if there is no ticker.
    ///
    /// Something to keep in mind is that a ticker could exist for the hundreds place and also have
    /// a value of 0 for its number.  This means that 0 doesn't ALWAYS mean that a ticker doesn't exist for
    /// the given digit.
    pub fn digit_for_hundreds(&self) -> u32 {
        self.ticker_for_hundreds.as_ref().and_then(|ticker| ticker.number).unwrap_or(0)
    }

    /// Returns the number that's in the tens' ticker.  Will return 0 if there is no ticker.
    ///
    /// Something to keep in mind is that a ticker could exist for the tens place and also have
    /// a value of 0 for its number.  This means that 0 doesn't ALWAYS mean that a ticker doesn't exist for
    /// the given digit.
    pub fn digit_for_tens(&self) -> u32 {
        self.ticker_for_tens.as_ref().and_then(|ticker| ticker.number).unwrap_or(0)
    }

    /// Returns the number that's in the ones' ticker.  Will return 0 if there is no ticker.
    ///
    /// Something to keep in mind is that a ticker could exist for the ones place and also have
    /// a value of 0 for its number.  This means that 0 doesn't ALWAYS mean that a ticker doesn't exist for
    /// the given digit.
    pub fn digit_for_ones(&self) -> u32 {
        self.ticker_for_ones.as_ref().and_then(|ticker| ticker.number).unwrap_or(0)
    }

    /// Returns the number that's in the tenths' ticker.  Will return 0 if there is no ticker.
    ///
    /// Something to keep in mind is that a ticker could exist for the tenths place and also have
    /// a value of 0 for its number.  This means that 0 doesn't ALWAYS mean that a ticker doesn't exist for
    /// the given digit.
    pub fn digit_for_tenths(&self) -> u32 {
        self.ticker_for_tenths.as_ref().and_then(|ticker| ticker.number).unwrap_or(0)
    }

    /// Returns the number that's in the hundredths' ticker.  Will return 0 if there is no ticker.
    ///
    /// Something to keep in mind is that a ticker could exist for the hundredths place and also have
    /// a value of 0 for its number.  This means that 0 doesn't ALWAYS mean that a ticker doesn't exist for
    /// the given digit.
    pub fn digit_for_hundredths(&self) -> u32 {
        self.ticker_for_hundredths.as_ref().and_then(|ticker| ticker.number).unwrap_or(0)
    }

    /// Returns the number that's in the thousandths' ticker.  Will return 0 if there is no ticker.
    ///
    /// Something to keep in mind is that a ticker could exist for the thousandths place and also have
    /// a value of 0 for its number.  This means that 0 doesn't ALWAYS mean that a ticker doesn't exist for
    /// the given digit.
    pub fn digit_for_thousandths(&self) -> u32 {
        self.ticker_for_thousandths.as_ref().and_then(|ticker| ticker.number).unwrap_or(0)
    }

    /// Returns the number that's in the hundreds' ticker as a string.  Will return 0 as a string if there is no ticker.
    ///
    /// Something to keep in mind is that a ticker could exist for the hundreds place and also have
    /// a value of 0 for its number.  This means that 0 doesn't ALWAYS mean that a ticker doesn't exist for
    /// the given digit.
    pub fn string_for_hundreds(&self) -> String {
        let hundreds = self.ticker_for_hundreds.as_ref().and_then(|ticker| ticker.number).unwrap_or(0);
        format!("{}", hundreds)
    }

    /// Returns the number that's in the tens' ticker as a string.  Will return 0 as a string if there is no ticker.
    ///
    /// Something to keep in mind is that a ticker could exist for the tens place and also have
    /// a value of 0 for its number.  This means that 0 doesn't ALWAYS mean that a ticker doesn't exist for
    /// the given digit.
    pub fn string_for_tens(&self) -> String {
        let tens = self.ticker_for_tens.as_ref().and_then(|ticker| ticker.number).unwrap_or(0);
        format!("{}", tens)
    }

    /// Returns the number that's in the ones' ticker as a string.  Will return 0 as a string if there is no ticker.
    ///
    /// Something to keep in mind is that a ticker could exist for the ones place and also have
    /// a value of 0 for its number.  This means that 0 doesn't ALWAYS mean that a ticker doesn't exist for
    /// the given digit.
    pub fn string_for_ones(&self) -> String {
        let ones = self.ticker_for_ones.as_ref().and_then(|ticker| ticker.number).unwrap_or(0);
        format!("{}", ones)
    }

    /// Returns the number that's in the tenths' ticker as a string.  Will return 0 as a string if there is no ticker.
    ///
    /// Something to keep in mind is that a ticker could exist for the tenths place and also have
    /// a value of 0 for its number.  This means that 0 doesn't ALWAYS mean that a ticker doesn't exist for
    /// the given digit.
    pub fn string_for_tenths(&self) -> String {
        let tenths = self.ticker_for_tenths.as_ref().and_then(|ticker| ticker.number).unwrap_or(0);
        format!("{}", tenths)
    }

    /// Returns the number that's in the hundredths' ticker as a string.  Will return 0 as a string if there is no ticker.
    ///
    /// Something to keep in mind is that a ticker could exist for the hundredths place and also have
    /// a value of 0 for its number.  This means that 0 doesn't ALWAYS mean that a ticker doesn't exist for
    /// the given digit.
    pub fn string_for_hundredths(&self) -> String {
        let hundredths = self.ticker_for_hundredths.as_ref().and_then(|ticker| ticker.number).unwrap_or(0);
        format!("{}", hundredths)
    }

    /// Returns the number that's in the thousandths' ticker as a string.  Will return 0 as a string if there is no ticker.
    ///
    /// Something to keep in mind is that a ticker could exist for the thousandths place and also have
    /// a value of 0 for its number.  This means that 0 doesn't ALWAYS mean that a ticker doesn't exist for
    /// the given digit.
    pub fn string_for_thousandths(&self) -> String {
        let thousandths = self.ticker_for_thousandths.as_ref().and_then(|ticker| ticker.number).unwrap_or(0);
        format!("{}", thousandths)
    }

    /// Will return the current value of the Chronolog.  Any unused digits will be labeled as 0.
    pub fn get_number(&self) -> f32 {

        let mut total_time: f32 = 0.0;

        // WHY I LOVE AND HATE RUST
        // 1. Borrow by using as_ref().
        // 2. Get rid of double Option results using and_then(#Anonymous).
        // 3. Get what's in number or set to 0 if nothing is there using unwrap_or(0).
        // 4. Typecast to f32 since tickers store u32 and the end result we're going for is a decimal.
        let hundreds    = (self.ticker_for_hundreds      .as_ref().and_then(|ticker| ticker.number).unwrap_or(0)) as f32 * 100.0;
        let tens        = (self.ticker_for_tens          .as_ref().and_then(|ticker| ticker.number).unwrap_or(0)) as f32 * 10.0;
        let ones        = (self.ticker_for_ones          .as_ref().and_then(|ticker| ticker.number).unwrap_or(0)) as f32 * 1.0;
        let tenths      = (self.ticker_for_tenths        .as_ref().and_then(|ticker| ticker.number).unwrap_or(0)) as f32 * 0.1;
        let hundredths  = (self.ticker_for_hundredths    .as_ref().and_then(|ticker| ticker.number).unwrap_or(0)) as f32 * 0.01;
        let thousandths = (self.ticker_for_thousandths   .as_ref().and_then(|ticker| ticker.number).unwrap_or(0)) as f32 * 0.001;

        total_time += hundreds;
        total_time += tens;
        total_time += ones;
        total_time += tenths;
        total_time += hundredths;
        total_time += thousandths;

        total_time
    }

    /// Will return the current value of the Chronolog as a string.  Any unused digits will be labeled as 0.
    pub fn get_string(&self) -> String {

        // WHY I LOVE AND HATE RUST
        // 1. Borrow by using as_ref().
        // 2. Get rid of double Option results using and_then(#Anonymous).
        // 3. Get what's in number or set to 0 if nothing is there using unwrap_or(0).
        let hundreds    = self.ticker_for_hundreds      .as_ref().and_then(|ticker| ticker.number).unwrap_or(0);
        let tens        = self.ticker_for_tens          .as_ref().and_then(|ticker| ticker.number).unwrap_or(0);
        let ones        = self.ticker_for_ones          .as_ref().and_then(|ticker| ticker.number).unwrap_or(0);
        let tenths      = self.ticker_for_tenths        .as_ref().and_then(|ticker| ticker.number).unwrap_or(0);
        let hundredths  = self.ticker_for_hundredths    .as_ref().and_then(|ticker| ticker.number).unwrap_or(0);
        let thousandths = self.ticker_for_thousandths   .as_ref().and_then(|ticker| ticker.number).unwrap_or(0);

        format!("{}{}{}.{}{}{}", hundreds, tens, ones, tenths, hundredths, thousandths)
    }
}
