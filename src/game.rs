use crate::agent::Enemy;
use crate::build::PlayerBuild;
use crate::ui::text::RenderUiText;
#[cfg(feature = "dev_tools")]
use bevy::prelude::KeyCode::Space;
use bevy::prelude::*;
use bevy::time::Stopwatch;
use bevy_ecs_ldtk::prelude::*;
use std::time::Duration;

pub struct GamePlugin;

#[derive(Resource, Default)]
pub struct GameClock(pub Stopwatch);

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum GameState {
    #[default]
    Loading,
    InGame,
    GameOver,
}

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct PauseState(pub bool);

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

// on translate to game over
fn on_game_over(
    _event: On<GameOver>,
    mut commands: Commands,
    mut ui_text_messages: MessageWriter<RenderUiText>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_pause_state: ResMut<NextState<PauseState>>,
) {
    next_game_state.set(GameState::GameOver);
    next_pause_state.set(PauseState(true));

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
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_pause_state: ResMut<NextState<PauseState>>,
    mut game_clock: ResMut<GameClock>,
    mut player_build: ResMut<PlayerBuild>,
) {
    info!("respawning level !");
    game_clock.0.reset();

    *player_build = PlayerBuild::default();

    *level_selection = LevelSelection::index(0);

    commands
        .entity(ldtk_projects.single().unwrap())
        .insert(Respawn);

    next_game_state.set(GameState::InGame);
    next_pause_state.set(PauseState(false));
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

fn tick_game_clock(mut game_clock: ResMut<GameClock>, time: Res<Time>) {
    game_clock.0.tick(time.delta());
}

#[cfg(feature = "dev_tools")]
fn toggle_game_state_on_space(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    pause_state: ResMut<State<PauseState>>,
    mut next_state: ResMut<NextState<PauseState>>,
) {
    if keyboard_input.just_pressed(Space) {
        next_state.set(match pause_state.get() {
            PauseState(false) => PauseState(true),
            PauseState(true) => PauseState(false),
        });
    }
}

fn count_enemies(mut enemy_counter: ResMut<EnemyCounter>, enemy_query: Query<Entity, With<Enemy>>) {
    enemy_counter.count = enemy_query.iter().count();
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<PauseState>()
            .init_state::<GameState>()
            .configure_sets(
                Update,
                InGameSystems
                    .run_if(in_state(GameState::InGame).and_then(in_state(PauseState(false)))),
            )
            .configure_sets(
                PreUpdate,
                InGameSystems
                    .run_if(in_state(GameState::InGame).and_then(in_state(PauseState(false)))),
            )
            .init_resource::<GameClock>()
            .init_resource::<EnemyCounter>()
            .add_observer(on_game_over)
            .add_observer(on_game_over_with_delay)
            .add_systems(PreUpdate, trigger_game_over_with_delay)
            .add_systems(PreUpdate, tick_game_clock.in_set(InGameSystems))
            .add_systems(Update, count_enemies.in_set(InGameSystems));

        #[cfg(feature = "dev_tools")]
        app.add_systems(Update, toggle_game_state_on_space);
    }
}
