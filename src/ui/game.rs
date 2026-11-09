use crate::assets::FontAssets;
use crate::game::{GameState, InGameSystems};
use crate::health::Health;
use crate::level::PlayerLevel;
use crate::player::Player;
use crate::ui::text::ui_text;
use bevy::app::{App, Plugin};
use bevy::color::Color;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub struct GameUiPlugin;

pub const BROWN: Color = Color::srgb_u8(26, 19, 15);
pub const DARK_GRAY: Color = Color::srgb_u8(50, 60, 57);
#[allow(dead_code)]
pub const BLUE: Color = Color::srgb_u8(16, 22, 46);
#[allow(dead_code)]
pub const PURPLE: Color = Color::srgb_u8(28, 20, 41);
#[allow(dead_code)]
pub const LIGHT_BLUE: Color = Color::srgb_u8(116, 143, 147);
#[allow(dead_code)]
pub const LIGHT_GRAY: Color = Color::srgb_u8(105, 106, 106);

#[derive(Component, Default)]
pub struct GameMainTextUiNode;

#[derive(Component, Default)]
struct PlayerHealthUI;

#[derive(Component, Default)]
struct PlayerLevelUI;

#[derive(Component, Default)]
struct PlayerXPUI;

fn setup_game_ui_node(mut commands: Commands, font_assets: Res<FontAssets>) {
    let font = FontSource::Handle(font_assets.pixuf_font.clone());

    commands
        .spawn((
            GameMainTextUiNode,
            Node {
                position_type: PositionType::Absolute,
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexStart,
                top: Val::Px(50.0),
                left: Val::Px(50.0),
                ..default()
            },
        ))
        .with_child((PlayerHealthUI, ui_text(font.clone(), 64.)))
        .with_child((PlayerLevelUI, ui_text(font.clone(), 32.)))
        .with_child((PlayerXPUI, ui_text(font.clone(), 32.)));
}

fn format_player_health_text(health: &Health) -> Text {
    format!("HP {} / {}", health.points, health.max).into()
}

fn update_player_health_text(
    player_health_query: Query<&Health, With<Player>>,
    mut player_health_text_query: Query<&mut Text, With<PlayerHealthUI>>,
) {
    let Ok(player_health) = player_health_query.single() else {
        return;
    };

    let Ok(mut player_health_text) = player_health_text_query.single_mut() else {
        return;
    };

    *player_health_text = format_player_health_text(player_health);
}

fn update_level_text(
    player_level: Res<PlayerLevel>,
    mut text_query: Query<&mut Text, With<PlayerLevelUI>>,
) {
    let Ok(mut text) = text_query.single_mut() else {
        return;
    };

    text.0 = format!("Alchemy Level {}", player_level.level());
}

fn update_xp_text(
    player_level: Res<PlayerLevel>,
    mut text_query: Query<&mut Text, With<PlayerXPUI>>,
) {
    let Ok(mut text) = text_query.single_mut() else {
        return;
    };

    text.0 = format!("XP {}", player_level.xp());
}

fn update_ui_scale(window: Single<&Window, With<PrimaryWindow>>, mut ui_scale: ResMut<UiScale>) {
    let scale = window.height() / 1200.0;

    if ui_scale.0 != scale {
        ui_scale.0 = scale;
    }
}

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_ui_scale)
            .add_systems(
                OnTransition {
                    exited: GameState::Loading,
                    entered: GameState::InGame,
                },
                setup_game_ui_node,
            )
            .add_systems(
                PostUpdate,
                (update_player_health_text, update_level_text, update_xp_text)
                    .in_set(InGameSystems),
            );
    }
}
