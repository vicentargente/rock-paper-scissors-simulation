use bevy::{ecs::{query::With, system::{Commands, Query, Res, ResMut}}, sprite::Anchor, state::state::NextState, text::{JustifyText, Text2d, TextFont, TextLayout}, transform::components::Transform, utils::default};

use crate::{components::team_count_text::TeamCountText, game_state::GameState, resources::{assets::TextAssets, team_counts::TeamCounts}, teams::Team};

pub fn update_team_counts(team_counts: Res<TeamCounts>, mut query: Query<&mut Text2d, With<TeamCountText>>,
) {
    if let Ok(mut text) = query.single_mut() {
        text.0 = format!(
            "Team counts\nRock: {}\nPaper: {}\nScissors: {}",
            team_counts.rock, team_counts.paper, team_counts.scissors
        );
    }
}


pub fn check_winner_team(team_counts: Res<TeamCounts>, mut commands: Commands, text_assets: Res<TextAssets>, mut next_state: ResMut<NextState<GameState>>) {
    let winner = if team_counts.rock == 0 {
        if team_counts.scissors != 0 {
            Team::Scissors
        }
        else {
            Team::Paper
        }
    }
    else if team_counts.paper == 0 {
        if team_counts.rock != 0 {
            Team::Rock
        }
        else {
            Team::Scissors
        }
    }
    else if team_counts.scissors == 0 {
        if team_counts.paper != 0 {
            Team::Paper
        }
        else {
            Team::Rock
        }
    }
    else {
        return;
    };

    let message = format!("Winner: {:?}", winner);
    commands.spawn((
        Text2d::new(message),
        TextFont {
            font: text_assets.font.clone(),
            font_size: 50.0,
            ..default()
        },
        TextLayout::new_with_justify(JustifyText::Center),
        Anchor::Center,
        Transform::from_xyz(0.0, 0.0, 1.0),
    ));

    next_state.set(GameState::GameOver);
}
