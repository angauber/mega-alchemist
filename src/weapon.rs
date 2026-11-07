use crate::agent::Enemy;
use crate::build::{PlayerBuild, Statistic, StatisticData};
use crate::game::InGameSystems;
use crate::player::Player;
use crate::projectile::{ProjectilePlugin, ThrowProjectile};
use crate::weapon::Weapon::{Bow, Knife};
use bevy::prelude::*;
use std::collections::HashMap;
use std::time::Duration;

pub struct WeaponPlugin;

#[derive(Component, Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Weapon {
    Knife,
    Bow,
}

impl Weapon {
    pub fn all() -> Vec<Weapon> {
        vec![Knife, Bow]
    }
}

#[derive(Component)]
pub struct ProjectileWeapon;

#[derive(Component)]
pub struct WeaponCooldown {
    pub cooldown: Timer,
}

impl From<Duration> for WeaponCooldown {
    fn from(value: Duration) -> Self {
        WeaponCooldown {
            cooldown: Timer::new(value, TimerMode::Once),
        }
    }
}

#[derive(Component)]
pub struct WeaponProjectileCounter {
    pub count: usize,
}

#[derive(Component)]
pub struct TargetReserved {
    timer: Timer,
}

impl Default for TargetReserved {
    fn default() -> Self {
        TargetReserved {
            timer: Timer::new(Duration::from_millis(100), TimerMode::Once),
        }
    }
}

#[derive(Event)]
pub struct SpawnWeapon(pub Weapon);

pub fn projectile_weapon_build(
    weapon_range: f32,
    weapon_cooldown: Duration,
    weapon_damage: f32,
    projectile_size: f32,
    projectile_speed: f32,
) -> HashMap<Statistic, StatisticData> {
    vec![
        (
            Statistic::ProjectileCount,
            StatisticData::from_base_value(1.0),
        ),
        (
            Statistic::Range,
            StatisticData::from_base_value(weapon_range),
        ),
        (
            Statistic::AttackSpeed,
            StatisticData::from_base_value(weapon_cooldown.as_millis() as f32),
        ),
        (
            Statistic::Damage,
            StatisticData::from_base_value(weapon_damage),
        ),
        (
            Statistic::ProjectileSize,
            StatisticData::from_base_value(projectile_size),
        ),
        (
            Statistic::ProjectileSpeed,
            StatisticData::from_base_value(projectile_speed),
        ),
    ]
    .into_iter()
    .collect()
}

// high attack speed, low range and damage
pub fn knife_build() -> HashMap<Statistic, StatisticData> {
    projectile_weapon_build(100.0, Duration::from_millis(1500), 10.0, 2.0, 150.0)
}

// High range and damage low attack speed
pub fn bow_build() -> HashMap<Statistic, StatisticData> {
    projectile_weapon_build(170.0, Duration::from_millis(1900), 15.0, 2.0, 200.0)
}

pub fn projectile_weapon() -> impl Bundle {
    (ProjectileWeapon, WeaponProjectileCounter { count: 0 })
}

fn knife() -> impl Bundle {
    (projectile_weapon(), Knife)
}

fn bow() -> impl Bundle {
    (projectile_weapon(), Bow)
}

fn on_spawn_weapon(
    event: On<SpawnWeapon>,
    player_query: Query<Entity, With<Player>>,
    mut player_build: ResMut<PlayerBuild>,
    mut commands: Commands,
) {
    let new_weapon = event.0;

    let Ok(player_entity) = player_query.single() else {
        return;
    };

    player_build.add_weapon(new_weapon);

    commands
        .entity(player_entity)
        .with_children(|parent| match new_weapon {
            Knife => {
                parent.spawn(knife());
            }
            Bow => {
                parent.spawn(bow());
            }
        });

    info!("new weapon spawned {:?}", new_weapon);
}

