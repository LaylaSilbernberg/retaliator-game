use godot::prelude::*;

#[path = "enums/animations.rs"]
mod animations;
mod arm;
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
mod weapon_statistics;

struct Retaliator;

#[gdextension]
unsafe impl ExtensionLibrary for Retaliator {}
