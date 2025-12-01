use bevy::{input::keyboard::KeyboardInput, prelude::*};

use crate::{
    logic::{LogicButton, Value},
    selection::Selected,
};

pub struct ActionButtonPlugin;
impl Plugin for ActionButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, change_button_value);
    }
}

fn change_button_value(
    mut button_query: Query<&mut Value, (With<LogicButton>, With<Selected>)>,
    mut inputs: MessageReader<KeyboardInput>,
) {
    for input in inputs.read() {
        if input.key_code == KeyCode::Space && input.state.is_pressed() {
            for mut value in button_query.iter_mut() {
                *value = Value {
                    state: !value.state,
                };
            }
        }
    }
}
