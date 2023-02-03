mod main_menu;

use bevy::prelude::*;
use kayak_ui::{prelude::KayakContextPlugin, widgets::KayakWidgets};
use crate::{GameState, utils::state_helper::StateExtend};

use self::main_menu::{spawn_main_menu, PreloadResource};

pub struct MenuPlugin {
    run_in: Option<GameState>
}
impl MenuPlugin {
    pub fn new(state: Option<GameState>) -> Self {
        Self {
            run_in: state
        }
    }
}

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<PreloadResource>()
        .add_plugin(KayakContextPlugin)
        .add_plugin(KayakWidgets)
        .add_startup_system_if_state(self.run_in,spawn_main_menu);
    }
}