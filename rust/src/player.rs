use godot::engine::input::MouseMode;
use godot::engine::{CharacterBody3D, ICharacterBody3D};
use godot::obj::WithBaseField;
use godot::prelude::*;

use crate::health_component::HealthComponent;
use crate::player_variables::PlayerVariables;

#[derive(GodotClass)]
#[class(init, base=CharacterBody3D)]
pub struct Player {
    #[export]
    health_component: Gd<HealthComponent>,
    #[export]
    speed: real,
    #[export]
    gravity: real,
    #[var]
    velocity: Vector3,
    #[base]
    base: Base<CharacterBody3D>,
}
#[godot_api]
impl Player {
    #[func]
    fn get_input(&mut self) -> Vector3 {
        let mut input_dir: Vector3 = Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let global_transform = self.base_mut().get_global_transform().basis;
        let input: Gd<Input> = Input::singleton();

        if input.is_action_pressed("move_forward".into()) {
            input_dir += -global_transform.col_c();
        }
        if input.is_action_pressed("move_backward".into()) {
            input_dir += global_transform.col_c();
        }
        if input.is_action_pressed("strafe_left".into()) {
            input_dir += -global_transform.col_a();
        }
        if input.is_action_pressed("strage_right".into()) {
            input_dir += global_transform.col_a();
        }
        input_dir
    }
}
#[godot_api]
impl ICharacterBody3D for Player {
    fn ready(&mut self) {
        let max_health = self.get_health_component().bind().get_max_health();
        self.health_component.bind_mut().set_health(max_health);

        Input::singleton().set_mouse_mode(MouseMode::CAPTURED);

        let player_vars = self
            .base()
            .get_node_as::<PlayerVariables>("PlayerVariables");

        self.set_speed(player_vars.bind().get_max_speed());
        self.set_gravity(player_vars.bind().get_gravity());
    }

    fn physics_process(&mut self, delta: f64) {
        self.velocity.y += self.get_gravity() * delta as f32;

        let desired_velocity = self.get_input() * self.get_speed();

        self.velocity.x = desired_velocity.x;
        self.velocity.z = desired_velocity.z;
        self.base_mut().move_and_slide();
    }
}
