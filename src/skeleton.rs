use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::agent::{AgentSteering, Enemy};
use crate::animation::FlippableSprite;
use crate::attack::ContactAttack;
use crate::health::Health;
use crate::player::Player;

pub struct SkeletonPlugin;

const SKELETON_SPRITE_INDEX: usize = 48;

#[derive(Event)]
pub struct SpawnSkeleton;

#[derive(Component, Default, Debug)]
pub struct SkeletonSpawner;

#[derive(Default, Bundle, Debug, LdtkEntity)]
pub struct SkeletonSpawnerBundle {
    transform: Transform,
    spawner: SkeletonSpawner,
}

#[derive(Bundle, Debug)]
pub struct SkeletonBundle {
    enemy: Enemy,
    health: Health,
    attack: ContactAttack,
    steering: AgentSteering,
    flippable: FlippableSprite,
    velocity: Velocity,
    transform: Transform,
    body: RigidBody,
    collider: Collider,
    sprite: Sprite,
    locked_axes: LockedAxes,
    mass: ColliderMassProperties,
    sleeping: Sleeping,
}

impl Default for SkeletonBundle {
    fn default() -> Self {
        SkeletonBundle {
            enemy: Enemy,
            health: Health { point: 10. },
            attack: ContactAttack { damage: 20., cooldown: Timer::from_seconds(0.3, TimerMode::Once) },
            steering: AgentSteering { max_speed: 100., max_accel: 500. },
            flippable: FlippableSprite,
            velocity: Velocity::default(),
            transform: Transform::default(),
            body: RigidBody::Dynamic,
            collider: Collider::ball(9.),
            sprite: Sprite::default(),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            mass: ColliderMassProperties::Density(0.1),
            sleeping: Sleeping::disabled(),
        }
    }
}


pub fn on_spawn_skeleton(
    _event: On<SpawnSkeleton>,
    mut commands: Commands,
    spawners_query: Query<&Transform, With<SkeletonSpawner>>,
    player_position_query: Query<&Transform, With<Player>>,
    level_query: Query<Entity, With<LevelIid>>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let Ok(level_id) = level_query.single() else {
        return;
    };

    let Ok(player_transform) = player_position_query.single() else {
        return;
    };

    let player_position = player_transform.translation.xy();

    let texture = asset_server.load("tileset/32rogues/monsters.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 12, 13, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let Some(furthest_spawner) = spawners_query
        .iter()
        .sort_by::<&Transform>(|spawner_a, spawner_b| player_position.distance(spawner_a.translation.xy()).total_cmp(&player_position.distance(spawner_b.translation.xy()))).next_back()
    else {
        warn!("no SkeletonSpawner found");
        return;
    };

    let mut skeleton_position = *furthest_spawner;

    skeleton_position.translation.z = 5.0;

    info!("Spawning skeleton at {:?}", skeleton_position.translation.xyz());

    commands.entity(level_id)
        .with_child(SkeletonBundle {
            transform: skeleton_position,
            sprite: Sprite::from_atlas_image(
                texture.clone(),
                TextureAtlas {
                    layout: texture_atlas_layout.clone(),
                    index: SKELETON_SPRITE_INDEX,
                }
            ),
            ..default()
        });
}

impl Plugin for SkeletonPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_ldtk_entity::<SkeletonSpawnerBundle>("Skeleton_Spawner")
            .add_observer(on_spawn_skeleton);
    }
}
