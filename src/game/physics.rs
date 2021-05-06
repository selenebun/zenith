use bevy::prelude::*;

use crate::game::starfield::Star;
use crate::game::GameState;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(apply_acceleration.system().label("apply_acceleration"))
                .with_system(apply_velocity.system().after("apply_acceleration")),
        )
        .add_system(apply_star_velocity.system());
    }
}

#[derive(Debug)]
pub struct Acceleration(pub Vec2);

#[derive(Debug)]
pub struct Velocity(pub Vec2);

fn apply_acceleration(mut query: Query<(&Acceleration, &mut Velocity)>) {
    for (acceleration, mut velocity) in query.iter_mut() {
        velocity.0 += acceleration.0;
    }
}

fn apply_star_velocity(mut query: Query<(&mut Transform, &Velocity), With<Star>>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation += velocity.0.extend(0.0);
    }
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity), Without<Star>>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation += velocity.0.extend(0.0);
    }
}
