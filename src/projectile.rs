use crate::agent::Enemy;
use crate::assets::ItemAssets;
use crate::damage::ApplyDamage;
use crate::game::{GameState, InGameSystems};
use crate::physics::started_collision_pairs;
use crate::player::Player;
use crate::weapon::Weapon::{Bow, Knife};
use crate::weapon::{KNIFE_SPRITE_INDEX, Weapon};
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
        projectile(transform, velocity, collider, range, damage),
        TargetTracking { target },
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
        projectile(transform, velocity, collider, range, damage),
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

fn handle_projectile_hit(
    mut commands: Commands,
    collision_events: MessageReader<CollisionEvent>,
    projectile_query: Query<&ProjectileDamage, With<Projectile>>,
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

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_throw_projectile).add_systems(
            Update,
            (
                handle_projectile_hit,
                despawn_projectile_on_range_exceeded,
                update_projectile_target_tracking,
            )
                .in_set(InGameSystems),
        );
    }
}
