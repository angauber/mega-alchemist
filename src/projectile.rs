use crate::agent::Enemy;
use crate::assets::ItemAssets;
use crate::build::PlayerBuild;
use crate::damage::ApplyDamage;
use crate::game::{GameState, InGameSystems};
use crate::physics::started_collision_pairs;
use crate::player::Player;
use crate::weapon::Weapon::{Bow, Knife};
use crate::weapon::{KNIFE_SPRITE_INDEX, TargetReserved, Weapon};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::rapier::prelude::CollisionEventFlags;

pub struct ProjectilePlugin;

pub const ARROW_SPRITE_INDEX: usize = 253;

#[derive(Component)]
pub struct Projectile;

#[derive(Component)]
pub struct ProjectileDamage {
    pub damage: f32,
}

#[derive(Component)]
pub struct ProjectileRange {
    pub origin: Vec2,
    pub range: f32,
}

#[derive(Component)]
struct TargetTracking {
    target: Entity,
}

#[derive(Component, Default)]
struct Piercing {
    pierced: usize,
}

#[derive(Component, Default)]
struct Bounceable {
    bounces: Vec<Entity>,
}

#[derive(Event)]
struct BounceableProjectileHit {
    projectile: Entity,
    hit: Entity,
}

#[derive(Event)]
struct PiercingProjectileHit {
    projectile: Entity,
}

fn knife_projectile_sprite(item_assets: &ItemAssets) -> impl Bundle {
    Sprite::from_atlas_image(
        item_assets.sprite.clone(),
        TextureAtlas {
            layout: item_assets.texture_atlas_layout.clone(),
            index: KNIFE_SPRITE_INDEX,
        },
    )
}

fn arrow_projectile_sprite(item_assets: &ItemAssets) -> impl Bundle {
    Sprite::from_atlas_image(
        item_assets.sprite.clone(),
        TextureAtlas {
            layout: item_assets.texture_atlas_layout.clone(),
            index: ARROW_SPRITE_INDEX,
        },
    )
}

pub fn projectile(
    weapon: Weapon,
    transform: Transform,
    velocity: Velocity,
    collider: Collider,
    range: f32,
    damage: f32,
) -> impl Bundle {
    (
        Projectile,
        weapon,
        transform,
        velocity,
        collider,
        RigidBody::Dynamic,
        Sensor,
        ActiveEvents::COLLISION_EVENTS,
        ProjectileRange {
            origin: transform.translation.xy(),
            range,
        },
        ProjectileDamage { damage },
    )
}

fn throwing_knife_bundle(
    item_assets: &ItemAssets,
    transform: Transform,
    velocity: Velocity,
    collider: Collider,
    range: f32,
    damage: f32,
    target: Entity,
) -> impl Bundle {
    (
        projectile(Knife, transform, velocity, collider, range, damage),
        TargetTracking { target },
        Bounceable::default(),
        knife_projectile_sprite(item_assets),
    )
}

fn arrow_bundle(
    item_assets: &ItemAssets,
    transform: Transform,
    velocity: Velocity,
    collider: Collider,
    range: f32,
    damage: f32,
) -> impl Bundle {
    (
        projectile(Bow, transform, velocity, collider, range, damage),
        Piercing::default(),
        arrow_projectile_sprite(item_assets),
    )
}

#[derive(Event)]
pub struct ThrowProjectile {
    pub weapon_entity: Entity,
    pub target_entity: Entity,
    pub target_transform: Transform,
    pub weapon: Weapon,
    pub range: f32,
    pub speed: f32,
    pub size: f32,
    pub damage: f32,
}

fn on_throw_projectile(
    event: On<ThrowProjectile>,
    mut commands: Commands,
    player_position_query: Query<&Transform, With<Player>>,
    items_assets: Res<ItemAssets>,
) {
    info!(
        "Projectile throw triggered by weapon {} for enemy {}",
        event.weapon_entity, event.target_entity
    );

    let Ok(player_transform) = player_position_query.single() else {
        return;
    };

    let target_transform = event.target_transform;
    let projectile_range = event.range;
    let projectile_speed = event.speed;
    let projectile_size = event.size;
    let projectile_damage = event.damage;

    let projectile_direction =
        target_transform.translation.xy() - player_transform.translation.xy();
    let projectile_velocity =
        Velocity::linear(projectile_direction.normalize_or_zero() * projectile_speed);

    let mut projectile_transform = *player_transform;

    projectile_transform.translation.z = 10.0;

    let projectile_angle = Vec2::new(-1., 1.);

    projectile_transform.rotate_z(projectile_angle.angle_to(projectile_direction));

    let projectile_collider = Collider::ball(projectile_size);

    if event.weapon == Knife {
        commands.spawn((
            throwing_knife_bundle(
                &items_assets,
                projectile_transform,
                projectile_velocity,
                projectile_collider,
                projectile_range,
                projectile_damage,
                event.target_entity,
            ),
            DespawnOnEnter(GameState::InGame),
        ));

        return;
    }

    if event.weapon == Bow {
        commands.spawn((
            arrow_bundle(
                &items_assets,
                projectile_transform,
                projectile_velocity,
                projectile_collider,
                projectile_range,
                projectile_damage,
            ),
            DespawnOnEnter(GameState::InGame),
        ));

        return;
    }

    warn!("No sprite known for projectile weapon {:?}", event.weapon);
}

