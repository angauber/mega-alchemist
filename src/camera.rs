use crate::build::PlayerBuild;
use crate::game::InGameSystems;
use crate::player::Player;
use bevy::camera::ScalingMode;
use bevy::prelude::*;

pub struct CameraPlugin;

fn setup_camera(mut commands: Commands) {
    let mut projection = OrthographicProjection::default_2d();

    projection.scaling_mode = ScalingMode::FixedVertical {
        viewport_height: 200.0,
    };

    commands.spawn((Camera2d, Projection::from(projection), Transform::default()));
}

fn center_camera_on_player(
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    player_transform_query: Query<&Transform, (With<Player>, Without<Camera2d>)>,
) {
    let Ok(player_transform) = player_transform_query.single() else {
        return;
    };
    let Ok(mut camera_transform) = camera_query.single_mut() else {
        return;
    };

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}

fn scale_viewport(
    mut camera_query: Query<&mut Projection, With<Camera2d>>,
    player_build: Res<PlayerBuild>,
) {
    let Ok(mut projection) = camera_query.single_mut() else {
        return;
    };

    let Projection::Orthographic(projection) = &mut *projection else {
        return;
    };

    projection.scale = 1.0 + (player_build.range / 2.0);
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera).add_systems(
            Update,
            (center_camera_on_player, scale_viewport).in_set(InGameSystems),
        );
    }
}
