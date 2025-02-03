use crate::prelude::*;

fn main_menu_screen(ctx: Res<BracketContext>) {
    ctx.draw_box_double(24, 18, 31, 10, RGB::named(WHEAT), RGB::named(BLACK));

    ctx.print_color_centered(20, RGB::named(YELLOW), RGB::named(BLACK), "Welcome to...");
    ctx.print_color_centered(21, RGB::named(CYAN), RGB::named(BLACK), "DARKLANDS");
    ctx.print_color_centered(
        22,
        RGB::named(GRAY),
        RGB::named(BLACK),
        "Use Up/Down Arrows and Enter",
    );
}

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            main_menu_screen.run_if(in_state(RunState::MainMenuScreen)),
        );
    }
}
