
use bevy_ecs::prelude::*;
use bevy_time::Time;
use crate::types::ticker_type::*;

/// Advances every `Ticker<V, P>` in the world by the time elapsed since the last frame.
///
/// This tick system is based on Bevy's Time resource, and more specifically its delta.  Best
/// used in Bevy's scheduler under a frame schedule (Update, First, Last, etcetera).
pub(crate) fn tick_tickers<V: TickerValue, P: TickerPrecision>(
    time: Res<Time>,
    mut tickers: Query<&mut Ticker<V, P>>,
) {
    // Read the high-precision f64 delta and convert it down to f32 or f16 if it's not an f64.
    let delta_in_seconds = P::from_f64(time.delta_secs_f64());
    for mut ticker in tickers.iter_mut() {
        ticker.tick(delta_in_seconds);
    }
}
