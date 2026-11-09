use crate::agent::Enemy;
use crate::animation::FlippableSprite;
use crate::build::PlayerBuild;
use crate::damage::PlayerDamage;
use crate::game::{GameOver, InGameSystems};
use crate::health::Health;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::na::clamp;
use bevy_rapier2d::prelude::*;

pub struct PlayerPlugin;

const BASE_SPEED: f32 = 75.0;
const ACCELERATION: f32 = 300.0;
const DECELERATION: f32 = 600.0;

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
    #[with(player_mass)]
    mass: ColliderMassProperties,
    #[with(player_locked_axes)]
    locked_axes: LockedAxes,
    #[with(player_active_events)]
    active_events: ActiveEvents,
    #[sprite_sheet]
    sprite: Sprite,
}

#[derive(Resource, Default)]
pub struct PlayerPressure(pub(crate) f32);

fn player_health(_: &EntityInstance) -> Health {
    Health::new(PlayerBuild::default().max_health)
}

fn player_collider(_: &EntityInstance) -> Collider {
    Collider::capsule(Vec2::new(0., -9.), Vec2::new(0., 4.), 6.0)
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
    player_build: Res<PlayerBuild>,
) {
    let Ok(mut player_velocity) = velocity_query.single_mut() else {
        return;
    };

    let dt = time.delta_secs();

    let mut dir = Vec2::ZERO;

    if keyboard_input.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
        dir.x = -1.
    };
    if keyboard_input.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
        dir.x = 1.
    };
    if keyboard_input.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) {
        dir.y = 1.
    };
    if keyboard_input.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
        dir.y = -1.
    };

    let dir = dir.normalize_or_zero();

    let target = dir * BASE_SPEED * player_build.move_speed;
    let rate = if dir == Vec2::ZERO {
        DECELERATION * player_build.move_speed
    } else {
        ACCELERATION * player_build.move_speed
    };

    player_velocity.linear = player_velocity.linear.move_towards(target, rate * dt);
}

fn player_added(mut player_query: Query<&mut Transform, Added<Player>>) {
    if let Ok(mut player_transform) = player_query.single_mut() {
        player_transform.translation.z = 3.0;
        info!("Player spawned at {}", player_transform.translation);
    }
}

fn update_player_pressure(
    enemy_query: Query<(&Transform, &Health), With<Enemy>>,
    player_position_query: Query<&Transform, With<Player>>,
    mut player_pressure: ResMut<PlayerPressure>,
) {
    let max_distance = 300.;
    let mut pressure_points = 0.;

    let Ok(player_position) = player_position_query.single() else {
        return;
    };

    for (enemy_position, enemy_health) in &enemy_query {
        let distance = player_position
            .translation
            .xy()
            .distance(enemy_position.translation.xy());

        // distance based weight, closer enemies = more pressure
        let weight = clamp(1.0 - distance / max_distance, 0.2, 1.0);

        pressure_points += enemy_health.points * weight;
    }

    player_pressure.0 = pressure_points;
}

pub fn on_game_over(
    _event: On<GameOver>,
    mut player_pressure: ResMut<PlayerPressure>,
    mut player_damage: ResMut<PlayerDamage>,
) {
    info!("player pressure reset");
    *player_pressure = PlayerPressure::default();
    *player_damage = PlayerDamage::default();
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<PlayerBundle>("Player")
            .init_resource::<PlayerPressure>()
            .add_observer(on_game_over)
            .add_systems(Update, player_added)
            .add_systems(
                Update,
                (update_player_pressure, player_movement).in_set(InGameSystems),
            );
    }
}
