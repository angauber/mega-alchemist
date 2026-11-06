use crate::damage::ApplyDamage;
use crate::game::InGameSystems;
use crate::player::Player;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

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
    attacker_query: Query<Entity, With<ContactAttack>>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity_a, entity_b, _flags) => {
                for (player_entity, attacker_entity) in [(entity_a, entity_b), (entity_b, entity_a)]
                {
                    if !attacker_query.contains(*attacker_entity) {
                        continue;
                    };

                    if !player_query.contains(*player_entity) {
                        continue;
                    };

                    commands.entity(*attacker_entity).insert(Attacking);
                }
            }
            CollisionEvent::Stopped(entity_a, entity_b, _flags) => {
                for (player_entity, attacker_entity) in [(entity_a, entity_b), (entity_b, entity_a)]
                {
                    if !attacker_query.contains(*attacker_entity) {
                        continue;
                    };

                    if !player_query.contains(*player_entity) {
                        continue;
                    };

                    commands.entity(*attacker_entity).remove::<Attacking>();
                }
            }
        }
    }
}

fn attacking_player_damage_system(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    attackers_query: Query<(Entity, &ContactAttack), (With<Attacking>, Without<AttackCooldown>)>,
) {
    let Ok(player_entity) = player_query.single() else {
        return;
    };

    for (attacker_entity, contact_attack) in &attackers_query {
        commands.entity(attacker_entity).insert(AttackCooldown {
            duration: contact_attack.cooldown.clone(),
        });

        commands.trigger(ApplyDamage {
            from_player: false,
            entity: player_entity,
            damage: contact_attack.damage,
            text_color: Some(Color::linear_rgb(1.0, 0.0, 0.0)),
        })
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
        app.add_systems(
            Update,
            (
                attack_contact_system,
                cooldown_system,
                attacking_player_damage_system,
            )
                .in_set(InGameSystems),
        );
    }
}
