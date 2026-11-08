use crate::agent::{AgentSteering, Enemy};
use crate::animation::FlippableSprite;
use crate::assets::MonsterAssets;
use crate::attack::ContactAttack;
use crate::game::GameState;
use crate::health::Health;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const DRAGON_SPRITE_INDEX: usize = 97;

pub struct DragonPlugin;

#[derive(Event)]
pub struct SpawnDragon(pub Transform);

pub fn dragon_sprite(monster_assets: &MonsterAssets) -> Sprite {
    Sprite::from_atlas_image(
        monster_assets.sprite.clone(),
        TextureAtlas {
            layout: monster_assets.texture_atlas_layout.clone(),
            index: DRAGON_SPRITE_INDEX,
        },
    )
}

pub fn dragon(monster_assets: &MonsterAssets, position: Transform) -> impl Bundle {
    (
        Enemy,
        Health::new(150.),
        ContactAttack {
            damage: 50.,
            cooldown: Timer::from_seconds(0.3, TimerMode::Once),
        },
        AgentSteering {
            max_speed: 20.,
            max_accel: 200.,
        },
        FlippableSprite,
        Velocity::default(),
        position,
        RigidBody::Dynamic,
        Collider::ball(10.),
        LockedAxes::ROTATION_LOCKED,
        ColliderMassProperties::Density(0.1),
        Sleeping::disabled(),
        dragon_sprite(monster_assets),
    )
}

pub fn on_spawn_dragon(
    event: On<SpawnDragon>,
    mut commands: Commands,
    monster_assets: Res<MonsterAssets>,
) {
    let spawner_position = event.0;

    commands.spawn((
        dragon(
            &monster_assets,
            spawner_position.with_scale(Vec3::splat(1.0)),
        ),
        DespawnOnEnter(GameState::InGame),
    ));
}

impl Plugin for DragonPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_spawn_dragon);
    }
}
