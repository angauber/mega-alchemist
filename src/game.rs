use crate::agent::Enemy;
use crate::health::Health;
use crate::player::Player;
use crate::ui::{RenderUiText, UiTextBundle};
use bevy::prelude::KeyCode::Space;
use bevy::prelude::*;
use bevy::time::Stopwatch;
use bevy_ecs_ldtk::prelude::*;
use std::time::Duration;

pub struct GamePlugin;

#[derive(Resource, Default)]
pub struct GameClock(Stopwatch);

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
pub struct PlayerHealthUI;

#[derive(Component, Default)]
pub struct GameClockUI;

#[derive(Resource, Default)]
pub struct EnemyCounter {
    pub count: usize,
}

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
    game_clock: Res<GameClock>,
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
                align_items: AlignItems::FlexStart,
                top: Val::Px(50.0),
                left: Val::Px(50.0),
                ..default()
            },
        ))
        .with_child((
            PlayerHealthUI,
            UiTextBundle {
                text: format!("HP {}", player_health.point).into(),
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
                text: format!("Time {:.2}", game_clock.0.elapsed_secs()).into(),
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

    player_health_text.0 = format!("{} HP", player_health.point);
}

fn on_game_over(
    _event: On<GameOver>,
    mut commands: Commands,
    mut ui_text_messages: MessageWriter<RenderUiText>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    next_state.set(GameState::Paused);
    commands.spawn(GameOverTimer::new(Duration::from_secs(4)));

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
    main_text_ui_query: Query<Entity, With<GameMainTextUiNode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut game_clock: ResMut<GameClock>,
) {
    info!("respawning level !");
    if let Ok(game_ui) = main_text_ui_query.single() {
        commands.entity(game_ui).despawn();
    }

    game_clock.0.reset();

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

fn tick_game_clock(
    mut game_clock: ResMut<GameClock>,
    time: Res<Time>,
    mut game_clock_text_query: Query<&mut Text, With<GameClockUI>>,
) {
    game_clock.0.tick(time.delta());

    let Ok(mut game_clock_text) = game_clock_text_query.single_mut() else {
        return;
    };

    game_clock_text.0 = format!("Time {:.2}", game_clock.0.elapsed_secs());
}

fn update_game_clock_text() {}

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

fn count_enemies(mut enemy_counter: ResMut<EnemyCounter>, enemy_query: Query<Entity, With<Enemy>>) {
    enemy_counter.count = enemy_query.iter().count();
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .configure_sets(Update, InGameSystems.run_if(in_state(GameState::InGame)))
            .configure_sets(PreUpdate, InGameSystems.run_if(in_state(GameState::InGame)))
            .init_resource::<GameClock>()
            .init_resource::<EnemyCounter>()
            .add_observer(on_game_over)
            .add_observer(on_game_over_with_delay)
            .add_systems(PreUpdate, trigger_game_over_with_delay)
            .add_systems(PreUpdate, tick_game_clock.in_set(InGameSystems))
            .add_systems(
                Update,
                (toggle_game_state_on_space, update_player_health_text),
            )
            .add_systems(
                Update,
                (
                    count_enemies,
                    spawn_game_main_text_ui_node,
                    update_game_clock_text,
                )
                    .in_set(InGameSystems),
            );
    }
}
