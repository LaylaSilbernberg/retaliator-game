use godot::engine::{input, Node3D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(tool, init, base = Node3D)]
pub struct MovementComponent {
    #[export]
    speed: real,
    #[export]
    #[init(default = Vector3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    })]
    velocity: Vector3,
    #[base]
    base: Base<Node3D>,
}
#[godot_api]
impl MovementComponent {
    #[func]
    fn get_input(&mut self) -> Vector3 {
        let mut input_dir: Vector3 = Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let global_transform = Self::to_gd(self).get_global_transform().basis;
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
impl INode3D for MovementComponent {
    fn ready(&mut self) {
        Input::singleton().set_mouse_mode(input::MouseMode::MOUSE_MODE_CAPTURED)
    }
}
