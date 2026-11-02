use crate::player::Player;
use crate::weapon::{Weapon, WeaponProjectileSize, WeaponProjectileSpeed};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct ProjectilePlugin;

#[derive(Bundle, Debug)]
pub struct ProjectileBundle {
    transform: Transform,
    body: RigidBody,
    velocity: Velocity,
    collider: Collider,
    sensor: Sensor,
    active_events: ActiveEvents,
    sprite: Sprite,
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
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
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

    let texture = asset_server.load("tileset/32rogues/items.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 11, 26, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let projectile_sprite = Sprite::from_atlas_image(
        texture.clone(),
        TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: 0,
        },
    );

    commands.entity(level_id).with_child(ProjectileBundle {
        transform: projectile_transform,
        body: RigidBody::Dynamic,
        velocity: projectile_velocity,
        collider: projectile_collider,
        sensor: Sensor,
        active_events: ActiveEvents::COLLISION_EVENTS,
        sprite: projectile_sprite,
    });
}

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_throw_projectile);
    }
}
