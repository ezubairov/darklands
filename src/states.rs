use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Default)]
pub enum RunState {
    #[default]
    MainMenuScreen,
    OptionsScreen,
    GameLoop,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Default)]
pub enum GameLoopState {
    #[default]
    NotHere,
    MapGeneration,
    PlayerTurn,
}
