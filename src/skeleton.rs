use crate::agent::{AgentSteering, Enemy};
use crate::animation::FlippableSprite;
use crate::assets::MonsterAssets;
use crate::attack::ContactAttack;
use crate::game::GameState;
use crate::health::Health;
use bevy::app::App;
use bevy::image::TextureAtlas;
use bevy::prelude::*;
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
        Collider::ball(12.),
        LockedAxes::ROTATION_LOCKED,
        ColliderMassProperties::Density(0.1),
        Sleeping::disabled(),
        skeleton_sprite(monster_assets),
    )
}

pub fn on_spawn_skeleton(
    event: On<SpawnSkeleton>,
    mut commands: Commands,
    monster_assets: Res<MonsterAssets>,
) {
    let spawner_position = event.0;

    commands.spawn((
        skeleton(
            &monster_assets,
            spawner_position.with_scale(Vec3::splat(0.85)),
        ),
        DespawnOnEnter(GameState::InGame),
    ));
}

impl Plugin for SkeletonPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_spawn_skeleton);
    }
}
