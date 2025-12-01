use bevy::prelude::*;

use crate::action::{button::ActionButtonPlugin, gate::ActionGatePlugin};

mod button;
mod gate;

pub struct ActionPlugin;
impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ActionGatePlugin)
            .add_plugins(ActionButtonPlugin);
    }
}
