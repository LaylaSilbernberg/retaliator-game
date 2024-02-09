use godot::obj::Gd;

use crate::health_component::HealthComponent;

pub trait Health {
    fn get_health_component(&self) -> Gd<HealthComponent>;
}
