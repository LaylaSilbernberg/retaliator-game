use godot::engine::CharacterBody3D;
use godot::prelude::*;

use crate::health_component::HealthComponent;
use crate::movement_component::MovementComponent;

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
pub struct Player {
    #[export]
    health_component: Gd<HealthComponent>,
    #[export]
    movement_component: Gd<MovementComponent>,
    #[base]
    base: Base<CharacterBody3D>,
}
