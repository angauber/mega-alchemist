use crate::agent::{AgentSteering, Enemy};
use crate::animation::FlippableSprite;
use crate::attack::ContactAttack;
use crate::health::Health;
use crate::sprite_assets::MonsterAssets;
use bevy::app::App;
use bevy::image::TextureAtlas;
use bevy::log::info;
use bevy::math::Vec3;
use bevy::prelude::{
    Bundle, Commands, Entity, Event, On, Plugin, Query, Res, Sprite, Timer, TimerMode, Transform,
    With,
};
use bevy_ecs_ldtk::LevelIid;
use bevy_rapier2d::dynamics::{LockedAxes, RigidBody, Sleeping, Velocity};
use bevy_rapier2d::geometry::{Collider, ColliderMassProperties};

const SKELETON_SPRITE_INDEX: usize = 48;

pub struct SkeletonPlugin;

#[derive(Event)]
pub struct SpawnSkeleton(pub Transform);

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
        Health::new(50.),
        ContactAttack {
            damage: 25.,
            cooldown: Timer::from_seconds(0.3, TimerMode::Once),
        },
        AgentSteering {
            max_speed: 25.,
            max_accel: 300.,
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
    event: On<SpawnSkeleton>,
    mut commands: Commands,
    level_query: Query<Entity, With<LevelIid>>,
    monster_assets: Res<MonsterAssets>,
) {
    let Ok(level_id) = level_query.single() else {
        return;
    };

    let spawner_position = event.0;

    info!("Spawning skeleton at {:?}", spawner_position);

    commands.entity(level_id).with_child(skeleton(
        &monster_assets,
        spawner_position.with_scale(Vec3::splat(0.85)),
    ));
}

impl Plugin for SkeletonPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_spawn_skeleton);
    }
}
