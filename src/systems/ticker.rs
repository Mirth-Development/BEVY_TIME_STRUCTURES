
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
pub fn ticker_ticking(
    time: Res<Time>,
    mut tickers: Query<&mut Ticker>,
) {

    let delta = time.delta();

    for mut ticker in &mut tickers {
        ticker.tick(delta);
    }
}
