use bevy::prelude::*;

// Keep the `game_logic` module in the workspace; we won't import the state enums here yet.
mod game;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup() {
    info!("Setup complete (minimal). game_logic module present but unused in main.");
}
