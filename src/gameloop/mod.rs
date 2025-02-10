use crate::prelude::*;
pub mod map;
pub use map::*;
pub mod spawner;
pub use spawner::*;
pub mod player;
pub use player::*;

use bevy::{app::PluginGroupBuilder, prelude::*};

fn setup(mut gameloop_state: ResMut<NextState<GameLoopState>>) {
    gameloop_state.set(GameLoopState::MapGeneration);
}

struct GameloopOrchestratorPlugin;
impl Plugin for GameloopOrchestratorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(RunState::GameLoop), setup);
    }
}

pub struct GameloopPluginGroup;
impl PluginGroup for GameloopPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(GameloopOrchestratorPlugin)
            .add(MapPlugin)
            .add(SpawnerPlugin)
            .add(PlayerPlugin)
    }
}