fn weapon_cooldown_system(
    mut commands: Commands,
    mut weapon_cooldown_query: Query<(Entity, &mut WeaponCooldown), With<Weapon>>,
    time: Res<Time>,
) {
    for (weapon, mut weapon_cooldown) in weapon_cooldown_query.iter_mut() {
        weapon_cooldown.cooldown.tick(time.delta());

        if weapon_cooldown.cooldown.is_finished() {
            commands.entity(weapon).remove::<WeaponCooldown>();

            continue;
        };
    }
}

fn clear_reserved_targets(
    mut commands: Commands,
    mut reserved_targets: Query<(Entity, &mut TargetReserved)>,
    time: Res<Time>,
) {
    for (entity, mut target) in reserved_targets.iter_mut() {
        target.timer.tick(time.delta());

        if target.timer.is_finished() {
            commands.entity(entity).remove::<TargetReserved>();
        }
    }
}

fn throw_projectiles(
    mut commands: Commands,
    player_position: Query<&Transform, With<Player>>,
    weapon_query: Query<(Entity, &Weapon), (With<ProjectileWeapon>, Without<WeaponCooldown>)>,
    mut projectile_counter_query: Query<&mut WeaponProjectileCounter, With<Weapon>>,
    enemies_query: Query<(Entity, &Transform), (With<Enemy>, Without<TargetReserved>)>,
    player_build: Res<PlayerBuild>,
) {
    let Ok(player_transform) = player_position.single() else {
        return;
    };
    let player_position = player_transform.translation.xy();

    for (weapon_entity, weapon) in weapon_query {
        let Ok(mut projectile_counter) = projectile_counter_query.get_mut(weapon_entity) else {
            continue;
        };

        let Ok(cooldown_duration) = player_build.weapon_cooldown(weapon) else {
            warn!("cooldown missing for {:?}", weapon);
            continue;
        };

        let Ok(projectile_count) = player_build.weapon_projectile_count(weapon) else {
            warn!("projectile count missing for {:?}", weapon);
            continue;
        };

        let Ok(range) = player_build.weapon_range(weapon) else {
            warn!("range missing for {:?}", weapon);
            continue;
        };

        let Ok(speed) = player_build.weapon_projectile_speed(weapon) else {
            warn!("projectile speed missing for {:?}", weapon);
            continue;
        };

        let Ok(size) = player_build.weapon_projectile_size(weapon) else {
            warn!("projectile size missing for {:?}", weapon);
            continue;
        };
        let Ok(damage) = player_build.weapon_damage(weapon) else {
            warn!("damage missing for {:?}", weapon);
            continue;
        };

        let mut in_range_enemies = enemies_query
            .iter()
            .filter(|(_enemy_entity, enemy_transform)| {
                player_position.distance(enemy_transform.translation.xy()) < range
            })
            .collect::<Vec<_>>();

        in_range_enemies.sort_by(|a, b| {
            let player_a = player_position.distance(a.1.translation.xy());
            let player_b = player_position.distance(b.1.translation.xy());

            player_a.total_cmp(&player_b)
        });

        'enemy: for (enemy, enemy_transform) in in_range_enemies {
            commands.trigger(ThrowProjectile {
                weapon_entity,
                target_entity: enemy,
                target_transform: *enemy_transform,
                weapon: *weapon,
                range,
                speed,
                size,
                damage,
            });

            commands.entity(enemy).insert(TargetReserved::default());

            projectile_counter.count += 1;

            if projectile_counter.count == projectile_count {
                commands
                    .entity(weapon_entity)
                    .insert(WeaponCooldown::from(cooldown_duration));

                projectile_counter.count = 0;

                break 'enemy;
            }
        }
    }
}

fn spawn_player_default_weapon(mut commands: Commands, player_query: Query<Entity, Added<Player>>) {
    if player_query.single().is_err() {
        return;
    };

    commands.trigger(SpawnWeapon(Knife));
}

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ProjectilePlugin)
            .add_observer(on_spawn_weapon)
            .add_systems(PreUpdate, weapon_cooldown_system.in_set(InGameSystems))
            .add_systems(
                Update,
                (
                    spawn_player_default_weapon,
                    throw_projectiles,
                    clear_reserved_targets,
                )
                    .in_set(InGameSystems),
            );
    }
}
