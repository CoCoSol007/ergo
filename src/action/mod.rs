use bevy::prelude::*;

use crate::action::gate::ActionGatePlugin;

pub mod gate;

pub struct ActionPlugin;
impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ActionGatePlugin);
    }
}
