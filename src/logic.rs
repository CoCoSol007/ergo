use bevy::prelude::*;
mod display;

pub struct LogicPlugin;
impl Plugin for LogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_logic_system)
            .add_systems(Update, display::display_gates)
            .add_systems(Update, display::update_gate_colors);
    }
}

#[derive(Component)]
pub enum Gate {
    And(Option<Entity>, Option<Entity>),
    Or(Option<Entity>, Option<Entity>),
    Not(Option<Entity>),
}

#[derive(Component)]
pub struct Value {
    pub state: bool,
}

impl Gate {
    fn evaluate(&self, values: &Query<&Value>) -> Option<bool> {
        match self {
            Gate::And(a, b) => {
                let Some(a) = a else {
                    return None;
                };
                let Some(b) = b else {
                    return None;
                };
                let a_val = values.get(*a).map_or(false, |v| v.state);
                let b_val = values.get(*b).map_or(false, |v| v.state);
                Some(a_val && b_val)
            }
            Gate::Or(a, b) => {
                let Some(a) = a else {
                    return None;
                };
                let Some(b) = b else {
                    return None;
                };
                let a_val = values.get(*a).map_or(false, |v| v.state);
                let b_val = values.get(*b).map_or(false, |v| v.state);
                Some(a_val || b_val)
            }
            Gate::Not(a) => {
                let Some(a) = a else {
                    return None;
                };
                let a_val = values.get(*a).map_or(false, |v| v.state);
                Some(!a_val)
            }
        }
    }
}

fn update_logic_system(
    gates: Query<(&Gate, Entity)>,
    values: Query<&Value>,
    mut commands: Commands,
) {
    for (gate, entity) in gates.iter() {
        if let Some(state) = gate.evaluate(&values) {
            commands.entity(entity).insert(Value { state });
        } else {
            commands.entity(entity).try_remove::<Value>();
        }
    }
}
