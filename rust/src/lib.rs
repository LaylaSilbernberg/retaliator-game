use godot::prelude::*;

mod global_state;
mod health_component;
mod player;
mod player_head;
mod player_variables;
mod prod_grunt;

struct Retaliator;

#[gdextension]
unsafe impl ExtensionLibrary for Retaliator {}
