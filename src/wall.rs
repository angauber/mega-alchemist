use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct WallPlugin;

#[derive(Bundle, Default, LdtkEntity)]
struct WallBundle {
    #[with(wall_collider)]
    collider: Collider,
    #[sprite_sheet]
    sprite: Sprite,
}

fn wall_collider(_: &EntityInstance) -> Collider {
    Collider::cuboid(16., 16.)
}

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<WallBundle>("Rock_Wall")
            .register_ldtk_entity::<WallBundle>("Rock_Wall_Top");
    }
}
