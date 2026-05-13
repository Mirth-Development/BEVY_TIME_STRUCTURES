
// Imports
use bevy::app::{App, Plugin, First};
use bevy::ecs::system::{Res, Query};
use bevy::time::Time;
use crate::definition_ticker::Ticker;
use crate::definition_chronolog::Chronolog;

/// Used to enable the systems for the various TimeStructures (provides implementation to the definitions).
///
/// These systems primarily run on the "First" state within the Bevy scheduler since we want properties of
/// time to be adjusted before anything else gets messed with (time comes before all).
pub struct SystemsForTimeStructures {}
impl Plugin for SystemsForTimeStructures {
    fn build(&self, app: &mut App) {
        app.add_systems(First, (chronolog_ticking, ticker_ticking));
    }
}



// ####################################################################################################################### //
// SYSTEMS

/// Will loop through queried chronologs to see if they have tickers within them that need to be tick-tocked.
pub fn chronolog_ticking(
    time: Res<Time>,
    mut logs: Query<&mut Chronolog>,
) {

    let delta = time.delta();

    for mut log in &mut logs {
        if let Some(ticker) = &mut log.ticker_for_hundreds    { ticker.tick(delta); }
        if let Some(ticker) = &mut log.ticker_for_tens        { ticker.tick(delta); }
        if let Some(ticker) = &mut log.ticker_for_ones        { ticker.tick(delta); }
        if let Some(ticker) = &mut log.ticker_for_tenths      { ticker.tick(delta); }
        if let Some(ticker) = &mut log.ticker_for_hundredths  { ticker.tick(delta); }
        if let Some(ticker) = &mut log.ticker_for_thousandths { ticker.tick(delta); }

        //println!("{}", log.get_number());
    }
}

/// Will loop through queried tickers to initiate tick-tocking.  Chronolog tickers are bound to their chronolog
/// and IN THEORY should act as separate entity.  I think...?
pub fn ticker_ticking(
    time: Res<Time>,
    mut tickers: Query<&mut Ticker>,
) {

    let delta = time.delta();

    for mut ticker in &mut tickers {
        ticker.tick(delta);
        //println!("{}", ticker.get_number());
    }
}
// ####################################################################################################################### //
