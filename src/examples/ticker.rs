
use bevy::prelude::*;

// Tests for Ticker
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TimeStructures{})
        .run();
}
