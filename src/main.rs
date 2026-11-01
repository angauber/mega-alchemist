use crate::camera::CameraPlugin;
use crate::physics::PhysicsPlugin;
use crate::player::PlayerPlugin;
use crate::skeleton::SkeletonPlugin;
use crate::wall::WallPlugin;
use crate::agent::AgentPlugin;
use crate::attack::AttackPlugin;
use crate::health::HealthPlugin;
use crate::animation::AnimationPlugin;
use crate::ui::UiPlugin;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use crate::game::GamePlugin;

mod camera;
mod player;
mod physics;
mod game;
mod wall;
mod skeleton;
mod health;
mod attack;
mod ui;
mod agent;
mod animation;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(
            WindowPlugin {
                primary_window: Some(Window {
                    title: "MegaAlchemist".to_string(),
                    ..default()
                }),
                ..default()
            }
        ))
        .add_plugins(GamePlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(PhysicsPlugin)
        .add_plugins(LdtkPlugin)
        .add_plugins(WallPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(AgentPlugin)
        .add_plugins(SkeletonPlugin)
        .add_plugins(AttackPlugin)
        .add_plugins(HealthPlugin)
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