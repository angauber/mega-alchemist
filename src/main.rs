extern crate core;

use crate::agent::AgentPlugin;
use crate::animation::AnimationPlugin;
use crate::assets::AssetPlugin;
use crate::attack::AttackPlugin;
use crate::build::BuildPlugin;
use crate::camera::CameraPlugin;
use crate::damage::DamagePlugin;
use crate::game::GamePlugin;
use crate::health::HealthPlugin;
use crate::level::LevelPlugin;
use crate::loading::LoadingPlugin;
use crate::physics::PhysicsPlugin;
use crate::player::PlayerPlugin;
use crate::spawn_manager::SpawnManagerPlugin;
use crate::spawner::SpawnerPlugin;
use crate::transmutation::TransmutationPlugin;
use crate::ui::UiPlugin;
use crate::wall::WallPlugin;
use crate::weapon::WeaponPlugin;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

mod agent;
mod animation;
mod asset_tracking;
mod assets;
mod attack;
mod bat;
mod build;
mod camera;
mod damage;
mod game;
mod health;
mod level;
mod loading;
mod manuscript;
mod physics;
mod player;
mod projectile;
mod skeleton;
mod spawn_manager;
mod spawner;
mod transmutation;
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
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: false,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(asset_tracking::plugin)
        .add_plugins(AssetPlugin)
        .add_plugins(LoadingPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(PhysicsPlugin)
        .add_plugins(LdtkPlugin)
        .add_plugins(GamePlugin)
        .add_plugins(UiPlugin)
        .add_plugins(LevelPlugin)
        .add_plugins(TransmutationPlugin)
        .add_plugins(WallPlugin)
        .add_plugins(BuildPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(SpawnManagerPlugin)
        .add_plugins(AgentPlugin)
        .add_plugins(SpawnerPlugin)
        .add_plugins(DamagePlugin)
        .add_plugins(AttackPlugin)
        .add_plugins(HealthPlugin)
        .add_plugins(WeaponPlugin)
        .add_plugins(AnimationPlugin)
        .add_systems(Startup, scene.spawn())
        .insert_resource(LevelSelection::index(0))
        .run();
}

fn scene() -> impl Scene {
    bsn! {
        LdtkProjectHandle { handle: "world.ldtk" }
    }
}
