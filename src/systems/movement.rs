use bevy::{ecs::system::{Query, Res}, math::Vec2, time::Time, transform::components::Transform};

use crate::{resources::borders::Borders, teams::{Velocity, Wander}};

const MOVE_SPEED: f32 = 75.0;
const STEER_STRENGTH: f32 = 2.0;
const BORDER_MARGIN: f32 = 50.0;
const AVOIDANCE_STRENGTH: f32 = 4.0;

pub fn movement_system(
    time: Res<Time>,
    borders: Res<Borders>,
    mut query: Query<(&mut Wander, &mut Velocity, &mut Transform)>,
) {

    for (mut wander, mut velocity, mut transform) in query.iter_mut() {
        let mut avoidance_force = Vec2::ZERO;

        if transform.translation.x < borders.left + BORDER_MARGIN {
            avoidance_force.x = 1.0;
        } else if transform.translation.x > borders.right - BORDER_MARGIN {
            avoidance_force.x = -1.0;
        }

        if transform.translation.y < borders.bottom + BORDER_MARGIN {
            avoidance_force.y = 1.0;
        } else if transform.translation.y > borders.top - BORDER_MARGIN {
            avoidance_force.y = -1.0;
        }

        // Only pick a new random direction if we are NOT actively avoiding a border.
        if avoidance_force == Vec2::ZERO {
            wander.timer.tick(time.delta());
            if wander.timer.just_finished() {
                wander.target_direction = Vec2::new(rand::random::<f32>() * 2.0 - 1.0, rand::random::<f32>() * 2.0 - 1.0).normalize_or_zero();
            }
        }

        let avoidance_direction = avoidance_force.normalize_or_zero();

        let desired_velocity = 
            (wander.target_direction + avoidance_direction * AVOIDANCE_STRENGTH).normalize_or_zero() * MOVE_SPEED;

        let steering = (desired_velocity - velocity.0) * STEER_STRENGTH * time.delta_secs();
        velocity.0 += steering;
        velocity.0 = velocity.0.clamp_length_max(MOVE_SPEED);

        transform.translation.x += velocity.0.x * time.delta_secs();
        transform.translation.y += velocity.0.y * time.delta_secs();
    }
}
