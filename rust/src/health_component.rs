use godot::engine::Node3D;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(tool, init, base=Node3D)]
pub struct HealthComponent {
    health: i32,
    #[export]
    max_health: i32,
    #[base]
    base: Base<Node3D>,
}
#[godot_api]
impl INode3D for HealthComponent {
    fn ready(&mut self) {
        self.health = self.max_health;
    }
}
