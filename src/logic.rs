use crate::selection::Moveable;
use bevy::prelude::*;

pub struct LogicPlugin;
impl Plugin for LogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_logic_system);
    }
}

#[derive(Component, Default)]
pub struct Item;

#[derive(Component)]
#[require(Item, Moveable)]
pub enum Gate {
    And(Option<Entity>, Option<Entity>),
    Or(Option<Entity>, Option<Entity>),
    Not(Option<Entity>),
}

#[derive(Component, Default)]
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
                let Ok(a_value) = values.get(*a) else {
                    return None;
                };
                let Ok(b_value) = values.get(*b) else {
                    return None;
                };

                Some(a_value.state && b_value.state)
            }
            Gate::Or(a, b) => {
                let Some(a) = a else {
                    return None;
                };
                let Some(b) = b else {
                    return None;
                };
                let Ok(a_value) = values.get(*a) else {
                    return None;
                };
                let Ok(b_value) = values.get(*b) else {
                    return None;
                };
                Some(a_value.state || b_value.state)
            }
            Gate::Not(a) => {
                let Some(a) = a else {
                    return None;
                };

                let Ok(a_value) = values.get(*a) else {
                    return None;
                };

                Some(!a_value.state)
            }
        }
    }
}

pub fn update_logic_system(
    mut gates: Query<(&mut Gate, Entity)>,
    values: Query<&Value>,
    mut commands: Commands,
) {
    for (mut gate, entity) in gates.iter_mut() {
        if let Some(state) = gate.evaluate(&values) {
            commands.entity(entity).insert(Value { state });
        } else {
            *gate = match &*gate {
                Gate::And(a, b) => {
                    let mut new_a = None;
                    let mut new_b = None;

                    if let Some(original_a) = a {
                        if commands.get_entity(*original_a).is_ok() {
                            new_a = Some(*original_a);
                        }
                    }
                    if let Some(original_b) = b {
                        if commands.get_entity(*original_b).is_ok() {
                            new_b = Some(*original_b);
                        }
                    }
                    Gate::And(new_a, new_b)
                }
                Gate::Or(a, b) => {
                    let mut new_a = None;
                    let mut new_b = None;

                    if let Some(original_a) = a {
                        if commands.get_entity(*original_a).is_ok() {
                            new_a = Some(*original_a);
                        }
                    }
                    if let Some(original_b) = b {
                        if commands.get_entity(*original_b).is_ok() {
                            new_b = Some(*original_b);
                        }
                    }
                    Gate::Or(new_a, new_b)
                }
                Gate::Not(a) => {
                    let mut new_a = None;

                    if let Some(original_a) = a {
                        if commands.get_entity(*original_a).is_ok() {
                            new_a = Some(*original_a);
                        }
                    }
                    Gate::Not(new_a)
                }
            };
            commands.entity(entity).try_remove::<Value>();
        }
    }
}

#[derive(Component)]
#[require(Value, Item, Moveable)]
pub struct LogicButton;
