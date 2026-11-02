use crate::agent::{AgentSteering, Enemy};
use crate::animation::FlippableSprite;
use crate::asset_tracking::LoadResource;
use crate::attack::ContactAttack;
use crate::health::Health;
use crate::player::Player;
use crate::sprite_assets::MonsterAssets;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

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

pub fn skeleton_sprite(monster_assets: &MonsterAssets) -> Sprite {
    Sprite::from_atlas_image(
        monster_assets.sprite.clone(),
        TextureAtlas {
            layout: monster_assets.texture_atlas_layout.clone(),
            index: SKELETON_SPRITE_INDEX,
        },
    )
}

pub fn skeleton(monster_assets: &MonsterAssets, position: Transform) -> impl Bundle {
    (
        Enemy,
        Health { point: 10. },
        ContactAttack {
            damage: 10.,
            cooldown: Timer::from_seconds(0.3, TimerMode::Once),
        },
        AgentSteering {
            max_speed: 100.,
            max_accel: 500.,
        },
        FlippableSprite,
        Velocity::default(),
        position,
        RigidBody::Dynamic,
        Collider::ball(7.),
        LockedAxes::ROTATION_LOCKED,
        ColliderMassProperties::Density(0.1),
        Sleeping::disabled(),
        skeleton_sprite(monster_assets),
    )
}

pub fn on_spawn_skeleton(
    _event: On<SpawnSkeleton>,
    mut commands: Commands,
    spawners_query: Query<&Transform, With<SkeletonSpawner>>,
    player_position_query: Query<&Transform, With<Player>>,
    level_query: Query<Entity, With<LevelIid>>,
    monster_assets: Res<MonsterAssets>,
) {
    let Ok(level_id) = level_query.single() else {
        return;
    };

    let Ok(player_transform) = player_position_query.single() else {
        return;
    };

    let player_position = player_transform.translation.xy();

    let Some(furthest_spawner) = spawners_query
        .iter()
        .sort_by::<&Transform>(|spawner_a, spawner_b| {
            player_position
                .distance(spawner_a.translation.xy())
                .total_cmp(&player_position.distance(spawner_b.translation.xy()))
        })
        .next_back()
    else {
        warn!("no SkeletonSpawner found");
        return;
    };

    let mut skeleton_position = *furthest_spawner;

    skeleton_position.translation.z = 5.0;

    info!(
        "Spawning skeleton at {:?}",
        skeleton_position.translation.xyz()
    );

    commands
        .entity(level_id)
        .with_child(skeleton(&monster_assets, skeleton_position));
}

impl Plugin for SkeletonPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<SkeletonSpawnerBundle>("Skeleton_Spawner")
            .load_resource::<MonsterAssets>()
            .add_observer(on_spawn_skeleton);
    }
}
