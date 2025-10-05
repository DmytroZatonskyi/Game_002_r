use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    AppMenu, // Main Menu, Profile Management, Settings, Save/Load etc...
    InGame, // Meta-gameplay (upgrades, base view) Core-gameplay (mission execution)
}
