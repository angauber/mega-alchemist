#[cfg(feature = "dev_tools")]
use crate::ui::debug::DebugPlugin;
use crate::ui::game::GameUiPlugin;
use crate::ui::text::TextUiPlugin;
use bevy::prelude::*;

#[cfg(feature = "dev_tools")]
pub mod debug;
pub(crate) mod game;
pub(crate) mod text;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TextUiPlugin).add_plugins(GameUiPlugin);

        #[cfg(feature = "dev_tools")]
        {
            app.add_plugins(DebugPlugin);
        }
    }
}