fn despawn_projectile_on_range_exceeded(
    mut commands: Commands,
    projectile_query: Query<(Entity, &ProjectileRange, &Transform), With<Projectile>>,
) {
    for (projectile_entity, projectile_range, projectile_position) in &projectile_query {
        if projectile_position
            .translation
            .xy()
            .distance(projectile_range.origin)
            > projectile_range.range
        {
            commands.entity(projectile_entity).despawn();
        }
    }
}

fn trigger_projectile_hit(
    mut commands: Commands,
    collision_events: MessageReader<CollisionEvent>,
    projectile_query: Query<&ProjectileDamage, With<Projectile>>,
    bounceable_query: Query<Entity, With<Bounceable>>,
    piercing_query: Query<Entity, With<Piercing>>,
    enemy_query: Query<Entity, With<Enemy>>,
) {
    for (projectile, enemy) in
        started_collision_pairs(collision_events, CollisionEventFlags::SENSOR)
    {
        if !enemy_query.contains(enemy) {
            continue;
        }

        let Ok(projectile_damage) = projectile_query.get(projectile) else {
            continue;
        };

        let damage = projectile_damage.damage;
        info!("applying {} damage to enemy {}", damage, enemy);

        commands.trigger(ApplyDamage {
            from_player: true,
            entity: enemy,
            damage,
            text_color: None,
        });

        if bounceable_query.get(projectile).is_ok() {
            commands.trigger(BounceableProjectileHit {
                projectile,
                hit: enemy,
            });
            continue;
        }

        if piercing_query.get(projectile).is_ok() {
            commands.trigger(PiercingProjectileHit { projectile });
            continue;
        }

        commands.entity(projectile).despawn();
    }
}

fn update_projectile_target_tracking(
    mut projectile_query: Query<(&mut Velocity, &mut Transform, &TargetTracking), With<Projectile>>,
    target_query: Query<&Transform, (With<Enemy>, Without<Projectile>)>,
) {
    for (mut velocity, mut transform, tracking) in projectile_query.iter_mut() {
        let Ok(target_transform) = target_query.get(tracking.target) else {
            continue;
        };

        let projectile_target = target_transform.translation.xy() - transform.translation.xy();

        let drift = projectile_target.angle_to(velocity.linear);

        *velocity =
            Velocity::linear(projectile_target.normalize_or_zero() * velocity.linear.length());
        transform.rotate_z(-drift);
    }
}

fn handle_bounceable_projectile_hit(
    event: On<BounceableProjectileHit>,
    mut bounceable_projectile_query: Query<
        (
            &mut TargetTracking,
            &mut Bounceable,
            &mut ProjectileRange,
            &Weapon,
            &Transform,
        ),
        With<Projectile>,
    >,
    enemies_query: Query<(Entity, &Transform), (With<Enemy>, Without<TargetReserved>)>,
    player_build: Res<PlayerBuild>,
    mut commands: Commands,
) {
    let Ok((mut tracking, mut bounceable, mut projectile_range, weapon, projectile_transform)) =
        bounceable_projectile_query.get_mut(event.projectile)
    else {
        return;
    };

    let max_bounces: usize = player_build.weapon_bounces(weapon).unwrap_or_default();

    if bounceable.bounces.len() >= max_bounces {
        commands.entity(event.projectile).despawn();
        return;
    }

    let projectile_position = projectile_transform.translation.xy();

    let mut in_range_enemies = enemies_query
        .iter()
        .filter(|(enemy_entity, _enemy_transform)| {
            *enemy_entity != event.hit && !bounceable.bounces.contains(enemy_entity)
        })
        .filter(|(_enemy_entity, enemy_transform)| {
            projectile_position.distance(enemy_transform.translation.xy()) <= projectile_range.range
        })
        .collect::<Vec<_>>();

    in_range_enemies.sort_by(|a, b| {
        let enemy_a = projectile_position.distance(a.1.translation.xy());
        let enemy_b = projectile_position.distance(b.1.translation.xy());

        enemy_a.total_cmp(&enemy_b)
    });

    let Some((closest_enemy, _closest_enemy_pos)) = in_range_enemies.first() else {
        commands.entity(event.projectile).despawn();
        return;
    };

    let origin_target_distance = projectile_range
        .origin
        .distance(projectile_transform.translation.xy());

    projectile_range.range -= origin_target_distance;
    projectile_range.origin = projectile_transform.translation.xy();

    tracking.target = *closest_enemy;

    bounceable.bounces.push(event.hit);
}

fn handle_piercing_projectile_hit(
    event: On<PiercingProjectileHit>,
    mut piercing_query: Query<(&mut Piercing, &Weapon), With<Projectile>>,
    player_build: Res<PlayerBuild>,
    mut commands: Commands,
) {
    let Ok((mut piercing, weapon)) = piercing_query.get_mut(event.projectile) else {
        return;
    };

    let max_pierce: usize = player_build.weapon_piercing(weapon).unwrap_or_default();

    if piercing.pierced >= max_pierce {
        commands.entity(event.projectile).despawn();
    }

    piercing.pierced += 1;
}

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_throw_projectile)
            .add_observer(handle_bounceable_projectile_hit)
            .add_observer(handle_piercing_projectile_hit)
            .add_systems(
                Update,
                (
                    trigger_projectile_hit,
                    despawn_projectile_on_range_exceeded,
                    update_projectile_target_tracking,
                )
                    .in_set(InGameSystems),
            );
    }
}
