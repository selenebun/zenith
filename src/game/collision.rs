use bevy::prelude::*;

use crate::game::bullet::{Bullet, Damage};
use crate::game::enemy::{Enemy, Health};
use crate::game::player::{Player, PlayerFaction};
use crate::game::starfield::Star;
use crate::game::{GameState, WindowSize};

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(bound_player.system().after("move_player"))
                .with_system(collide_with_player_bullets.system()),
        )
        .add_system(despawn_outside.system())
        .add_system(wrap_stars.system());
    }
}

#[derive(Debug)]
pub struct DespawnOutside;

#[derive(Debug)]
pub struct Hitbox {
    pub radius: f32,
}

#[derive(Debug)]
pub struct SpriteSize {
    pub width: f32,
    pub height: f32,
}

impl SpriteSize {
    /// Calculate the sprite size.
    pub fn new(width: f32, height: f32, scale: f32) -> Self {
        Self {
            width: width * scale,
            height: height * scale,
        }
    }
}

/// Get the inner bound for a sprite within a region.
pub fn inner_bound(dimension: f32, sprite: f32) -> f32 {
    (dimension - sprite) / 2.0
}

/// Get the outer bound for a sprite within a region.
pub fn outer_bound(dimension: f32, sprite: f32) -> f32 {
    (dimension + sprite) / 2.0
}

fn bound_player(
    window: Res<WindowSize>,
    mut query: Query<(&SpriteSize, &mut Transform), With<Player>>,
) {
    for (sprite, mut transform) in query.iter_mut() {
        let width = inner_bound(window.width, sprite.width);
        let height = inner_bound(window.height, sprite.height);
        transform.translation.x = transform.translation.x.min(width).max(-width);
        transform.translation.y = transform.translation.y.min(height).max(-height);
    }
}

fn collide_with_player_bullets(
    mut commands: Commands,
    bullets: Query<(Entity, &Damage, &Hitbox, &Transform), (With<Bullet>, With<PlayerFaction>)>,
    mut enemies: Query<(&mut Health, &Hitbox, &Transform), With<Enemy>>,
) {
    for (mut health, enemy_hitbox, enemy_transform) in enemies.iter_mut() {
        for (entity, damage, hitbox, transform) in bullets.iter() {
            // Check for collision.
            let distance = enemy_transform
                .translation
                .truncate()
                .distance_squared(transform.translation.truncate());
            let radius_sum = enemy_hitbox.radius + hitbox.radius;
            if distance < radius_sum * radius_sum {
                commands.entity(entity).despawn();
                health.damage(damage.0);
            }
        }
    }
}

fn despawn_outside(
    mut commands: Commands,
    window: Res<WindowSize>,
    sprite_sheets: Query<(Entity, &SpriteSize, &Transform), With<DespawnOutside>>,
    sprites: Query<(Entity, &Sprite, &Transform), With<DespawnOutside>>,
) {
    for (entity, sprite, transform) in sprite_sheets.iter() {
        let width = outer_bound(window.width, sprite.width) + 12.0;
        let height = outer_bound(window.height, sprite.height) + 12.0;
        if transform.translation.x > width
            || transform.translation.x < -width
            || transform.translation.y > height
            || transform.translation.y < -height
        {
            commands.entity(entity).despawn();
        }
    }

    for (entity, sprite, transform) in sprites.iter() {
        let width = outer_bound(window.width, sprite.size.x * transform.scale.x) + 12.0;
        let height = outer_bound(window.height, sprite.size.y * transform.scale.y) + 12.0;
        if transform.translation.x > width
            || transform.translation.x < -width
            || transform.translation.y > height
            || transform.translation.y < -height
        {
            commands.entity(entity).despawn();
        }
    }
}

fn wrap_stars(window: Res<WindowSize>, mut query: Query<(&Sprite, &mut Transform), With<Star>>) {
    for (sprite, mut transform) in query.iter_mut() {
        let height = outer_bound(window.height, sprite.size.y * transform.scale.y);
        if transform.translation.y < -height {
            transform.translation.y = height;
        }
    }
}
