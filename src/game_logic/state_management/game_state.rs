use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    MetaState, // Player in the mode of base editing between missions
    CoreRunning, //Player on the mission
    CorePaused,  //Player paused the game while on a mission
}
