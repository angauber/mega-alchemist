use crate::bat::BatPlugin;
use crate::skeleton::SkeletonPlugin;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub struct SpawnerPlugin;

#[derive(Component, Default, Debug)]
pub struct Spawner;

#[derive(Default, Bundle, Debug, LdtkEntity)]
pub struct SpawnerBundle {
    spawner: Spawner,
}

fn spawn_added(mut spawn_query: Query<&mut Transform, Added<Spawner>>) {
    for mut spawner in spawn_query.iter_mut() {
        spawner.translation.z = 2.;
    }
}

impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BatPlugin)
            .add_plugins(SkeletonPlugin)
            .register_ldtk_entity::<SpawnerBundle>("Spawner")
            .add_systems(Update, spawn_added);
    }
}
