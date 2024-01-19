use godot::prelude::*;

mod health_component;
mod movement_component;
mod player;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
