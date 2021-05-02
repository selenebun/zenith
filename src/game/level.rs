use std::ops::Range;

use bevy::prelude::*;
use bevy::utils::Duration;

use crate::game::enemy::Enemy;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system());
    }
}

#[derive(Debug)]
pub struct CurrentLevel {
    pub level: Option<usize>,
}

#[derive(Bundle)]
pub struct CurrentLevelBundle {
    pub enemies_left: EnemiesLeft,
    pub level: CurrentLevel,
    pub timer: SpawnTimer,
}

#[derive(Debug)]
pub struct EnemiesLeft {
    pub count: u32,
}

#[derive(Debug)]
pub struct SpawnTimer {
    pub timer: Timer,
}

impl SpawnTimer {
    /// Create a new spawn timer.
    pub fn new(seconds: f32) -> Self {
        Self {
            timer: Timer::from_seconds(seconds, false),
        }
    }

    /// Check if the timer is finished.
    pub fn finished(&self) -> bool {
        self.timer.finished()
    }

    /// Reset the timer.
    pub fn reset(&mut self, millis: u64) {
        self.timer.set_duration(Duration::from_millis(millis));
        self.timer.reset();
    }

    /// Tick the timer.
    pub fn tick(&mut self, delta: Duration) {
        self.timer.tick(delta);
    }
}

#[derive(Debug)]
pub struct Level {
    pub delay: Range<u64>,
    pub enemies: Vec<(Enemy, u32)>,
    pub enemy_limit: u32,
}

fn setup(mut commands: Commands) {
    // Initialize levels.
    let levels = vec![Level {
        delay: 800..3200,
        enemies: vec![(Enemy::Basic, 1)],
        enemy_limit: 10,
    }];

    // Initialize current level data.
    commands.spawn_bundle(CurrentLevelBundle {
        enemies_left: EnemiesLeft {
            count: levels[0].enemy_limit,
        },
        level: CurrentLevel { level: Some(0) },
        timer: SpawnTimer::new(1.0),
    });

    commands.insert_resource(levels);
}
