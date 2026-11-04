use crate::agent::Enemy;
use crate::game::InGameSystems;
use crate::player::Player;
use crate::projectile::{ProjectilePlugin, ThrowProjectile};
use bevy::prelude::*;
use std::time::Duration;

pub struct WeaponPlugin;

#[derive(Component)]
pub struct Weapon;

#[derive(Component)]
pub struct ThrowableWeapon;

#[derive(Component)]
pub struct WeaponRange {
    pub range: f32,
}

#[derive(Component)]
pub struct WeaponCooldown {
    pub minimum: Timer,
    pub cooldown: Timer,
}

#[derive(Component)]
pub struct WeaponDamage {
    pub damage: f32,
}

#[derive(Component)]
pub struct WeaponProjectileSize {
    pub size: f32,
}

#[derive(Component)]
pub struct WeaponProjectileSpeed {
    pub speed: f32,
}

#[derive(Component)]
pub struct WeaponProjectileCount {
    pub max: usize,
    pub count: usize,
}

pub fn throwable_weapon(
    weapon_range: f32,
    weapon_cooldown: Duration,
    weapon_damage: f32,
    projectile_count: usize,
    projectile_size: f32,
    projectile_speed: f32,
) -> impl Bundle {
    (
        Weapon,
        ThrowableWeapon,
        WeaponRange {
            range: weapon_range,
        },
        WeaponCooldown {
            minimum: Timer::new(Duration::from_millis(80), TimerMode::Once),
            cooldown: Timer::new(weapon_cooldown, TimerMode::Once),
        },
        WeaponDamage {
            damage: weapon_damage,
        },
        WeaponProjectileCount {
            max: projectile_count,
            count: 0,
        },
        WeaponProjectileSize {
            size: projectile_size,
        },
        WeaponProjectileSpeed {
            speed: projectile_speed,
        },
    )
}

fn knife() -> impl Bundle {
    throwable_weapon(200.0, Duration::from_millis(1000), 10.0, 1, 5.0, 150.0)
}

fn throw_projectiles(
    mut commands: Commands,
    player_position: Query<&Transform, With<Player>>,
    weapon_query: Query<(Entity, &WeaponRange, &WeaponDamage), With<Weapon>>,
    mut weapon_projectile_count_query: Query<&mut WeaponProjectileCount, With<Weapon>>,
    mut weapon_cooldown_query: Query<&mut WeaponCooldown, With<Weapon>>,
    enemies_query: Query<(Entity, &Transform), With<Enemy>>,
    time: Res<Time>,
) {
    let Ok(player_transform) = player_position.single() else {
        return;
    };
    let player_position = player_transform.translation.xy();

    for (weapon, weapon_range, weapon_damage) in weapon_query {
        let Ok(mut weapon_cooldown) = weapon_cooldown_query.get_mut(weapon) else {
            continue;
        };

        weapon_cooldown.minimum.tick(time.delta());
        weapon_cooldown.cooldown.tick(time.delta());

        if !weapon_cooldown.cooldown.is_finished() {
            continue;
        };
        if !weapon_cooldown.minimum.is_finished() {
            continue;
        };

        let mut projectile_count = weapon_projectile_count_query.get_mut(weapon).unwrap();

        let mut in_range_enemies = enemies_query
            .iter()
            .filter(|(_enemy_entity, enemy_transform)| {
                player_position.distance(enemy_transform.translation.xy()) < weapon_range.range
            })
            .collect::<Vec<_>>();

        in_range_enemies.sort_by(|a, b| {
            let player_a = player_position.distance(a.1.translation.xy());
            let player_b = player_position.distance(b.1.translation.xy());

            player_a.total_cmp(&player_b)
        });

        'enemy: for (enemy, enemy_transform) in in_range_enemies {
            commands.trigger(ThrowProjectile {
                weapon,
                target: enemy,
                target_transform: *enemy_transform,
                range: weapon_range.range,
                damage: weapon_damage.damage,
            });
            weapon_cooldown.minimum.reset();

            projectile_count.count += 1;

            if projectile_count.count == projectile_count.max {
                weapon_cooldown.cooldown.reset();
                projectile_count.count = 0;

                break 'enemy;
            }
        }
    }
}

fn spawn_player_weapon(mut commands: Commands, player_query: Query<Entity, Added<Player>>) {
    let Ok(player_entity) = player_query.single() else {
        return;
    };

    commands.entity(player_entity).with_child(knife());
}

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ProjectilePlugin).add_systems(
            Update,
            (spawn_player_weapon, throw_projectiles).in_set(InGameSystems),
        );
    }
}
