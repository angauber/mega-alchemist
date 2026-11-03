use crate::game::{GameMainTextUiNode, InGameSystems};
use crate::spawn_manager::SpawnManager;
use crate::ui::UiTextBundle;
use bevy::prelude::*;

pub struct DebugPlugin;

#[derive(Component)]
pub struct DebugSpawnerSettingUI;

fn setup_debug_nodes(
    mut commands: Commands,
    game_ui_query: Query<Entity, Added<GameMainTextUiNode>>,
    spawn_manager: Res<SpawnManager>,
    asset_server: Res<AssetServer>,
) {
    let Ok(game_ui_entity) = game_ui_query.single() else {
        return;
    };

    let font_handle = asset_server.load("fonts/Pixuf.ttf");

    commands.entity(game_ui_entity).with_child((
        DebugSpawnerSettingUI,
        UiTextBundle {
            text: format!(
                "Bank Value: {:.2} | Bank Budget: {:.2}",
                spawn_manager.bank_value(),
                spawn_manager.bank_budget()
            )
            .into(),
            color: TextColor(Color::WHITE),
            font: TextFont {
                font: font_handle.into(),
                font_size: FontSize::Px(32.),
                ..default()
            },
            layout: TextLayout::justify(Justify::Center),
        },
    ));
}

fn update_debug_nodes(
    mut spawn_setting_text_query: Query<&mut Text, With<DebugSpawnerSettingUI>>,
    spawn_manager: Res<SpawnManager>,
) {
    let Ok(mut spawn_setting_text) = spawn_setting_text_query.single_mut() else {
        return;
    };

    spawn_setting_text.0 = format!(
        "Bank Value: {:.2} | Bank Budget: {:.2}",
        spawn_manager.bank_value(),
        spawn_manager.bank_budget()
    )
    .into();
}

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (setup_debug_nodes, update_debug_nodes).in_set(InGameSystems),
        );
    }
}
