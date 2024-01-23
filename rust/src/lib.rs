use godot::prelude::*;

mod global_state;
mod health_component;
mod movement_component;
mod player;
mod player_variables;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
