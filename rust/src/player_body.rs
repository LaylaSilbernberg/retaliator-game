use core::f32;

use godot::engine::input::MouseMode;
use godot::engine::utilities::{clampf, deg_to_rad};
use godot::engine::{CharacterBody3D, ICharacterBody3D, InputEvent, InputEventMouseMotion};
use godot::obj::WithBaseField;
use godot::prelude::*;

use crate::player_head::PlayerHead;

#[derive(GodotClass)]
#[class(init, base=CharacterBody3D)]
pub struct PlayerBody {
    #[export]
    #[init(default = 0.002)]
    sensitivity: real,
    #[export]
    head: Option<Gd<PlayerHead>>,
    #[export]
    #[init(default = 8.0)]
    speed: real,
    #[export]
    #[init(default = -30.0)]
    gravity: real,
    #[base]
    base: Base<CharacterBody3D>,
}
#[godot_api]
impl ICharacterBody3D for PlayerBody {
    fn ready(&mut self) {
        Input::singleton().set_mouse_mode(MouseMode::CAPTURED);
    }

    fn unhandled_input(&mut self, event: Gd<InputEvent>) {
        if let Ok(event_motion) = event.try_cast::<InputEventMouseMotion>() {
            let mut head = self.get_head().expect("Head must be initialised");
            let mut camera = head
                .bind_mut()
                .get_camera()
                .expect("Camera must be initialised");
            head.rotate_y(-event_motion.get_relative().x * self.sensitivity);
            camera.rotate_x(-event_motion.get_relative().y * self.sensitivity);
            camera.get_rotation().x = clampf(
                camera.get_rotation().x as f64,
                deg_to_rad(-40.0),
                deg_to_rad(60.0),
            ) as f32;
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let mut player_velocity = Vector3::ZERO;
        if !self.base_mut().is_on_floor() {
            player_velocity.y += self.get_gravity() * (delta as f32);
        }
        let input_dir = Input::singleton().get_vector(
            "strafe_left".into(),
            "strafe_right".into(),
            "move_forward".into(),
            "move_back".into(),
        );
        let mut head = self.get_head().expect("Head must be initialised");
        let direction = head.bind_mut().get_head_transform_basis()
            * Vector3 {
                x: input_dir.x,
                y: 0.0,
                z: input_dir.y,
            };
        player_velocity.x = direction.x * self.get_speed();
        player_velocity.z = direction.z * self.get_speed();
        self.base_mut().set_velocity(player_velocity);
        self.base_mut().move_and_slide();
    }
}
