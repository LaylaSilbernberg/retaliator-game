use godot::prelude::*;

mod global_state;
mod health_component;
mod player;
mod player_variables;

struct Retaliator;

#[gdextension]
unsafe impl ExtensionLibrary for Retaliator {}
