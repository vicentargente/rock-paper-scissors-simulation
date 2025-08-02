use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Team {
    Rock,
    Paper,
    Scissors,
}

impl Team {
    pub fn wins_against(&self, other: &Team) -> bool {
        match self {
            Team::Rock => *other == Team::Scissors,
            Team::Paper => *other == Team::Rock,
            Team::Scissors => *other == Team::Paper,
        }
    }
}


#[derive(Component)]
pub struct Velocity(pub Vec2);


#[derive(Component)]
pub struct Wander {
    pub target_direction: Vec2,
    pub timer: Timer,
}
