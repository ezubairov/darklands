use bracket_bevy::prelude::*;
use bracket_lib::terminal::RGB;

use crate::prelude::*;

pub fn spawn(mut commands: Commands, mb: Res<MapBuilder>) {
    let player_start = mb.player_start;
    commands.spawn((
        Player {},
        Named("You, the player".to_string()),
        Position {
            x: player_start.x,
            y: player_start.y,
        },
        Renderable {
            glyph: to_cp437('@'),
            fg: RGB::named(WHITE),
            bg: RGB::named(BLACK),
        },
    ));
}

pub struct SpawnerPlugin;
impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameLoopState::MapGeneration), spawn);
    }
}
