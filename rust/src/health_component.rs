use godot::engine::Resource;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Resource)]
pub struct HealthComponent {
    #[export]
    health: i32,
    #[export]
    max_health: i32,
    #[base]
    base: Base<Resource>,
}
