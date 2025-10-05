use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    AppMenu, // Main Menu, Profile Management, Settings, Save/Load etc...
    Meta,        // Player in the mode of base editing between missions
    CoreRunning, //Player on the mission
    CorePaused,  //Player paused the game while on a mission
}
