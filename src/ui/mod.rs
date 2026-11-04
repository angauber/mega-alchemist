#[cfg(feature = "dev_tools")]
use crate::ui::debug::DebugPlugin;
use crate::ui::game::GameUiPlugin;
use crate::ui::text::TextUiPlugin;
use crate::ui::transmutation::TransmutationUiPlugin;
use bevy::prelude::*;

#[cfg(feature = "dev_tools")]
pub mod debug;
pub(crate) mod game;
pub(crate) mod text;
pub(crate) mod transmutation;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TextUiPlugin)
            .add_plugins(GameUiPlugin)
            .add_plugins(TransmutationUiPlugin);

        #[cfg(feature = "dev_tools")]
        {
            app.add_plugins(DebugPlugin);
        }
    }
}
