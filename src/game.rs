use crate::health::Health;
use crate::player::Player;
use crate::ui::{RenderUiText, UiTextBundle};
use bevy::prelude::KeyCode::Space;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use std::time::Duration;

pub struct GamePlugin;

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum GameState {
    #[default]
    InGame,
    Paused,
}

#[derive(SystemSet, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct InGameSystems;

#[derive(Event)]
pub struct GameOver;
#[derive(Event)]
pub struct GameOverWithDelay;

#[derive(Component)]
struct GameOverTimer {
    timer: Timer,
}

#[derive(Component, Default)]
pub struct GameMainTextUiNode;

#[derive(Component, Default)]
pub struct PlayerHealth;

impl GameOverTimer {
    fn new(duration: Duration) -> Self {
        Self {
            timer: Timer::new(duration, TimerMode::Once),
        }
    }
}

fn spawn_game_main_text_ui_node(
    mut commands: Commands,
    player_query: Query<&Health, Added<Player>>,
    assets: Res<AssetServer>,
) {
    let Ok(player_health) = player_query.single() else {
        return;
    };

    info!(
        "adding main text game ui with player health: {:?}",
        player_health
    );

    let font_handle = assets.load("fonts/Pixuf.ttf");

    commands
        .spawn((
            GameMainTextUiNode,
            Node {
                position_type: PositionType::Absolute,
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                top: Val::Px(50.0),
                left: Val::Px(50.0),
                ..default()
            },
        ))
        .with_child((
            PlayerHealth,
            UiTextBundle {
                text: format!("{} HP", player_health.point).into(),
                color: TextColor(Color::WHITE),
                font: TextFont {
                    font: font_handle.into(),
                    font_size: FontSize::Px(64.),
                    ..default()
                },
                layout: TextLayout::justify(Justify::Center),
            },
        ));
}

fn update_player_health_text(
    player_health_query: Query<&Health, (Changed<Health>, With<Player>)>,
    mut player_health_text_query: Query<&mut Text, With<PlayerHealth>>,
) {
    let Ok(player_health) = player_health_query.single() else {
        return;
    };

    let Ok(mut player_health_text) = player_health_text_query.single_mut() else {
        return;
    };

    player_health_text.0 = format!("{} HP", player_health.point);
}

fn on_game_over(
    _event: On<GameOver>,
    mut commands: Commands,
    mut ui_text_messages: MessageWriter<RenderUiText>,
    main_text_ui_query: Query<Entity, With<GameMainTextUiNode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    next_state.set(GameState::Paused);
    commands.spawn(GameOverTimer::new(Duration::from_secs(4)));

    if let Ok(game_ui) = main_text_ui_query.single() {
        commands.entity(game_ui).despawn();
    }

    ui_text_messages.write(RenderUiText {
        text: Text2d::new("GAME OVER"),
        duration: Duration::from_secs(4),
        ..default()
    });
}

fn on_game_over_with_delay(
    _event: On<GameOverWithDelay>,
    mut commands: Commands,
    mut level_selection: ResMut<LevelSelection>,
    ldtk_projects: Query<Entity, With<LdtkProjectHandle>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    *level_selection = LevelSelection::index(0);

    commands
        .entity(ldtk_projects.single().unwrap())
        .insert(Respawn);

    next_state.set(GameState::InGame);
}

fn trigger_game_over_with_delay(
    time: Res<Time>,
    mut commands: Commands,
    mut game_over_timer_query: Query<(Entity, &mut GameOverTimer)>,
) {
    for (entity, mut event) in &mut game_over_timer_query {
        event.timer.tick(time.delta());

        if event.timer.is_finished() {
            commands.trigger(GameOverWithDelay);

            // Despawn the timer entity
            commands.entity(entity).despawn();
        }
    }
}

fn toggle_game_state_on_space(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(Space) {
        next_state.set(match current_state.get() {
            GameState::InGame => GameState::Paused,
            GameState::Paused => GameState::InGame,
        });
    }
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .configure_sets(Update, InGameSystems.run_if(in_state(GameState::InGame)))
            .add_observer(on_game_over)
            .add_observer(on_game_over_with_delay)
            .add_systems(PreUpdate, trigger_game_over_with_delay)
            .add_systems(Update, toggle_game_state_on_space)
            .add_systems(Update, spawn_game_main_text_ui_node)
            .add_systems(Update, update_player_health_text);
    }
}
