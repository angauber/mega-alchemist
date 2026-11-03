use crate::asset_tracking::LoadResource;
use crate::bat::BatPlugin;
use crate::skeleton::SkeletonPlugin;
use crate::sprite_assets::MonsterAssets;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub struct SpawnerPlugin;

#[derive(Component, Default, Debug)]
pub struct Spawner;

#[derive(Default, Bundle, Debug, LdtkEntity)]
pub struct SpawnerBundle {
    transform: Transform,
    spawner: Spawner,
}

impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BatPlugin)
            .add_plugins(SkeletonPlugin)
            .register_ldtk_entity::<SpawnerBundle>("Spawner")
            .load_resource::<MonsterAssets>();
    }
}
