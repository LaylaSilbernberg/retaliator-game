use godot::prelude::*;

#[path = "traits/damageable.rs"]
mod damageable;
mod global_state;
mod gun;
#[path = "traits/health.rs"]
mod health;
mod health_component;
mod muzzle_flash;
mod player;
mod player_head;
mod player_variables;
mod prod_grunt;

struct Retaliator;

#[gdextension]
unsafe impl ExtensionLibrary for Retaliator {}
