use crate::asset_tracking::LoadResource;
use crate::player::Player;
use crate::sprite_assets::ItemAssets;
use crate::weapon::{Weapon, WeaponProjectileSize, WeaponProjectileSpeed};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct ProjectilePlugin;

const KNIFE_SPRITE_INDEX: usize = 0;

pub fn knife_sprite(item_assets: &ItemAssets) -> Sprite {
    Sprite::from_atlas_image(
        item_assets.sprite.clone(),
        TextureAtlas {
            layout: item_assets.texture_atlas_layout.clone(),
            index: KNIFE_SPRITE_INDEX,
        },
    )
}

pub fn projectile(transform: Transform, velocity: Velocity, collider: Collider) -> impl Bundle {
    (
        transform,
        velocity,
        collider,
        RigidBody::Dynamic,
        Sensor,
        ActiveEvents::COLLISION_EVENTS,
    )
}

pub fn knife_projectile(
    transform: Transform,
    velocity: Velocity,
    collider: Collider,
    item_assets: &ItemAssets,
) -> impl Bundle {
    (
        projectile(transform, velocity, collider),
        knife_sprite(item_assets),
    )
}

#[derive(Event)]
pub struct ThrowProjectile {
    pub weapon: Entity,
    pub target: Entity,
    pub target_transform: Transform,
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
        &items_assets,
    ));
}

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.load_resource::<ItemAssets>()
            .add_observer(on_throw_projectile);
    }
}
