
// Modules
mod plugin;
mod definition_ticker;
mod definition_chronolog;
mod systems;


// Imports
use bevy::app::{App, Plugin};
use bevy::DefaultPlugins;
use crate::plugin::TimeStructures;

// A MAIN FILE?
// I'm using the main file for testing out things in the plugin.
// I'm not using a unit test file since I don't know how to implement one alongside
// Bevy's scheduler, and I need to test things in the scheduler to see how things are changing
// as the game plays.
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TimeStructures{})
        .run();
}
