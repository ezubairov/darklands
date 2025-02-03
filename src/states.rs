use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Default)]
pub enum RunState {
    MainMenuScreen,
    #[default]
    MapGeneration,
    AwaitingInput,
}
