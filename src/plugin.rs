
// Imports
use bevy::app::{App, Plugin};
use crate::definition_ticker::Ticker;
use crate::definition_chronolog::Chronolog;
use crate::systems::SystemsForTimeStructures;

/// Structure that acts as the main plugin for all of the time structures, adding this
/// as a plugin would implement the various timer structure definitions and Bevy systems.
pub struct TimeStructures {}
impl Plugin for TimeStructures {
    fn build(&self, app: &mut App) {
        app.register_type::<Ticker>();
        app.register_type::<Chronolog>();
        app.add_plugins(SystemsForTimeStructures{});
    }
}
