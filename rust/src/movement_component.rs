use godot::engine::Node3D;
use godot::obj::WithBaseField;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base = Node3D, init)]
pub struct MovementComponent {
    #[export]
    speed: f64,
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
        let mut velocity: Vector3 = Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let global_transform = self.to_gd().get_global_transform().basis;
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
        input_dir.normalized()
    }
}
