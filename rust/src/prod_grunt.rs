use ::godot::engine::CharacterBody3D;
use ::godot::prelude::*;
use godot::{
    engine::{AnimatedSprite3D, CollisionShape3D, ICharacterBody3D, NavigationAgent3D},
    obj::WithBaseField,
};

use crate::{
    damageable::Damageable, health::Health, health_component::HealthComponent, player::Player,
};

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
    #[export]
    health: Option<Gd<HealthComponent>>,
    #[export]
    collision: Array<Gd<CollisionShape3D>>,
}
#[godot_api]
impl ProdGrunt {
    fn move_to_target(&mut self) {
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
                    self.base_mut().move_and_slide();
                }
            }
        }
    }
    fn death(&mut self) {
        self.base_mut().set_process(false);
        self.base_mut().set_physics_process(false);
        for mut shape in self.get_collision().iter_shared() {
            shape.set_disabled(true);
        }
        if let Some(health) = self.get_health() {
            if let Some(death_string) = health.bind().death() {
                if let Some(mut sprite) = self.get_sprite() {
                    sprite.set_animation(death_string);
                    sprite.play();
                }
            }
        }
    }
    fn set_animation_to_default(&mut self, mut sprite: Gd<AnimatedSprite3D>) {
        if let Some(animation) = sprite.get_sprite_frames() {
            if sprite.get_frame() == animation.get_frame_count("hit".into()) - 1 {
                sprite.set_animation("default".into());
                sprite.play();
            }
        }
    }
}
#[godot_api]
impl ICharacterBody3D for ProdGrunt {
    fn ready(&mut self) {
        if let Some(mut sprite) = self.get_sprite() {
            sprite.set_animation("default".into());
        }
    }
    fn process(&mut self, _delta: f64) {
        let default = StringName::from("default");
        let hit = StringName::from("hit");
        if let Some(sprite) = self.get_sprite() {
            if sprite.get_animation() == default {
                self.move_to_target();
            }
            if sprite.get_animation() == hit {
                self.set_animation_to_default(sprite);
            }
        }

        if let Some(health) = self.get_health() {
            if health.bind().get_health() <= 0 {
                self.death();
            }
        }
    }
}

impl Health for ProdGrunt {
    fn get_health_component(&self) -> Gd<HealthComponent> {
        self.get_health().unwrap()
    }
}
impl Damageable for ProdGrunt {
    fn damage(&mut self, damage: i32) {
        if let Some(mut sprite) = self.get_sprite() {
            self.get_health_component().bind_mut().take_damage(damage);
            sprite.set_animation("hit".into());
            sprite.play();
        }
    }
}
