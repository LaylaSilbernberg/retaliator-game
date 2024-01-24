use godot::engine::Resource;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Resource)]
pub struct HealthComponent {
    #[var]
    health: i32,
    #[export]
    max_health: i32,
    #[base]
    base: Base<Resource>,
}
#[godot_api]
impl HealthComponent {
    #[func]
    pub fn initialize_health(&mut self) {
        self.health = self.max_health;
    }
}
