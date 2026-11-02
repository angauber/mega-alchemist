use crate::agent::AgentPlugin;
use crate::animation::AnimationPlugin;
use crate::attack::AttackPlugin;
use crate::camera::CameraPlugin;
use crate::health::HealthPlugin;
use crate::physics::PhysicsPlugin;
use crate::player::PlayerPlugin;
use crate::skeleton::SkeletonPlugin;
use crate::ui::UiPlugin;
use crate::wall::WallPlugin;

use crate::game::GamePlugin;
use crate::spawn_manager::SpawnManagerPlugin;
use crate::weapon::WeaponPlugin;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

mod agent;
mod animation;
mod attack;
mod camera;
mod game;
mod health;
mod physics;
mod player;
mod projectile;
mod skeleton;
mod spawn_manager;
mod ui;
mod wall;
mod weapon;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "MegaAlchemist".to_string(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(GamePlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(PhysicsPlugin)
        .add_plugins(LdtkPlugin)
        .add_plugins(WallPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(SpawnManagerPlugin)
        .add_plugins(AgentPlugin)
        .add_plugins(SkeletonPlugin)
        .add_plugins(AttackPlugin)
        .add_plugins(HealthPlugin)
        .add_plugins(WeaponPlugin)
        .add_plugins(AnimationPlugin)
        .add_plugins(UiPlugin)
        .add_systems(Startup, scene.spawn())
        .insert_resource(LevelSelection::index(0))
        .run();
}

fn scene() -> impl Scene {
    bsn! {
        LdtkProjectHandle { handle: "world.ldtk" }
    }
}
