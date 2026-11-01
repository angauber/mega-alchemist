use bevy::prelude::*;
use bevy_rapier2d::dynamics::Velocity;

pub struct AnimationPlugin;

#[derive(Component, Default, Debug)]
pub struct FlippableSprite;

pub fn animate_flippable_sprite(
    mut sprite_query: Query<(&mut Sprite, &Velocity), With<FlippableSprite>>
) {
    for (mut sprite, velocity) in sprite_query.iter_mut() {
        if velocity.linear.x > 0.0 {
            sprite.flip_x = true;
        }

        if velocity.linear.x < 0.0 {
            sprite.flip_x = false;
        }
    }
}

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate_flippable_sprite);
    }
}
