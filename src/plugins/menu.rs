mod main_menu;

use bevy::prelude::*;
use crate::GameState;

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
        
    }
}