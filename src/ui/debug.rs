use crate::assets::FontAssets;
use crate::damage::PlayerDamage;
use crate::game::{EnemyCounter, InGameSystems};
use crate::spawn_manager::SpawnManager;
use crate::ui::game::GameMainTextUiNode;
use crate::ui::text::UiTextBundle;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

pub struct DebugPlugin;

#[derive(Component)]
struct EnemyCountText;
#[derive(Component)]
struct FramePerSecondText;
#[derive(Component)]
struct PlayerDpsText;
#[derive(Component)]
struct BankBudgetText;

fn setup_debug_ui_nodes(
    mut commands: Commands,
    game_ui_query: Query<Entity, Added<GameMainTextUiNode>>,
    enemy_counter: Res<EnemyCounter>,
    spawn_manager: Res<SpawnManager>,
    player_damage: Res<PlayerDamage>,
    diagnostic: Res<DiagnosticsStore>,
    font_assets: Res<FontAssets>,
) {
    let Ok(game_ui_entity) = game_ui_query.single() else {
        return;
    };

    let font = FontSource::Handle(font_assets.pixuf_font.clone());

    commands
        .entity(game_ui_entity)
        .with_child((
            FramePerSecondText,
            UiTextBundle {
                text: fps_text(diagnostic),
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
            PlayerDpsText,
            UiTextBundle {
                text: player_dps_text(player_damage.per_second),
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
                    font: font.clone(),
                    font_size: FontSize::Px(32.),
                    ..default()
                },
                layout: TextLayout::justify(Justify::Center),
            },
        ));
}

fn update_fps_text(
    mut fps_text_query: Query<&mut Text, With<FramePerSecondText>>,
    diagnostic: Res<DiagnosticsStore>,
) {
    let Ok(mut fps) = fps_text_query.single_mut() else {
        return;
    };

    *fps = fps_text(diagnostic).into();
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

fn update_player_dps_text(
    mut player_dps_query: Query<&mut Text, With<PlayerDpsText>>,
    player_damage: Res<PlayerDamage>,
) {
    let Ok(mut dps_text) = player_dps_query.single_mut() else {
        return;
    };

    *dps_text = player_dps_text(player_damage.per_second);
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

fn fps_text(diagnostic: Res<DiagnosticsStore>) -> Text {
    let Some(fps_diag) = diagnostic.get(&FrameTimeDiagnosticsPlugin::FPS) else {
        return Text::default();
    };

    let Some(fps_avg) = fps_diag.average() else {
        return Text::default();
    };

    format!("FPS {:.0}", fps_avg).into()
}

fn enemy_count_text(enemy_count: usize) -> Text {
    format!("Enemy Count {}", enemy_count).into()
}

fn player_dps_text(bank_value: f32) -> Text {
    format!("Player DPS {:.2}", bank_value).into()
}

fn bank_budget_text(bank_budget: f32) -> Text {
    format!("Bank Budget {:.2}", bank_budget).into()
}

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin::default())
            .add_systems(
                Update,
                (
                    setup_debug_ui_nodes,
                    update_fps_text,
                    update_enemy_count_text,
                    update_player_dps_text,
                    update_bank_budget_text,
                )
                    .in_set(InGameSystems),
            );
    }
}
