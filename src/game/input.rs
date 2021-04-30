use bevy::prelude::*;

use crate::game::player::{Player, Speed};
use crate::game::GameState;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(move_player.system().label("move_player")),
        );
    }
}

fn move_player(
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&Speed, &mut Transform), With<Player>>,
) {
    for (speed, mut transform) in query.iter_mut() {
        // Move at half speed when holding down shift.
        let speed = if keys.pressed(KeyCode::LShift) {
            speed.0 / 2.0
        } else {
            speed.0
        };

        // Calculate velocity.
        let mut velocity = Vec2::ZERO;
        if keys.pressed(KeyCode::Left) {
            velocity -= Vec2::new(speed, 0.0);
        }
        if keys.pressed(KeyCode::Right) {
            velocity += Vec2::new(speed, 0.0);
        }
        if keys.pressed(KeyCode::Up) {
            velocity += Vec2::new(0.0, speed);
        }
        if keys.pressed(KeyCode::Down) {
            velocity -= Vec2::new(0.0, speed);
        }

        // Clamp velocity to current speed.
        transform.translation += velocity.clamp_length_max(speed).extend(0.0);
    }
}
