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
    health_component: Gd<HealthComponent>,
    #[base]
    base: Base<Node3D>,
}
#[godot_api]
impl INode3D for Player {
    fn ready(&mut self) {
        self.health_component.bind_mut().initialize_health();
    }
}
