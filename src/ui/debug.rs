use crate::game::{EnemyCounter, GameMainTextUiNode, InGameSystems};
use crate::spawn_manager::SpawnManager;
use crate::ui::UiTextBundle;
use bevy::prelude::*;

pub struct DebugPlugin;

#[derive(Component)]
struct BankValueText;
#[derive(Component)]
struct BankBudgetText;
#[derive(Component)]
struct EnemyCountText;

fn setup_debug_ui_nodes(
    mut commands: Commands,
    game_ui_query: Query<Entity, Added<GameMainTextUiNode>>,
    enemy_counter: Res<EnemyCounter>,
    spawn_manager: Res<SpawnManager>,
    asset_server: Res<AssetServer>,
) {
    let Ok(game_ui_entity) = game_ui_query.single() else {
        return;
    };

    let font = FontSource::Handle(asset_server.load("fonts/Pixuf.ttf"));

    commands
        .entity(game_ui_entity)
        .with_child((
            EnemyCountText,
            UiTextBundle {
                text: enemy_count_text(enemy_counter.count),
                color: TextColor(Color::WHITE),
                font: TextFont {
                    font: font.clone(),
                    font_size: FontSize::Px(32.),
                    ..default()
                },
                layout: TextLayout::justify(Justify::Center),
            },
        ))
        .with_child((
            BankValueText,
            UiTextBundle {
                text: bank_value_text(spawn_manager.bank_value()),
                color: TextColor(Color::WHITE),
                font: TextFont {
                    font: font.clone(),
                    font_size: FontSize::Px(32.),
                    ..default()
                },
                layout: TextLayout::justify(Justify::Center),
            },
        ))
        .with_child((
            BankBudgetText,
            UiTextBundle {
                text: bank_budget_text(spawn_manager.bank_budget()),
                color: TextColor(Color::WHITE),
                font: TextFont {
                    font,
                    font_size: FontSize::Px(32.),
                    ..default()
                },
                layout: TextLayout::justify(Justify::Center),
            },
        ));
}

fn update_enemy_count_text(
    mut enemy_count_query: Query<&mut Text, With<EnemyCountText>>,
    enemy_counter: Res<EnemyCounter>,
) {
    let Ok(mut enemy_count) = enemy_count_query.single_mut() else {
        return;
    };

    *enemy_count = enemy_count_text(enemy_counter.count);
}

fn update_bank_value_text(
    mut bank_value_query: Query<&mut Text, With<BankValueText>>,
    spawn_manager: Res<SpawnManager>,
) {
    let Ok(mut bank_value) = bank_value_query.single_mut() else {
        return;
    };

    *bank_value = bank_value_text(spawn_manager.bank_value());
}

fn update_bank_budget_text(
    mut bank_budget_query: Query<&mut Text, With<BankBudgetText>>,
    spawn_manager: Res<SpawnManager>,
) {
    let Ok(mut bank_budget) = bank_budget_query.single_mut() else {
        return;
    };

    *bank_budget = bank_budget_text(spawn_manager.bank_budget());
}

fn enemy_count_text(enemy_count: usize) -> Text {
    format!("Enemy Count {}", enemy_count).into()
}

fn bank_value_text(bank_value: f32) -> Text {
    format!("Bank Value {}", bank_value).into()
}

fn bank_budget_text(bank_budget: f32) -> Text {
    format!("Bank Budget {}", bank_budget).into()
}

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                setup_debug_ui_nodes,
                update_enemy_count_text,
                update_bank_value_text,
                update_bank_budget_text,
            )
                .in_set(InGameSystems),
        );
    }
}
