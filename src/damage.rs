use crate::health::Health;
use crate::ui::text::RenderUiText;
use bevy::prelude::*;
use bevy_rapier2d::na::clamp;
use std::time::Duration;

pub struct DamagePlugin;

#[derive(Event)]
pub struct ApplyDamage {
    pub from_player: bool,
    pub entity: Entity,
    pub damage: f32,
    pub text_color: Option<Color>,
}

#[derive(Resource, Default)]
pub struct PlayerDamage {
    pub current_frame: f32,
    pub per_second: f32,
}

fn apply_damage(
    event: On<ApplyDamage>,
    transform_query: Query<&Transform>,
    mut player_dps: ResMut<PlayerDamage>,
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

    if event.from_player {
        player_dps.current_frame += actual_damage;
    }

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

fn update_player_dps(mut player_damage: ResMut<PlayerDamage>, time: Res<Time>) {
    let dt = time.delta_secs();
    let window_seconds: f32 = 5.0;

    player_damage.per_second +=
        (player_damage.current_frame - player_damage.per_second * dt) / window_seconds;

    if player_damage.current_frame > 0. {
        info!(
            "player delt {} dmg this frame, dps: {}",
            player_damage.current_frame, player_damage.per_second
        );
    }

    player_damage.current_frame = 0.0;
}

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerDamage>()
            .add_observer(apply_damage)
            .add_systems(PostUpdate, update_player_dps);
    }
}
