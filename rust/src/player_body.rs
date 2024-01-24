use core::f32;

use godot::engine::input::MouseMode;
use godot::engine::utilities::{clampf, deg_to_rad};
use godot::engine::{CharacterBody3D, ICharacterBody3D, InputEvent, InputEventMouseMotion};
use godot::obj::WithBaseField;
use godot::prelude::*;

use crate::player_head::PlayerHead;
use crate::player_variables::PlayerVariables;

#[derive(GodotClass)]
#[class(init, base=CharacterBody3D)]
pub struct PlayerBody {
    #[export]
    head: Option<Gd<PlayerHead>>,
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
impl PlayerBody {
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
impl ICharacterBody3D for PlayerBody {
    fn ready(&mut self) {
        Input::singleton().set_mouse_mode(MouseMode::CAPTURED);

        let player_vars = self
            .base()
            .get_node_as::<PlayerVariables>("PlayerVariables");

        self.set_speed(player_vars.bind().get_max_speed());
        self.set_gravity(player_vars.bind().get_gravity());
    }

    fn unhandled_input(&mut self, event: Gd<InputEvent>) {
        if let Ok(event_motion) = event.try_cast::<InputEventMouseMotion>() {
            let mut head = self.get_head().expect("Head must be initialised");
            let mut camera = head
                .bind_mut()
                .get_camera()
                .expect("Camera must be initialised");
            let sensitivity = self
                .base_mut()
                .get_node_as::<PlayerVariables>("PlayerVariables")
                .bind()
                .get_mouse_sensitivity();
            head.rotate_y(-event_motion.get_relative().x * sensitivity);
            camera.rotate_x(-event_motion.get_relative().y * sensitivity);
            camera.get_rotation().x = clampf(
                camera.get_rotation().x as f64,
                deg_to_rad(-40.0),
                deg_to_rad(60.0),
            ) as f32;
        }
    }

    fn physics_process(&mut self, delta: f64) {
        self.velocity.y += self.get_gravity() * delta as f32;
        let mut head = self.get_head().expect("Head must be initialised");
        let direction = head.bind_mut().get_head_transform_basis() * self.get_input();
        let desired_velocity = direction * self.get_speed();

        self.velocity.x = desired_velocity.x;
        self.velocity.z = desired_velocity.z;
        self.base_mut().move_and_slide();
    }
}
