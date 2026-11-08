use crate::bat::BatPlugin;
use crate::skeleton::SkeletonPlugin;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[cfg(feature = "dev_tools")]
use bevy::color::palettes::css::NAVY;

pub struct SpawnerPlugin;

#[derive(Component, Default, Debug)]
pub struct Spawner;

#[derive(Default, Bundle, Debug, LdtkEntity)]
pub struct SpawnerBundle {
    spawner: Spawner,
}

#[cfg(feature = "dev_tools")]
fn draw_spawner_circle(mut gizmos: Gizmos, spawners: Query<&Transform, With<Spawner>>) {
    for spawner in &spawners {
        gizmos.circle_2d(
            Isometry2d::from_translation(spawner.with_scale(Vec3::splat(0.75)).translation.xy()),
            3.,
            NAVY,
        );
    }
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

        #[cfg(feature = "dev_tools")]
        app.add_systems(Update, draw_spawner_circle);
    }
}
