mod components;
mod styles;
mod systems;

use systems::interactions::*;
use systems::layout::*;

use bevy::prelude::*;

pub struct FileMenuPlugin;

impl Plugin for FileMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter State Systems
            .add_startup_system(spawn_menu)
            // Systems
            .add_systems(
                (interact_with_save_button, 
                interact_with_load_button,
                interact_with_undo_button,
                interact_with_redo_button,
                interact_with_switch_projection_button));
    }
}