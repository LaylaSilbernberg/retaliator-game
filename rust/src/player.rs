use godot::engine::CharacterBody3d;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=CharacterBody3d)]
pub struct Prayer {
    #[export]
    health_component: HealthComponent,
    movement_component: MovementComponent,
    #[base]
    base: Base<CharacterBody3d>,
}
