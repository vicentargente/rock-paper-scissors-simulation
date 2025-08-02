use bevy::{asset::AssetServer, core_pipeline::core_2d::Camera2d, ecs::system::{Commands, Res}, math::{Vec2, Vec3}, sprite::{Anchor, Sprite}, text::{JustifyText, Text2d, TextFont, TextLayout}, time::{Timer, TimerMode}, transform::components::Transform, utils::default};

use crate::{components::team_count_text::TeamCountText, constants::{ENTITY_COUNT, WINDOW_HEIGHT, WINDOW_WIDTH}, resources::{assets::{TeamAssets, TextAssets}, borders::Borders, spatial_grid::SpatialGrid, team_counts::TeamCounts}, teams::{Team, Velocity, Wander}};

pub fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let team_assets = TeamAssets {
        rock: asset_server.load("sprites/rock.png"),
        paper: asset_server.load("sprites/paper.png"),
        scissors: asset_server.load("sprites/scissors.png"),
    };
    commands.insert_resource(team_assets);

    let text_assets = TextAssets {
        font: asset_server.load("fonts/FiraSans-Bold.ttf")
    };
    commands.insert_resource(text_assets);

    let borders = Borders {
        left: -WINDOW_WIDTH / 2.0,
        right: WINDOW_WIDTH / 2.0,
        top: WINDOW_HEIGHT / 2.0,
        bottom: -WINDOW_HEIGHT / 2.0,
    };
    commands.insert_resource(borders);

    
    let team_counts = TeamCounts {
        rock: ENTITY_COUNT,
        paper: ENTITY_COUNT,
        scissors: ENTITY_COUNT,
    };
    commands.insert_resource(team_counts);


    let spatial_grid = SpatialGrid {
        cells: core::array::from_fn(|_| Vec::new())
    };
    commands.insert_resource(spatial_grid);


    commands.spawn(Camera2d::default());
}


pub fn setup_ui(mut commands: Commands, text_assets: Res<TextAssets>) {
    let font = text_assets.font.clone();
    let text_font = TextFont {
        font: font.clone(),
        font_size: 35.0,
        ..default()
    };

    commands.spawn((
        Text2d::new("Team counts"),
        text_font.clone(),
        TextLayout::new_with_justify(JustifyText::Left),
        Anchor::TopLeft,
        Transform::from_translation(Vec3::new(-WINDOW_WIDTH / 2.0 + 20.0, WINDOW_HEIGHT / 2.0 - 20.0, 0.0)),
        TeamCountText
    ));
}

pub fn spawn_sprites(mut commands: Commands, team_assets: Res<TeamAssets>) {
    for team in [Team::Rock, Team::Paper, Team::Scissors] {
        let image = match team {
            Team::Rock => &team_assets.rock,
            Team::Paper => &team_assets.paper,
            Team::Scissors => &team_assets.scissors,
        };

        for _ in 0..ENTITY_COUNT {
            commands.spawn((
                Sprite {
                    image: image.clone(),
                    ..default()
                },
                Transform::from_xyz(
                    (rand::random::<f32>() - 0.5) * WINDOW_WIDTH,
                    (rand::random::<f32>() - 0.5) * WINDOW_HEIGHT,
                    0.0
                ).with_scale(Vec3::splat(0.025)),
                team,
                Velocity(Vec2::ZERO),
                Wander {
                    target_direction: Vec2::new(rand::random::<f32>() * 2.0 - 1.0, rand::random::<f32>() * 2.0 - 1.0).normalize_or_zero(),
                    timer: Timer::from_seconds(rand::random::<f32>() * 2.0, TimerMode::Repeating)
                }
            ));
        }
    }
}
