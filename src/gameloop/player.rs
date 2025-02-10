use crate::prelude::*;

fn try_move_player(delta_x: i32, delta_y: i32, position: &mut Position) {
    position.x += delta_x;
    position.y += delta_y;
}

fn process_inputs(
    kb_input: Res<ButtonInput<KeyCode>>,
    mut player_position: Query<(Entity, &mut Position), With<Player>>,
) {
    let (player_entity, mut position) = player_position.single_mut();
    match kb_input.get_just_pressed().next() {
        Some(KeyCode::ArrowLeft | KeyCode::KeyH) => try_move_player(-1, 0, &mut position),
        Some(KeyCode::ArrowDown | KeyCode::KeyJ) => try_move_player(0, 1, &mut position),
        Some(KeyCode::ArrowUp | KeyCode::KeyK) => try_move_player(0, -1, &mut position),
        Some(KeyCode::ArrowRight | KeyCode::KeyL) => try_move_player(1, 0, &mut position),
        _ => (),
    }
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            process_inputs.run_if(in_state(GameLoopState::PlayerTurn)),
        );
    }
}
