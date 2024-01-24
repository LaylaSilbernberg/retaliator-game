use ::godot::engine::Node3D;
use ::godot::prelude::*;

use crate::health_component::HealthComponent;
use crate::player_body::PlayerBody;

#[derive(GodotClass)]
#[class(init, base = Node3D)]
pub struct Player {
    #[export]
    player_body: Option<Gd<PlayerBody>>,
    #[export]
    health_component: Option<Gd<HealthComponent>>,
    #[base]
    base: Base<Node3D>,
}
#[godot_api]
impl INode3D for Player {
    fn ready(&mut self) {
        let mut health: Gd<HealthComponent> = self
            .get_health_component()
            .expect("HealthComponent needs to be defined.");
        health.bind_mut().initialize_health();
    }
}
