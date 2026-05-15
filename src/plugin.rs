
// Imports
use bevy::prelude::*;
use crate::types::*;
use crate::systems::*;

/// Structure that acts as the main plugin for all of the time structures, adding this
/// as a plugin would implement the various timer structure definitions and Bevy systems.
pub struct TimeStructures {}
impl Plugin for TimeStructures {
    fn build(&self, app: &mut App) {
        app.register_type::<Ticker>();
        app.register_type::<Chronolog>();
        app.add_systems(First, (ticker_ticking, chronolog_ticking).chain());
    }
}
