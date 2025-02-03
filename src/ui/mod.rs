use crate::prelude::*;

#[derive(Resource)]
struct MenuItems {
    items: Vec<MenuItemsEnum>,
    selected_index: usize,
}

enum MenuItemsEnum {
    StartGame,
    Options,
}
impl MenuItemsEnum {
    fn as_str(&self) -> String {
        match self {
            MenuItemsEnum::StartGame => "Start game".to_string(),
            MenuItemsEnum::Options => "Options".to_string(),
        }
    }

    fn to_vec() -> Vec<MenuItemsEnum> {
        vec![MenuItemsEnum::StartGame, MenuItemsEnum::Options]
    }
}

fn main_menu_screen(ctx: Res<BracketContext>, menu_items: Res<MenuItems>) {
    // Title box
    ctx.draw_box_double(24, 10, 31, 10, RGB::named(WHEAT), RGB::named(BLACK));
    ctx.print_color_centered(12, RGB::named(YELLOW), RGB::named(BLACK), "Welcome to...");
    ctx.print_color_centered(13, RGB::named(CYAN), RGB::named(BLACK), "DARKLANDS");
    ctx.print_color_centered(
        17,
        RGB::named(GRAY),
        RGB::named(BLACK),
        "Use Up/Down Arrows and Enter",
    );

    // Menu items
    ctx.draw_box_double(
        24,
        27,
        31,
        3 + menu_items.items.len() as i32,
        RGB::named(WHEAT),
        RGB::named(BLACK),
    );
    for (i, item) in menu_items.items.iter().enumerate() {
        if i == menu_items.selected_index {
            ctx.print_color_centered(
                29 + i as i32,
                RGB::named(BLACK),
                RGB::named(WHITE),
                item.as_str(),
            );
        } else {
            ctx.print_color_centered(
                29 + i as i32,
                RGB::named(WHITE),
                RGB::named(BLACK),
                item.as_str(),
            );
        }
    }
}

fn main_menu_input(
    kb_input: Res<ButtonInput<KeyCode>>,
    mut menu_items: ResMut<MenuItems>,
    mut run_state: ResMut<NextState<RunState>>,
) {
    match kb_input.get_just_pressed().next() {
        Some(KeyCode::ArrowUp | KeyCode::KeyK) => {
            println!("{}", menu_items.selected_index);
            if menu_items.selected_index != 0 {
                menu_items.selected_index -= 1
            } else {
                menu_items.selected_index = menu_items.items.len() - 1
            }
        }
        Some(KeyCode::ArrowDown | KeyCode::KeyJ) => {
            if menu_items.selected_index != menu_items.items.len() - 1 {
                menu_items.selected_index += 1
            } else {
                menu_items.selected_index = 0
            }
        }
        Some(KeyCode::Enter) => {
            let selected_item = &menu_items.items[menu_items.selected_index];
            match selected_item {
                MenuItemsEnum::StartGame => run_state.set(RunState::GameLoop),
                MenuItemsEnum::Options => run_state.set(RunState::OptionsScreen), 
            }
        }
        _ => (),
    }
}

fn clear_screen(ctx: Res<BracketContext>) {
    ctx.cls();
}

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (main_menu_screen, main_menu_input).run_if(in_state(RunState::MainMenuScreen)),
        )
        .add_systems(OnExit(RunState::MainMenuScreen), clear_screen)
        .insert_resource(MenuItems {
            items: MenuItemsEnum::to_vec(),
            selected_index: 0,
        });
    }
}
