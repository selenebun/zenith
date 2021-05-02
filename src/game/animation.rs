use bevy::core::Stopwatch;
use bevy::prelude::*;
use bevy::utils::Duration;

use crate::game::GameState;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(animate_sprites.system().label("animate_sprites")),
        )
        .add_system(
            despawn_finished_animations
                .system()
                .after("animate_sprites"),
        );
    }
}

#[derive(Debug)]
pub struct AnimationTimeLimit {
    duration: Duration,
}

#[derive(Debug)]
pub struct AnimationTimer {
    elapsed: Option<Stopwatch>,
    timer: Timer,
}

impl AnimationTimer {
    /// Create a new animation timer.
    pub fn new(seconds: f32) -> Self {
        Self {
            elapsed: None,
            timer: Timer::from_seconds(seconds, true),
        }
    }

    /// Create a new animation timer with total elapsed time.
    pub fn with_elapsed(seconds: f32) -> Self {
        Self {
            elapsed: Some(Stopwatch::new()),
            timer: Timer::from_seconds(seconds, true),
        }
    }

    /// Check whether elapsed time exceeds a given amount.
    pub fn elapsed(&self, duration: Duration) -> bool {
        match &self.elapsed {
            Some(elapsed) => elapsed.elapsed() >= duration,
            None => false,
        }
    }

    /// Check if the timer is finished.
    pub fn finished(&self) -> bool {
        self.timer.finished()
    }

    /// Tick the timer.
    pub fn tick(&mut self, delta: Duration) {
        self.elapsed.as_mut().map(|e| {
            e.tick(delta);
            e
        });
        self.timer.tick(delta);
    }
}

#[derive(Bundle)]
pub struct ExplosionBundle {
    #[bundle]
    sprite: SpriteSheetBundle,
    time_limit: AnimationTimeLimit,
    timer: AnimationTimer,
}

pub fn spawn_explosion(
    server: &AssetServer,
    audio: &Audio,
    atlases: &mut Assets<TextureAtlas>,
    transform: Transform,
) -> ExplosionBundle {
    // Play audio.
    let sound = server.load("sounds/explosion.wav");
    audio.play(sound);

    // Get texture atlas handle.
    let texture_atlas = {
        let asset = server.load("textures/explosion.png");
        let atlas = TextureAtlas::from_grid(asset, Vec2::new(96.0, 96.0), 12, 1);
        atlases.add(atlas)
    };

    // Set up animation timer.
    let timer = AnimationTimer::with_elapsed(0.1);
    let time_limit = {
        let atlas = atlases.get(&texture_atlas).unwrap();
        let frames = atlas.textures.len();
        let duration = timer.timer.duration() * frames as u32;
        AnimationTimeLimit { duration }
    };

    ExplosionBundle {
        sprite: SpriteSheetBundle {
            texture_atlas,
            transform,
            ..Default::default()
        },
        time_limit,
        timer,
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

fn despawn_finished_animations(
    mut commands: Commands,
    query: Query<(Entity, &AnimationTimeLimit, &AnimationTimer)>,
) {
    for (entity, time_limit, timer) in query.iter() {
        // Despawn if animation is finished.
        if timer.elapsed(time_limit.duration) {
            commands.entity(entity).despawn();
        }
    }
}
