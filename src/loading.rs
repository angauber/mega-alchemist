use crate::asset_tracking::ResourceHandles;
use crate::game::GameState;
use bevy::prelude::*;

pub struct LoadingPlugin;

fn spawn_loading_screen() {
    info!("Loading...");
}

fn enter_gameplay_screen(mut next_screen: ResMut<NextState<GameState>>) {
    info!("Entering InGame state");
    next_screen.set(GameState::InGame);
}

fn all_assets_loaded(resource_handles: Res<ResourceHandles>) -> bool {
    resource_handles.is_all_done()
}

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Loading), spawn_loading_screen);
        app.add_systems(
            Update,
            enter_gameplay_screen.run_if(in_state(GameState::Loading).and_then(all_assets_loaded)),
        );
    }
}
