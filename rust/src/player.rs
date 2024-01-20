use godot::engine::CharacterBody3D;
use godot::prelude::*;

use crate::health_component::HealthComponent;
use crate::movement_component::MovementComponent;

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
pub struct Prayer {
    health_component: HealthComponent,
    movement_component: MovementComponent,
    #[base]
    base: Base<CharacterBody3D>,
}
