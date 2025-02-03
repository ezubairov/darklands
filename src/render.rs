use crate::prelude::*;
use bevy::prelude::*;
use bracket_bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins(BTermBuilder::simple_80x50())
        .add_systems(Update, render.run_if(not(in_state(RunState::MainMenuScreen))));
}

fn render(ctx: Res<BracketContext>, renderables: Query<(&Position, &Renderable)>) {
    ctx.cls();

    renderables.iter().for_each(|(pos, render)| {
        ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
    });
}
