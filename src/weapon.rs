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
pub struct WeaponRange {
    pub range: f32,
}

#[derive(Component)]
pub struct WeaponCooldown {
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
pub struct WeaponProjectileRange {
    pub range: f32,
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

#[derive(Bundle)]
pub struct ThrowingWeaponBundle {
    weapon: Weapon,
    range: WeaponRange,
    cooldown: WeaponCooldown,
    damage: WeaponDamage,
    projectile_range: WeaponProjectileRange,
    projectile_count: WeaponProjectileCount,
    projectile_size: WeaponProjectileSize,
    projectile_speed: WeaponProjectileSpeed,
}

fn throw_projectiles(
    mut commands: Commands,
    player_position: Query<&Transform, With<Player>>,
    weapon_query: Query<Entity, With<Weapon>>,
    weapon_range_query: Query<&WeaponRange, With<Weapon>>,
    mut weapon_projectile_count_query: Query<&mut WeaponProjectileCount, With<Weapon>>,
    mut weapon_cooldown_query: Query<&mut WeaponCooldown, With<Weapon>>,
    enemies_query: Query<(Entity, &Transform), With<Enemy>>,
    time: Res<Time>,
) {
    let Ok(player_position) = player_position.single() else {
        return;
    };

    for weapon in weapon_query {
        let Ok(mut weapon_cooldown) = weapon_cooldown_query.get_mut(weapon) else {
            continue;
        };

        weapon_cooldown.cooldown.tick(time.delta());

        if !weapon_cooldown.cooldown.is_finished() {
            continue;
        };

        let weapon_range = weapon_range_query.get(weapon).unwrap();
        let mut projectile_count = weapon_projectile_count_query.get_mut(weapon).unwrap();

        let in_range_enemies = enemies_query
            .iter()
            .filter(|(_enemy_entity, enemy_transform)| {
                player_position
                    .translation
                    .distance(enemy_transform.translation)
                    < weapon_range.range
            });

        'enemy: for (enemy, enemy_transform) in in_range_enemies {
            commands.trigger(ThrowProjectile {
                weapon,
                target: enemy,
                target_transform: *enemy_transform,
            });

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

    commands
        .entity(player_entity)
        .with_child(ThrowingWeaponBundle {
            weapon: Weapon,
            range: WeaponRange { range: 200. },
            cooldown: WeaponCooldown {
                cooldown: Timer::new(Duration::from_millis(750), TimerMode::Once),
            },
            damage: WeaponDamage { damage: 5.0 },
            projectile_range: WeaponProjectileRange { range: 100.0 },
            projectile_count: WeaponProjectileCount { max: 1, count: 0 },
            projectile_size: WeaponProjectileSize { size: 10.0 },
            projectile_speed: WeaponProjectileSpeed { speed: 300.0 },
        });
}

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ProjectilePlugin).add_systems(
            Update,
            (spawn_player_weapon, throw_projectiles).in_set(InGameSystems),
        );
    }
}
