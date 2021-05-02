use bevy::prelude::*;
use rand::prelude::*;

use crate::game::animation::{self, AnimationTimer};
use crate::game::bullet::{Bullet, FireRate};
use crate::game::collision::{self, DespawnOutside, Hitbox, SpriteSize};
use crate::game::level::{CurrentLevel, EnemiesLeft, Level, SpawnTimer};
use crate::game::physics::Velocity;
use crate::game::{GameState, SpriteScale, WindowSize};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(explode_enemies.system())
                .with_system(fire_bullets.system())
                .with_system(spawn_enemies.system()),
        );
    }
}

#[derive(Debug)]
pub enum Attack {
    Basic,
}

#[derive(Clone, Copy, Debug)]
pub enum Enemy {
    Basic,
}

impl Enemy {
    /// Spawn a single enemy.
    pub fn spawn_single(
        self,
        server: &AssetServer,
        scale: &SpriteScale,
        atlases: &mut Assets<TextureAtlas>,
        window: &WindowSize,
    ) -> EnemyBundle {
        let mut rng = rand::thread_rng();
        let (atlas, attack, fire_rate, health, radius, velocity) = match self {
            Self::Basic => {
                // Get texture atlas.
                let atlas = {
                    let asset = server.load("textures/enemies/basic.png");
                    TextureAtlas::from_grid(asset, Vec2::new(50.0, 43.0), 4, 1)
                };

                // Calculate fire rate.
                let fire_rate = {
                    let seconds = rng.gen_range(0.4..0.55);
                    FireRate::from_seconds(seconds)
                };

                // Calculate velocity.
                let velocity = {
                    let speed = rng.gen_range(1.0..2.0);
                    Velocity(Vec2::new(0.0, -speed))
                };

                (atlas, Attack::Basic, fire_rate, 1, 31.0, velocity)
            }
        };

        // Get texture atlas handle.
        let texture_atlas = atlases.add(atlas);

        // Get texture dimensions.
        let sprite_size = {
            let atlas = atlases.get(&texture_atlas).unwrap();
            let rect = atlas.textures.first().unwrap();
            SpriteSize::new(rect.width(), rect.height(), scale.scale)
        };

        // Calculate transform.
        let transform = {
            let width = collision::inner_bound(window.width, sprite_size.width);
            let height = collision::outer_bound(window.height, sprite_size.height);
            scale.xyz(rng.gen_range(-width..width), height, 2.0)
        };

        EnemyBundle {
            attack,
            despawn_outside: DespawnOutside,
            enemy: self,
            fire_rate,
            health: Health::new(health),
            hitbox: Hitbox { radius },
            sprite: SpriteSheetBundle {
                texture_atlas,
                transform,
                ..Default::default()
            },
            sprite_size,
            timer: AnimationTimer::new(0.1),
            velocity,
        }
    }
}

#[derive(Bundle)]
pub struct EnemyBundle {
    pub attack: Attack,
    pub despawn_outside: DespawnOutside,
    pub enemy: Enemy,
    pub fire_rate: FireRate,
    pub health: Health,
    pub hitbox: Hitbox,
    #[bundle]
    pub sprite: SpriteSheetBundle,
    pub sprite_size: SpriteSize,
    pub timer: AnimationTimer,
    pub velocity: Velocity,
}

#[derive(Debug)]
pub struct EnemyFaction;

#[derive(Debug)]
pub struct Health {
    pub current: u32,
}

impl Health {
    /// Start at max health.
    pub fn new(amount: u32) -> Self {
        Self { current: amount }
    }

    /// Subtract damage from current health.
    pub fn damage(&mut self, amount: u32) {
        self.current = self.current.saturating_sub(amount);
    }
}

fn explode_enemies(
    mut commands: Commands,
    server: Res<AssetServer>,
    audio: Res<Audio>,
    mut atlases: ResMut<Assets<TextureAtlas>>,
    query: Query<(Entity, &Health, &Transform)>,
) {
    for (entity, health, transform) in query.iter() {
        // Explode once health reaches zero.
        if health.current == 0 {
            commands.entity(entity).despawn();
            commands.spawn_bundle(animation::spawn_explosion(
                &server,
                &audio,
                &mut atlases,
                *transform,
            ));
        }
    }
}

fn fire_bullets(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    scale: Res<SpriteScale>,
    time: Res<Time>,
    mut query: Query<(&Attack, &mut FireRate, &Transform), With<Enemy>>,
) {
    for (attack, mut fire_rate, transform) in query.iter_mut() {
        // Tick fire rate timer.
        fire_rate.tick(time.delta());
        if fire_rate.finished() {
            let bullets = match attack {
                Attack::Basic => Bullet::Basic.spawn(
                    &server,
                    &mut materials,
                    &scale,
                    transform.translation.truncate(),
                    -90.0,
                    &[0.0],
                    8.0,
                    4.0,
                ),
            };

            for bullet in bullets {
                commands.spawn_bundle(bullet).insert(EnemyFaction);
            }
        }
    }
}

fn spawn_enemies(
    mut commands: Commands,
    server: Res<AssetServer>,
    levels: Res<Vec<Level>>,
    scale: Res<SpriteScale>,
    mut atlases: ResMut<Assets<TextureAtlas>>,
    time: Res<Time>,
    window: Res<WindowSize>,
    mut query: Query<(&mut CurrentLevel, &mut EnemiesLeft, &mut SpawnTimer)>,
) {
    let (mut current, mut enemies_left, mut timer) =
        query.single_mut().expect("expected a single level");
    // Skip spawning if there are no more levels.
    let (mut index, level) = match current.level {
        Some(index) => (index, &levels[index]),
        None => return,
    };

    // Continue to next level once all enemies have been spawned.
    if enemies_left.count == 0 {
        index += 1;
        if index < levels.len() {
            current.level = Some(index);
            enemies_left.count = levels[index].enemy_limit;
        } else {
            current.level = None;
        }
    }

    // Tick spawn timer.
    timer.tick(time.delta());
    if timer.finished() {
        // Decrement enemies left.
        enemies_left.count -= 1;

        // Choose a random enemy to spawn.
        let mut rng = rand::thread_rng();
        let enemy = level.enemies.choose_weighted(&mut rng, |e| e.1).unwrap().0;
        commands.spawn_bundle(enemy.spawn_single(&server, &scale, &mut atlases, &window));

        // Reset spawn timer with a random duration.
        let delay = rng.gen_range(level.delay.clone());
        timer.reset(delay);
    }
}
