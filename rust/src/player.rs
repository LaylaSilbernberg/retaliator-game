use godot::engine::CharacterBody3d;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=CharacterBody3d)]
struct Prayer {
    #[export]
    health_component: HealthComponent,
    #[base]
    base: Base<CharacterBody3d>,
}
