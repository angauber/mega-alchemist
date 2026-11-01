use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::animation::FlippableSprite;
use crate::health::Health;

pub struct PlayerPlugin;

const MAX_SPEED: f32 = 150.0;
const ACCELERATION: f32 = 750.0;
const DECELERATION: f32 = 1500.0;

#[derive(Component, Default)]
pub struct Player;

#[derive(Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    player: Player,
    #[with(player_health)]
    health: Health,
    flippable: FlippableSprite,
    #[with(player_collider)]
    collider: Collider,
    #[with(player_rigid_body)]
    rigid_body: RigidBody,
    velocity: Velocity,
    transform: Transform,
    #[with(player_mass)]
    mass: ColliderMassProperties,
    #[with(player_locked_axes)]
    locked_axes: LockedAxes,
    #[with(player_active_events)]
    active_events: ActiveEvents,
    #[sprite_sheet]
    sprite: Sprite,
}

fn player_health(_: &EntityInstance) -> Health {
    Health { point: 100. }
}

fn player_collider(_: &EntityInstance) -> Collider {
    Collider::capsule(Vec2::new(0., -8.), Vec2::new(0., 8.), 8.0)
}

fn player_rigid_body(_: &EntityInstance) -> RigidBody {
    RigidBody::Dynamic
}

fn player_mass(_: &EntityInstance) -> ColliderMassProperties {
    ColliderMassProperties::Density(0.1)
}

fn player_locked_axes(_: &EntityInstance) -> LockedAxes {
    LockedAxes::ROTATION_LOCKED
}

fn player_active_events(_: &EntityInstance) -> ActiveEvents {
    ActiveEvents::COLLISION_EVENTS
}

fn player_movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut velocity_query: Query<&mut Velocity, With<Player>>,
) {
    let Ok(mut player_velocity) = velocity_query.single_mut() else {
        return;
    };

    let dt = time.delta_secs();

    let mut dir = Vec2::ZERO;

    if keyboard_input.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) { dir.x = -1. };
    if keyboard_input.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) { dir.x = 1. };
    if keyboard_input.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) { dir.y = 1. };
    if keyboard_input.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) { dir.y = -1. };

    let dir = dir.normalize_or_zero();

    let target = dir * MAX_SPEED;
    let rate = if dir == Vec2::ZERO { DECELERATION } else { ACCELERATION };

    player_velocity.linear = player_velocity.linear.move_towards(target, rate * dt);
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<PlayerBundle>("Player")
            .add_systems(Update, player_movement);
    }
}
