use bevy::prelude::*;

use crate::game::starfield::Star;
use crate::game::WindowSize;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(wrap_stars.system());
    }
}

/// Get the outer bound for a sprite within a region.
pub fn outer_bound(dimension: f32, sprite: f32) -> f32 {
    (dimension + sprite) / 2.0
}

fn wrap_stars(window: Res<WindowSize>, mut query: Query<(&Sprite, &mut Transform), With<Star>>) {
    for (sprite, mut transform) in query.iter_mut() {
        let height = outer_bound(window.height, sprite.size.y * transform.scale.y);
        if transform.translation.y < -height {
            transform.translation.y = height;
        }
    }
}
