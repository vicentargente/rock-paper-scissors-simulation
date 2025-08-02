use bevy::{prelude::*, window::EnabledButtons};
use rock_paper_scissors::{game_state::GameState, systems::{collision::{collision_system, update_spatial_grid_system}, movement::movement_system, setup::{setup_system, setup_ui, spawn_sprites}, team_counts::{check_winner_team, update_team_counts}}};
use rock_paper_scissors::constants::{WINDOW_WIDTH, WINDOW_HEIGHT};


fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rock Paper Scissors".to_string(),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                resize_constraints: WindowResizeConstraints {
                    min_width: WINDOW_WIDTH,
                    min_height: WINDOW_HEIGHT,
                    max_width: WINDOW_WIDTH,
                    max_height: WINDOW_HEIGHT
                },
                resizable: false,
                enabled_buttons: EnabledButtons {
                    maximize: false,
                    ..default()
                },
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        // .insert_resource(ClearColor(Color::srgb(0.9, 0.9, 0.9)))
        .add_systems(Startup, (setup_system, setup_ui, spawn_sprites).chain())
        .add_systems(Update, (movement_system,
            (update_spatial_grid_system, collision_system).chain(),
            update_team_counts,
            check_winner_team).chain().run_if(in_state(GameState::Running)))
        .run();
}




