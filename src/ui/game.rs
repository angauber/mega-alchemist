use crate::game::GameClock;
use crate::health::Health;
use crate::player::Player;
use crate::ui::text::UiTextBundle;
use bevy::app::{App, Plugin};
use bevy::asset::AssetServer;
use bevy::color::Color;
use bevy::prelude::*;

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
struct GameClockUI;

fn setup_game_ui_node(mut commands: Commands, assets: Res<AssetServer>) {
    let font_handle = assets.load("fonts/Pixuf.ttf");

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
        .with_child((
            PlayerHealthUI,
            UiTextBundle {
                text: Text::default(),
                color: TextColor(Color::WHITE),
                font: TextFont {
                    font: font_handle.clone().into(),
                    font_size: FontSize::Px(64.),
                    ..default()
                },
                layout: TextLayout::justify(Justify::Center),
            },
        ))
        .with_child((
            GameClockUI,
            UiTextBundle {
                text: Text::default(),
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

fn format_player_health_text(health: &Health) -> Text {
    format!("HP {} / {}", health.points, health.max).into()
}

fn update_player_health_text(
    player_health_query: Query<&Health, (Changed<Health>, With<Player>)>,
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

fn update_game_clock_text(
    game_clock: Res<GameClock>,
    mut game_clock_text_query: Query<&mut Text, With<GameClockUI>>,
) {
    let Ok(mut game_clock_text) = game_clock_text_query.single_mut() else {
        return;
    };

    game_clock_text.0 = format!("Time {:.2}", game_clock.0.elapsed_secs());
}

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_game_ui_node).add_systems(
            PostUpdate,
            (update_player_health_text, update_game_clock_text),
        );
    }
}
