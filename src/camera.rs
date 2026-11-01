use bevy::camera::ScalingMode;
use bevy::prelude::*;
use crate::player::Player;

pub struct CameraPlugin;

fn setup_camera(mut commands: Commands) {
    let mut projection = OrthographicProjection::default_2d();

    projection.scaling_mode = ScalingMode::FixedVertical {
        viewport_height: 350.0,
    };

    commands.spawn((
        Camera2d::default(),
        Projection::from(projection),
        Transform::default(),
    ));
}

pub fn center_camera_on_player(
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    player_transform_query: Query<&Transform, (With<Player>, Without<Camera2d>)>,
) {

    let Ok(player_transform) = player_transform_query.single() else { return; };
    let Ok(mut camera_transform) = camera_query.single_mut() else { return; };

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_camera)
            .add_systems(Update, center_camera_on_player);
    }
}
