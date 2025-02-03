use bevy::prelude::*;

#[derive(Component)]
struct MainCamera;

pub fn setup(mut commands: Commands) {
    let camera = Camera2d::default();
    commands.spawn((MainCamera, camera));
}
