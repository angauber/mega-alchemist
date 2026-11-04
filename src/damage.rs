use crate::health::Health;
use crate::ui::text::RenderUiText;
use bevy::prelude::*;
use bevy_rapier2d::na::clamp;
use std::time::Duration;

pub struct DamagePlugin;

#[derive(Event)]
pub struct ApplyDamage {
    pub entity: Entity,
    pub damage: f32,
    pub text_color: Option<Color>,
}

fn apply_damage(
    event: On<ApplyDamage>,
    transform_query: Query<&Transform>,
    mut health_query: Query<&mut Health>,
    mut render_ui_text_event: MessageWriter<RenderUiText>,
) {
    let Ok(mut entity_health) = health_query.get_mut(event.entity) else {
        return;
    };

    let Ok(entity_position) = transform_query.get(event.entity) else {
        return;
    };

    let actual_damage = clamp(event.damage, 1.0, entity_health.points);

    entity_health.points -= actual_damage;

    let mut damage_text_position = *entity_position;

    damage_text_position.translation.y += 20.0;

    render_ui_text_event.write(RenderUiText {
        text: format!("-{:.0}", actual_damage).into(),
        font_size: 8.,
        color: event.text_color.unwrap_or(Color::WHITE),
        duration: Duration::from_millis(250),
        transform: Some(damage_text_position),
    });
}

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(apply_damage);
    }
}
