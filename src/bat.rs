use crate::agent::{AgentSteering, Enemy};
use crate::animation::FlippableSprite;
use crate::attack::ContactAttack;
use crate::health::Health;
use crate::sprite_assets::MonsterAssets;
use bevy::app::{App, Plugin};
use bevy::image::TextureAtlas;
use bevy::log::info;
use bevy::prelude::{
    Bundle, Commands, Entity, Event, On, Query, Res, Sprite, Timer, TimerMode, Transform, Vec3,
    With,
};
use bevy_ecs_ldtk::LevelIid;
use bevy_rapier2d::dynamics::{LockedAxes, RigidBody, Sleeping, Velocity};
use bevy_rapier2d::geometry::{Collider, ColliderMassProperties};

const BAT_SPRITE_INDEX: usize = 78;

pub struct BatPlugin;

#[derive(Event)]
pub struct SpawnBat(pub Transform);

pub fn bat_sprite(monster_assets: &MonsterAssets) -> Sprite {
    Sprite::from_atlas_image(
        monster_assets.sprite.clone(),
        TextureAtlas {
            layout: monster_assets.texture_atlas_layout.clone(),
            index: BAT_SPRITE_INDEX,
        },
    )
}

pub fn bat(monster_assets: &MonsterAssets, position: Transform) -> impl Bundle {
    (
        Enemy,
        Health { point: 10. },
        ContactAttack {
            damage: 10.,
            cooldown: Timer::from_seconds(0.3, TimerMode::Once),
        },
        AgentSteering {
            max_speed: 40.,
            max_accel: 500.,
        },
        FlippableSprite,
        Velocity::default(),
        position,
        RigidBody::Dynamic,
        Collider::ball(5.),
        LockedAxes::ROTATION_LOCKED,
        ColliderMassProperties::Density(0.1),
        Sleeping::disabled(),
        bat_sprite(monster_assets),
    )
}

pub fn on_spawn_bat(
    event: On<SpawnBat>,
    mut commands: Commands,
    level_query: Query<Entity, With<LevelIid>>,
    monster_assets: Res<MonsterAssets>,
) {
    let Ok(level_id) = level_query.single() else {
        return;
    };

    let spawner_position = event.0;

    info!("Spawning bat at {:?}", spawner_position);

    commands.entity(level_id).with_child(bat(
        &monster_assets,
        spawner_position.with_scale(Vec3::splat(0.75)),
    ));
}

impl Plugin for BatPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_spawn_bat);
    }
}
