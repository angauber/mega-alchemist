use crate::agent::Enemy;
use crate::build::PlayerBuild;
use crate::game::InGameSystems;
use crate::player::Player;
use crate::projectile::{ProjectilePlugin, ThrowProjectile};
use bevy::prelude::*;
use std::time::Duration;
pub struct WeaponPlugin;

fn global_multiplier(a: f32, b: f32) -> f32 {
    1. + (a - 1.) + (b - 1.)
}

#[derive(Debug)]
pub enum WeaponType {
    Knife,
}

#[derive(Component)]
pub struct Weapon;

#[derive(Component)]
pub struct ThrowableWeapon;

#[derive(Component)]
pub struct WeaponCooldown {
    pub cooldown: Timer,
}

#[derive(Component)]
pub struct WeaponRange {
    pub range: f32,
    pub multiplier: f32,
}

#[derive(Component)]
pub struct WeaponAttackSpeed {
    pub duration: Duration,
    pub multiplier: f32,
}

impl WeaponAttackSpeed {
    pub fn cooldown(&self, player_attack_speed: f32) -> WeaponCooldown {
        WeaponCooldown {
            cooldown: Timer::new(
                self.duration
                    .div_f32(global_multiplier(self.multiplier, player_attack_speed)),
                TimerMode::Once,
            ),
        }
    }
}

#[derive(Component)]
pub struct WeaponDamage {
    pub damage: f32,
    pub multiplier: f32,
}

#[derive(Component)]
pub struct WeaponProjectileSize {
    pub size: f32,
    pub multiplier: f32,
}

#[derive(Component)]
pub struct WeaponProjectileSpeed {
    pub speed: f32,
    pub multiplier: f32,
}

#[derive(Component)]
pub struct WeaponProjectileCount {
    pub total: usize,
    pub used: usize,
}

pub fn throwable_weapon(
    weapon_range: f32,
    weapon_cooldown: Duration,
    weapon_damage: f32,
    projectile_size: f32,
    projectile_speed: f32,
) -> impl Bundle {
    (
        Weapon,
        ThrowableWeapon,
        WeaponRange {
            range: weapon_range,
            multiplier: 1.0,
        },
        WeaponAttackSpeed {
            duration: weapon_cooldown,
            multiplier: 1.0,
        },
        WeaponDamage {
            damage: weapon_damage,
            multiplier: 1.0,
        },
        WeaponProjectileCount { total: 1, used: 0 },
        WeaponProjectileSize {
            size: projectile_size,
            multiplier: 1.0,
        },
        WeaponProjectileSpeed {
            speed: projectile_speed,
            multiplier: 1.0,
        },
    )
}

fn knife() -> impl Bundle {
    throwable_weapon(200.0, Duration::from_millis(1500), 10.0, 2.0, 150.0)
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

fn throw_projectiles(
    mut commands: Commands,
    player_position: Query<&Transform, With<Player>>,
    weapon_query: Query<
        (
            Entity,
            &WeaponRange,
            &WeaponDamage,
            &WeaponProjectileSpeed,
            &WeaponProjectileSize,
            &WeaponAttackSpeed,
        ),
        (With<Weapon>, Without<WeaponCooldown>),
    >,
    mut weapon_projectile_count_query: Query<&mut WeaponProjectileCount, With<Weapon>>,
    enemies_query: Query<(Entity, &Transform), With<Enemy>>,
    player_build: Res<PlayerBuild>,
) {
    let Ok(player_transform) = player_position.single() else {
        return;
    };
    let player_position = player_transform.translation.xy();

    for (weapon, weapon_range, weapon_damage, projectile_speed, projectile_size, attack_speed) in
        weapon_query
    {
        let Ok(mut projectile_count) = weapon_projectile_count_query.get_mut(weapon) else {
            continue;
        };

        let range =
            weapon_range.range * global_multiplier(weapon_range.multiplier, player_build.range);
        let speed = projectile_speed.speed
            * global_multiplier(projectile_speed.multiplier, player_build.projectile_speed);
        let size = projectile_size.size
            * global_multiplier(projectile_size.multiplier, player_build.projectile_size);
        let damage =
            weapon_damage.damage * global_multiplier(weapon_damage.multiplier, player_build.damage);

        let count = projectile_count.total + player_build.projectile_count;

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
                weapon,
                target: enemy,
                target_transform: *enemy_transform,
                range,
                speed,
                size,
                damage,
            });

            projectile_count.used += 1;

            if projectile_count.used == count {
                commands
                    .entity(weapon)
                    .insert(attack_speed.cooldown(player_build.attack_speed));

                projectile_count.used = 0;

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
        app.add_plugins(ProjectilePlugin)
            .add_systems(PreUpdate, weapon_cooldown_system.in_set(InGameSystems))
            .add_systems(
                Update,
                (spawn_player_weapon, throw_projectiles).in_set(InGameSystems),
            );
    }
}
