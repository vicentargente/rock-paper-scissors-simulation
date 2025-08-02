use bevy::ecs::resource::Resource;

use crate::teams::Team;

#[derive(Resource, Debug)]
pub struct TeamCounts {
    pub rock: u32,
    pub paper: u32,
    pub scissors: u32,
}

impl TeamCounts {
    pub fn decrease_team(&mut self, team: &Team) {
        match team {
            Team::Rock => self.rock = self.rock.saturating_sub(1),
            Team::Paper => self.paper = self.paper.saturating_sub(1),
            Team::Scissors => self.scissors = self.scissors.saturating_sub(1),
        }
    }

    pub fn increase_team(&mut self, team: &Team) {
        match team {
            Team::Rock => self.rock += 1,
            Team::Paper => self.paper += 1,
            Team::Scissors => self.scissors += 1,
        }
    }
}
