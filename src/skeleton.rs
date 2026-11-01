use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::agent::AgentSteering;
use crate::animation::FlippableSprite;
use crate::attack::ContactAttack;
use crate::health::Health;

pub struct SkeletonPlugin;

const SKELETON_SPRITE_INDEX: usize = 48;

#[derive(Component, Default, Debug)]
pub struct SkeletonSpawner;

#[derive(Default, Bundle, Debug, LdtkEntity)]
pub struct SkeletonSpawnerBundle {
    transform: Transform,
    spawner: SkeletonSpawner,
}

#[derive(Bundle, Debug)]
pub struct SkeletonBundle {
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
            health: Health { point: 10. },
            attack: ContactAttack { damage: 25., cooldown: Timer::from_seconds(0.3, TimerMode::Once) },
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


pub fn spawn_skeleton(
    mut commands: Commands,
    mut spawners_query: Query<&mut Transform, Added<SkeletonSpawner>>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    level_query: Query<Entity, With<LevelIid>>,
) {
    let Ok(level_id) = level_query.single() else {
        warn!("level id is missing");
        return;
    };

    let texture = asset_server.load("tileset/32rogues/monsters.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 12, 13, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    for mut spawner_position in spawners_query.iter_mut() {
        spawner_position.translation.z = 5.0;
        info!("Spawning skeleton at {:?}", spawner_position.translation.xyz());

        commands.entity(level_id).with_child(SkeletonBundle {
            transform: *spawner_position,
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
}

impl Plugin for SkeletonPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_ldtk_entity::<SkeletonSpawnerBundle>("Skeleton_Spawner")
            .add_systems(Update, spawn_skeleton);
    }
}