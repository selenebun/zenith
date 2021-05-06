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
                .with_system(move_enemies.system())
                .with_system(spawn_enemies.system()),
        );
    }
}

#[derive(Debug)]
pub enum Attack {
    Basic,
    Bomb,
}

#[derive(Debug)]
pub enum DeathBehavior {
    None,
    Star,
}

#[derive(Clone, Copy, Debug)]
pub enum Enemy {
    Basic,
    Bomber,
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
        let (atlas, attack, death_behavior, fire_rate, health, movement, radius, velocity) =
            match self {
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
                    let velocity = Velocity({
                        let speed = rng.gen_range(1.0..2.0);
                        Vec2::new(0.0, -speed)
                    });

                    (
                        atlas,
                        Attack::Basic,
                        DeathBehavior::None,
                        fire_rate,
                        1,
                        Movement::Down,
                        31.0,
                        velocity,
                    )
                }
                Self::Bomber => {
                    // Get texture atlas.
                    let atlas = {
                        let asset = server.load("textures/enemies/bomber.png");
                        TextureAtlas::from_grid(asset, Vec2::new(52.0, 31.0), 4, 1)
                    };

                    // Calculate velocity.
                    let velocity = Velocity({
                        let speed = rng.gen_range(1.5..2.0);
                        let sign = if rng.gen::<f32>() < 0.5 { 1.0 } else { -1.0 };
                        Vec2::new(speed * sign, -speed / 4.0)
                    });

                    (
                        atlas,
                        Attack::Bomb,
                        DeathBehavior::Star,
                        FireRate::Random(0.005),
                        2,
                        Movement::Strafe,
                        31.0,
                        velocity,
                    )
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
            death_behavior,
            despawn_outside: DespawnOutside,
            enemy: self,
            fire_rate,
            health: Health::new(health),
            hitbox: Hitbox { radius },
            movement,
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
    pub death_behavior: DeathBehavior,
    pub despawn_outside: DespawnOutside,
    pub enemy: Enemy,
    pub fire_rate: FireRate,
    pub health: Health,
    pub hitbox: Hitbox,
    pub movement: Movement,
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

#[derive(Debug)]
pub enum Movement {
    Down,
    Strafe,
}

fn explode_enemies(
    mut commands: Commands,
    server: Res<AssetServer>,
    audio: Res<Audio>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    scale: Res<SpriteScale>,
    mut atlases: ResMut<Assets<TextureAtlas>>,
    query: Query<(Entity, &DeathBehavior, &Health, &Transform), (With<Enemy>, Changed<Health>)>,
) {
    for (entity, death_behavior, health, transform) in query.iter() {
        // Explode once health reaches zero.
        if health.current == 0 {
            commands.entity(entity).despawn();
            commands.spawn_bundle(animation::spawn_explosion(
                &server,
                &audio,
                &mut atlases,
                *transform,
            ));

            // Execute death behavior.
            let mut rng = rand::thread_rng();
            match death_behavior {
                DeathBehavior::None => {}
                DeathBehavior::Star => {
                    // Calculate a random base angle.
                    let base_angle = rng.gen_range(0.0..60.0);
                    for bullet in Bullet::Basic.spawn(
                        &server,
                        &mut materials,
                        &scale,
                        transform.translation.truncate(),
                        Vec2::ZERO,
                        Vec2::ZERO,
                        base_angle,
                        &[0.0, 60.0, 120.0, 180.0, 240.0, 300.0],
                        8.0,
                        4.0,
                    ) {
                        commands.spawn_bundle(bullet).insert(EnemyFaction);
                    }
                }
            }
        }
    }
}

fn fire_bullets(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    scale: Res<SpriteScale>,
    time: Res<Time>,
    mut query: Query<(&Attack, &mut FireRate, &Transform, &Velocity), With<Enemy>>,
) {
    for (attack, mut fire_rate, transform, velocity) in query.iter_mut() {
        // Tick fire rate timer.
        fire_rate.tick(time.delta());
        if fire_rate.finished() {
            let bullets = match attack {
                Attack::Basic => Bullet::Basic.spawn(
                    &server,
                    &mut materials,
                    &scale,
                    transform.translation.truncate(),
                    Vec2::ZERO,
                    Vec2::ZERO,
                    -90.0,
                    &[0.0],
                    8.0,
                    4.0,
                ),
                Attack::Bomb => {
                    // Calculate base velocity.
                    let mut base_velocity = velocity.0;
                    base_velocity.y = 0.0;

                    Bullet::Bomb.spawn(
                        &server,
                        &mut materials,
                        &scale,
                        transform.translation.truncate(),
                        base_velocity,
                        Vec2::new(0.0, -0.1),
                        0.0,
                        &[0.0],
                        0.0,
                        4.0,
                    )
                }
            };

            for bullet in bullets {
                commands.spawn_bundle(bullet).insert(EnemyFaction);
            }
        }
    }
}

fn move_enemies(
    window: Res<WindowSize>,
    mut query: Query<(&Movement, &SpriteSize, &mut Transform, &mut Velocity), With<Enemy>>,
) {
    let mut rng = rand::thread_rng();
    for (movement, sprite, mut transform, mut velocity) in query.iter_mut() {
        match movement {
            Movement::Down => {}
            Movement::Strafe => {
                // Change direction when hitting wall or at random.
                let width = collision::inner_bound(window.width, sprite.width);
                if rng.gen::<f32>() < 0.002
                    || transform.translation.x > width
                    || transform.translation.x < -width
                {
                    transform.translation.x = transform.translation.x.min(width).max(-width);
                    velocity.0.x *= -1.0;
                }
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

        return;
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
