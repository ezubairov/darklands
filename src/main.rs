use bevy::prelude::*;

mod components;
mod gameloop;
mod render;
mod states;
mod ui;
mod utils;

mod prelude {
    pub use bevy::prelude::*;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const SCREEN_WIDTH: i32 = 80;
    pub use crate::components::*;
    pub use crate::gameloop::*;
    pub use crate::states::*;
    pub use crate::ui::*;
    pub use crate::utils::*;
    pub use bracket_bevy::prelude::*;
    pub use rand::Rng;
}

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
                        fit_canvas_to_parent: true,
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(render::plugin)
        .init_state::<RunState>()
        .init_state::<GameLoopState>()
        .add_plugins(GameloopPluginGroup)
        .add_plugins(UIPlugin)
        .run();
}
