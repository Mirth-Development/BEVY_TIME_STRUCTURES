
use bevy::prelude::*;

// Tests for Chronolog
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TimeStructures{})
        .run();
}
