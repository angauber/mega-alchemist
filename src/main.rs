extern crate core;

use crate::agent::AgentPlugin;
use crate::animation::AnimationPlugin;
use crate::attack::AttackPlugin;
use crate::camera::CameraPlugin;
use crate::damage::DamagePlugin;
use crate::game::GamePlugin;
use crate::health::HealthPlugin;
use crate::physics::PhysicsPlugin;
use crate::player::PlayerPlugin;
use crate::spawn_manager::SpawnManagerPlugin;
use crate::spawner::SpawnerPlugin;
use crate::ui::UiPlugin;
use crate::wall::WallPlugin;
use crate::weapon::WeaponPlugin;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
#[cfg(feature = "dev_tools")]
use crate::debug::DebugPlugin;

mod agent;
mod animation;
mod asset_tracking;
mod attack;
mod bat;
mod camera;
mod damage;
mod game;
mod health;
mod physics;
mod player;
mod projectile;
mod skeleton;
mod spawn_manager;
mod spawner;
mod sprite_assets;
mod ui;
mod wall;
mod weapon;
#[cfg(feature = "dev_tools")]
mod debug;

fn main() {
    let mut app = App::new();
    
    app
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
        .add_plugins(asset_tracking::plugin)
        .add_plugins(GamePlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(PhysicsPlugin)
        .add_plugins(LdtkPlugin)
        .add_plugins(WallPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(SpawnManagerPlugin)
        .add_plugins(AgentPlugin)
        .add_plugins(SpawnerPlugin)
        .add_plugins(DamagePlugin)
        .add_plugins(AttackPlugin)
        .add_plugins(HealthPlugin)
        .add_plugins(WeaponPlugin)
        .add_plugins(AnimationPlugin)
        .add_plugins(UiPlugin)
        .add_systems(Startup, scene.spawn())
        .insert_resource(LevelSelection::index(0));

    
        #[cfg(feature = "dev_tools")]
        {
            app.add_plugins(DebugPlugin);
        }
        
        app.run();
}

fn scene() -> impl Scene {
    bsn! {
        LdtkProjectHandle { handle: "world.ldtk" }
    }
}
