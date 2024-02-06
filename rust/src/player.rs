use ::godot::prelude::*;

use godot::engine::input::MouseMode;
use godot::engine::utilities::{clampf, deg_to_rad};
use godot::engine::{CharacterBody3D, ICharacterBody3D, InputEvent, InputEventMouseMotion};
use godot::obj::WithBaseField;

use crate::health_component::HealthComponent;
use crate::player_head::PlayerHead;
use crate::player_variables::PlayerVariables;

#[derive(GodotClass)]
#[class(init, base = CharacterBody3D)]
pub struct Player {
    #[export]
    head: Option<Gd<PlayerHead>>,
    #[export]
    health_component: Option<Gd<HealthComponent>>,
    base: Base<CharacterBody3D>,
}
#[godot_api]
impl ICharacterBody3D for Player {
    fn ready(&mut self) {
        Input::singleton().set_mouse_mode(MouseMode::CAPTURED);
    }

    fn unhandled_input(&mut self, event: Gd<InputEvent>) {
        let sensitivity = self
            .base()
            .get_node_as::<PlayerVariables>("/root/PlayerVars")
            .bind()
            .get_mouse_sensitivity();
        if let Ok(event_motion) = event.try_cast::<InputEventMouseMotion>() {
            let mut head = self.get_head().expect("Head must be initialised");
            let mut camera = head
                .bind_mut()
                .get_camera()
                .expect("Camera must be initialised");
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
        let mut player_velocity = Vector3::ZERO;

        let player_vars = self
            .base()
            .get_node_as::<PlayerVariables>("/root/PlayerVars");
        let gravity = player_vars.bind().get_gravity();
        let speed = player_vars.bind().get_max_speed();
        if !self.base_mut().is_on_floor() {
            player_velocity.y += gravity * (delta as f32);
        }
        let input_dir = Input::singleton().get_vector(
            "strafe_left".into(),
            "strafe_right".into(),
            "move_forward".into(),
            "move_back".into(),
        );
        if let Some(mut head) = self.get_head() {
            let direction = head.bind_mut().get_head_transform_basis()
                * Vector3 {
                    x: input_dir.x,
                    y: 0.0,
                    z: input_dir.y,
                };
            player_velocity.x = direction.x * speed;
            player_velocity.z = direction.z * speed;
            self.base_mut().set_velocity(player_velocity);
            self.base_mut().move_and_slide();
        }
    }
}
