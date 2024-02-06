use ::godot::engine::CharacterBody3D;
use ::godot::prelude::*;
use godot::{
    engine::{AnimatedSprite3D, ICharacterBody3D, NavigationAgent3D},
    obj::WithBaseField,
};

use crate::player::Player;

#[derive(GodotClass)]
#[class(init, base = CharacterBody3D)]
pub struct ProdGrunt {
    base: Base<CharacterBody3D>,
    #[export]
    player: Option<Gd<Player>>,
    #[export]
    #[init(default = 3.0)]
    speed: real,
    #[export]
    sprite: Option<Gd<AnimatedSprite3D>>,
    #[export]
    nav_agent: Option<Gd<NavigationAgent3D>>,
}
// #[godot_api]
// impl ProdGrunt {
//     fn move_to_target(&mut self) {}
// }
#[godot_api]
impl ICharacterBody3D for ProdGrunt {
    fn ready(&mut self) {
        if let Some(mut sprite) = self.get_sprite() {
            sprite.set_animation("default".into());
        }
    }
    fn process(&mut self, _delta: f64) {
        self.base_mut().set_velocity(Vector3::ZERO);
        if let Some(_sprite) = self.get_sprite() {
            if let Some(player) = self.get_player() {
                if let Some(mut nav_agent) = self.get_nav_agent() {
                    nav_agent.set_target_position(player.get_global_position());
                    let next_nav_point = nav_agent.get_next_path_position();
                    let origin_self = self.base().get_global_transform().origin;
                    let speed = self.get_speed();
                    let y_position = self.base().get_global_position().y;
                    self.base_mut()
                        .set_velocity((next_nav_point - origin_self).normalized() * speed);
                    self.base_mut().look_at(Vector3 {
                        x: player.get_global_position().x,
                        y: y_position,
                        z: player.get_global_position().z,
                    });
                    dbg!(self.base().get_velocity());
                    self.base_mut().move_and_slide();
                }
            }
        }
    }
}
