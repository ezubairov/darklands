use bevy::prelude::*;

mod camera;
mod components;
mod map;
mod states;
mod utils;
mod render;

mod prelude {
    pub use bevy::prelude::*;
    pub const SCREEN_HEIGHT: i32 = 80;
    pub const SCREEN_WIDTH: i32 = 80;
    pub use crate::components::*;
    pub use crate::map::*;
    pub use crate::states::*;
    pub use crate::utils::*;
    pub use rand::Rng;
}

use bracket_bevy::BTermBuilder;
use prelude::*;

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Darklands".to_string(),
                        resolution: (SCREEN_WIDTH as f32 * 10.0, SCREEN_HEIGHT as f32 * 10.0)
                            .into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins((render::plugin))
        .init_state::<RunState>()
        // .add_systems(Startup, camera::setup)
        .add_plugins(MapPlugin)
        .run();
}
