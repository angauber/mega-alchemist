use crate::agent::{AgentSteering, Enemy};
use crate::animation::FlippableSprite;
use crate::assets::MonsterAssets;
use crate::attack::ContactAttack;
use crate::game::GameState;
use crate::health::Health;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

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
        Health::new(15.),
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
        Collider::ball(8.),
        LockedAxes::ROTATION_LOCKED,
        ColliderMassProperties::Density(0.1),
        Sleeping::disabled(),
        bat_sprite(monster_assets),
    )
}

pub fn on_spawn_bat(
    event: On<SpawnBat>,
    mut commands: Commands,
    monster_assets: Res<MonsterAssets>,
) {
    let spawner_position = event.0;

    commands.spawn((
        bat(
            &monster_assets,
            spawner_position.with_scale(Vec3::splat(0.65)),
        ),
        DespawnOnEnter(GameState::InGame),
    ));
}

impl Plugin for BatPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_spawn_bat);
    }
}
