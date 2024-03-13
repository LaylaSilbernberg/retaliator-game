use core::f32;

use godot::engine::utilities::randf_range;
use godot::engine::{
    AnimatedSprite2D, Control, CpuParticles3D, Node3D, NodeExt, RayCast3D, ResourcePreloader,
};
use godot::obj::WithBaseField;
use godot::prelude::*;

use crate::damageable::Damageables;
use crate::weapon_statistics::WeaponStatistics;

#[derive(GodotClass)]
#[class(init, base = Node3D)]
pub struct Gun {
    #[var]
    #[init(default = true)]
    can_shoot: bool,
    #[export]
    stats: Option<Gd<WeaponStatistics>>,
    #[export]
    gun_sprite: Option<Gd<AnimatedSprite2D>>,
    base: Base<Node3D>,
    #[export]
    gun_rays: Array<Gd<RayCast3D>>,
    #[export]
    is_inaccurate: bool,
    #[export]
    control: Option<Gd<Control>>,
}

#[godot_api]
impl INode3D for Gun {
    fn ready(&mut self) {
        if self.get_is_inaccurate() {
            let mut rotation = Vector3::ZERO;
            for mut ray in self.get_gun_rays().iter_shared() {
                rotation.x = randf_range(-6.0, 6.0) as f32;
                rotation.y = randf_range(-6.0, 6.0) as f32;
                ray.set_rotation_degrees(rotation);
            }
        }
    }
    fn process(&mut self, _delta: f64) {
        if let Some(gun_sprite) = self.get_gun_sprite() {
            if let Some(sprite_frames) = gun_sprite.get_sprite_frames() {
                if !self.can_shoot
                    && gun_sprite.get_frame() == sprite_frames.get_frame_count("shoot".into()) - 1
                {
                    self.finish_shooting();
                }
            }
        }
    }
}
#[godot_api]
impl Gun {
    pub fn shoot(&mut self) {
        if let Some(mut gun_sprite) = self.get_gun_sprite() {
            if self.get_can_shoot() {
                gun_sprite.set_animation("shoot".into());
                gun_sprite.play();
                self.flash();
                self.check_hit();
                self.can_shoot = false;
            }
        }
    }
    fn finish_shooting(&mut self) {
        if let Some(mut gun_sprite) = self.get_gun_sprite() {
            gun_sprite.set_animation("default".into());
            gun_sprite.play();
            self.can_shoot = true;
        }
    }

    fn check_hit(&mut self) {
        let mut root_node = self
            .base()
            .get_tree()
            .expect("tree needs to exist")
            .get_current_scene()
            .expect("current scene must be initialised");
        for mut ray in self.get_gun_rays().iter_shared() {
            if let Some(collider) = ray.get_collider() {
                let mut actor = Damageables::try_cast_damageable(collider);
                if actor.is_some() {
                    if let Some(stats) = self.get_stats() {
                        actor.do_damage(stats.bind().get_damage());
                        let mut blood = load::<PackedScene>("res://scenes/particles/blood.tscn")
                            .instantiate_as::<CpuParticles3D>();
                        root_node.add_child(blood.upcast::<Node>());
                        blood = root_node.get_node_as::<CpuParticles3D>("Blood");
                        blood.set_global_position(ray.get_collision_point());
                    }
                }
            }
            if self.get_is_inaccurate() {
                let mut rotation = Vector3::ZERO;
                rotation.x = randf_range(-6.0, 6.0) as f32;
                rotation.y = randf_range(-6.0, 6.0) as f32;
                ray.set_rotation_degrees(rotation);
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
