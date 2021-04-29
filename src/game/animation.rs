use bevy::prelude::*;
use bevy::utils::Duration;

use crate::game::GameState;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_update(GameState::Playing).with_system(animate_sprites.system()),
        );
    }
}

#[derive(Debug)]
pub struct AnimationTimer {
    timer: Timer,
}

impl AnimationTimer {
    /// Create a new animation timer.
    pub fn new(seconds: f32) -> Self {
        Self {
            timer: Timer::from_seconds(seconds, true),
        }
    }

    /// Check if the timer is finished.
    pub fn finished(&self) -> bool {
        self.timer.finished()
    }

    /// Tick the timer.
    pub fn tick(&mut self, delta: Duration) {
        self.timer.tick(delta);
    }
}

fn animate_sprites(
    atlases: Res<Assets<TextureAtlas>>,
    time: Res<Time>,
    mut query: Query<(
        &mut AnimationTimer,
        &Handle<TextureAtlas>,
        &mut TextureAtlasSprite,
    )>,
) {
    for (mut timer, handle, mut sprite) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            let atlas = atlases.get(handle).unwrap();
            sprite.index = ((sprite.index as usize + 1) % atlas.textures.len()) as u32;
        }
    }
}
