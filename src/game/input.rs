use bevy::prelude::*;

use crate::game::bullet::{Bullet, FireRate};
use crate::game::player::{Player, PlayerFaction, Speed};
use crate::game::{GameState, SpriteScale};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(fire_bullets.system())
                .with_system(move_player.system().label("move_player")),
        );
    }
}

fn fire_bullets(
    mut commands: Commands,
    server: Res<AssetServer>,
    audio: Res<Audio>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    keys: Res<Input<KeyCode>>,
    scale: Res<SpriteScale>,
    time: Res<Time>,
    mut query: Query<(&mut FireRate, &Transform), With<Player>>,
) {
    for (mut fire_rate, transform) in query.iter_mut() {
        // Fire when holding Z.
        fire_rate.tick(time.delta());
        if keys.pressed(KeyCode::Z) && fire_rate.finished() {
            // Play audio.
            let sound = server.load("sounds/fire.wav");
            audio.play(sound);

            for bullet in Bullet::Small.spawn(
                &server,
                &mut materials,
                &scale,
                transform.translation.truncate(),
                90.0,
                &[0.0],
                12.0,
                1.0,
            ) {
                commands.spawn_bundle(bullet).insert(PlayerFaction);
            }
        }
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
