use std::i32::MIN;

use godot::engine::Resource;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Resource)]
pub struct HealthComponent {
    #[export]
    health: i32,
    #[export]
    max_health: i32,
    base: Base<Resource>,
}
#[godot_api]
impl HealthComponent {
    pub fn take_damage(&mut self, damage: i32) {
        self.health -= damage;
        if self.health <= 0 {
            self.death();
        }
    }
    pub fn death(&self) -> Option<StringName> {
        match self.health {
            -20..=0 => Some("death".into()),
            MIN..=-21 => Some("explode".into()),
            _ => None,
        }
    }
}
