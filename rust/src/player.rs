use ::godot::engine::{Engine, Node3D};
use ::godot::prelude::*;

use crate::health_component::HealthComponent;
use crate::player_body::PlayerBody;
use crate::player_variables::PlayerVariables;

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
    fn process(&mut self, _delta: f64) {
        let player_vars: Option<Gd<Object>> =
            Engine::singleton().get_singleton("PlayerVariables".into());

        match player_vars {
            Some(_result) => println!("player_variables exists"),
            _ => println!("player_variables does not exist"),
        }
    }
}
