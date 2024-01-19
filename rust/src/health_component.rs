use godot::engine::Resource;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Resource)]
pub struct HealthComponent {
    #[export]
    health: i32,
    #[base]
    base: Base<Resource>,
}