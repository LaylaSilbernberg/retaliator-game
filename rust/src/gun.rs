use godot::engine::{AnimatedSprite2D, Node3D, RayCast3D};
use godot::obj::WithBaseField;
use godot::prelude::*;

use crate::damageable::Damageables;

#[derive(GodotClass)]
#[class(init, base = Node3D)]
pub struct Gun {
    #[var]
    #[init(default = 4)]
    last_frame: i32,
    #[var]
    #[init(default = true)]
    can_shoot: bool,
    #[export]
    damage: i32,
    #[export]
    gun_sprite: Option<Gd<AnimatedSprite2D>>,
    base: Base<Node3D>,
    #[export]
    gun_rays: Array<Gd<RayCast3D>>,
}

#[godot_api]
impl INode3D for Gun {
    fn process(&mut self, _delta: f64) {
        if let Some(mut gun_sprite) = self.get_gun_sprite() {
            if Input::singleton().is_action_just_pressed("shoot".into()) && self.get_can_shoot() {
                gun_sprite.set_animation("shoot".into());
                gun_sprite.play();
                self.flash();
                self.check_hit();
                self.can_shoot = false;
            }
            if !self.can_shoot && gun_sprite.get_frame() == self.get_last_frame() {
                self.finish_shooting();
            }
        }
    }
}
#[godot_api]
impl Gun {
    fn finish_shooting(&mut self) {
        if let Some(mut gun_sprite) = self.get_gun_sprite() {
            gun_sprite.set_animation("default".into());
            gun_sprite.play();
            self.can_shoot = true;
        }
    }

    fn check_hit(&mut self) {
        for ray in self.get_gun_rays().iter_shared() {
            if let Some(collider) = ray.get_collider() {
                let mut actor = Damageables::try_cast_damageable(collider);
                actor.do_damage(self.get_damage());
            }
        }
    }

    fn flash(&mut self) {
        if let Some(scene) =
            load::<PackedScene>("res://scenes/components/muzzle_flash.tscn").instantiate()
        {
            self.base_mut().add_child(scene);
        }
    }
}
