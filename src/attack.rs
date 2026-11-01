use std::time::Duration;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::health::Health;
use crate::player::Player;
use crate::ui::RenderUiText;

pub struct AttackPlugin;

#[derive(Component, Default, Debug)]
pub struct ContactAttack {
    pub damage: f32,
    pub cooldown: Timer,
}

#[derive(Component, Default, Debug)]
pub struct AttackCooldown {
    duration: Timer,
}

#[derive(Component, Default, Debug)]
pub struct Attacking;

fn attack_contact_system(
    mut commands: Commands,
    mut collision_events: MessageReader<CollisionEvent>,
    player_query: Query<Entity, With<Player>>,
    attacker_query: Query<Entity, With<ContactAttack>>
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity_a, entity_b, _flags) => {
                for (player_entity, attacker_entity) in [(entity_a, entity_b), (entity_b, entity_a)] {
                    if !attacker_query.contains(*attacker_entity){
                        continue;
                    };

                    if !player_query.contains(*player_entity){
                        continue;
                    };

                    info!("Enemy {} contacted with player", attacker_entity);

                    commands.entity(*attacker_entity).insert(Attacking);
                }
            },
            CollisionEvent::Stopped(entity_a, entity_b, _flags) => {
                for (player_entity, attacker_entity) in [(entity_a, entity_b), (entity_b, entity_a)] {
                    if !attacker_query.contains(*attacker_entity){
                        continue;
                    };

                    if !player_query.contains(*player_entity){
                        continue;
                    };

                    info!("Enemy {} ended contact with player", attacker_entity);

                    commands.entity(*attacker_entity).remove::<Attacking>();
                }
            }
        }
    }
}

fn attacking_player_damage_system(
    mut commands: Commands,
    mut player_query: Query<(&mut Health, &Transform), With<Player>>,
    mut render_ui_text_event: MessageWriter<RenderUiText>,
    attackers_query: Query<(Entity, &ContactAttack), (With<Attacking>, Without<AttackCooldown>)>,
)
{
    let Ok((mut player_health, player_transform)) = player_query.single_mut() else {
        return;
    };

    for (attacker_entity, contact_attack) in &attackers_query {
        player_health.point -= contact_attack.damage;
        commands.entity(attacker_entity).insert(AttackCooldown { duration: contact_attack.cooldown.clone() });

        let mut text_transform = *player_transform;

        text_transform.translation.y += 20.;

        render_ui_text_event.write(RenderUiText {
            text: format!("-{} HP", contact_attack.damage).into(),
            font_size: 8.,
            color: Color::linear_rgb(1.0, 0.0, 0.0),
            duration: Duration::from_millis(250),
            transform: Some(text_transform),
        });
    }
}

fn cooldown_system(
    mut commands: Commands,
    time: Res<Time>,
    mut attack_cooldown_query: Query<(Entity, &mut AttackCooldown)>,
) {
    for (entity, mut attack_cooldown) in attack_cooldown_query.iter_mut() {
        attack_cooldown.duration.tick(time.delta());

        if attack_cooldown.duration.is_finished() {
            commands.entity(entity).remove::<AttackCooldown>();
        }
    }
}

impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, attack_contact_system);
        app.add_systems(Update, attacking_player_damage_system);
        app.add_systems(Update, cooldown_system);
    }
}