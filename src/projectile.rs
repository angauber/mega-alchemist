use crate::agent::Enemy;
use crate::asset_tracking::LoadResource;
use crate::damage::ApplyDamage;
use crate::game::InGameSystems;
use crate::player::Player;
use crate::sprite_assets::ItemAssets;
use crate::weapon::{Weapon, WeaponProjectileSize, WeaponProjectileSpeed};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::rapier::prelude::CollisionEventFlags;

pub struct ProjectilePlugin;

const KNIFE_SPRITE_INDEX: usize = 0;

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

pub fn knife_sprite(item_assets: &ItemAssets) -> Sprite {
    Sprite::from_atlas_image(
        item_assets.sprite.clone(),
        TextureAtlas {
            layout: item_assets.texture_atlas_layout.clone(),
            index: KNIFE_SPRITE_INDEX,
        },
    )
}

pub fn projectile(
    transform: Transform,
    velocity: Velocity,
    collider: Collider,
    range: f32,
    damage: f32,
) -> impl Bundle {
    (
        Projectile,
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

pub fn knife_projectile(
    transform: Transform,
    velocity: Velocity,
    collider: Collider,
    range: f32,
    damage: f32,
    item_assets: &ItemAssets,
) -> impl Bundle {
    (
        projectile(transform, velocity, collider, range, damage),
        knife_sprite(item_assets),
    )
}

#[derive(Event)]
pub struct ThrowProjectile {
    pub weapon: Entity,
    pub target: Entity,
    pub target_transform: Transform,
    pub range: f32,
    pub damage: f32,
}

fn on_throw_projectile(
    event: On<ThrowProjectile>,
    mut commands: Commands,
    projectile_query: Query<(&WeaponProjectileSize, &WeaponProjectileSpeed), With<Weapon>>,
    player_position_query: Query<&Transform, With<Player>>,
    level_query: Query<Entity, With<LevelIid>>,
    items_assets: Res<ItemAssets>,
) {
    info!(
        "Projectile throw triggered by weapon {} for enemy {}",
        event.weapon, event.target
    );

    let Ok(level_id) = level_query.single() else {
        return;
    };

    let Ok(player_transform) = player_position_query.single() else {
        return;
    };

    let Ok((projectile_size, projectile_speed)) = projectile_query.single() else {
        warn!("projectile is missing either size or speed component");
        return;
    };

    let projectile_range = event.range;
    let projectile_damage = event.damage;
    let target_transform = event.target_transform;

    let projectile_direction =
        target_transform.translation.xy() - player_transform.translation.xy();
    let projectile_velocity =
        Velocity::linear(projectile_direction.normalize_or_zero() * projectile_speed.speed);

    let mut projectile_transform = *player_transform;

    projectile_transform.translation.z = 10.0;

    let projectile_angle = Vec2::new(-1., 1.);

    projectile_transform.rotate_z(projectile_angle.angle_to(projectile_direction));

    let projectile_collider = Collider::ball(projectile_size.size / 2.);

    commands.entity(level_id).with_child(knife_projectile(
        projectile_transform,
        projectile_velocity,
        projectile_collider,
        projectile_range,
        projectile_damage,
        &items_assets,
    ));
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

fn handle_projectile_hit(
    mut commands: Commands,
    mut collision_events: MessageReader<CollisionEvent>,
    projectile_query: Query<&ProjectileDamage, With<Projectile>>,
    enemy_query: Query<Entity, With<Enemy>>,
) {
    let started_events = collision_events.read().filter_map(|&event| match event {
        CollisionEvent::Started(a, b, flags) => match flags {
            CollisionEventFlags::SENSOR => Some((a, b)),
            _ => None,
        },
        _ => None,
    });

    for (projectile, enemy) in started_events.map(|(a, b)| [(a, b), (b, a)]).flatten() {
        if !enemy_query.contains(enemy) {
            continue;
        }

        let Ok(projectile_damage) = projectile_query.get(projectile) else {
            continue;
        };

        info!(
            "applying {} damage to enemy {}",
            projectile_damage.damage, enemy
        );
        commands.trigger(ApplyDamage {
            entity: enemy,
            damage: projectile_damage.damage,
            text_color: None,
        });

        commands.entity(projectile).despawn();
    }
}

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.load_resource::<ItemAssets>()
            .add_observer(on_throw_projectile)
            .add_systems(
                Update,
                (handle_projectile_hit, despawn_projectile_on_range_exceeded).in_set(InGameSystems),
            );
    }
}
