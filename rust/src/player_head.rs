use std::usize;

use crate::arm::Arm;
use crate::gun::Gun;
use godot::engine::{Node3D, NodeExt};
use godot::obj::WithBaseField;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base = Node3D)]
pub struct PlayerHead {
    #[export]
    camera: Option<Gd<Camera3D>>,
    base: Base<Node3D>,
    #[export]
    left_arm: Option<Gd<Arm>>,
    #[var]
    second_left_arm: Option<Gd<Arm>>,
    #[export]
    right_arm: Option<Gd<Arm>>,
    #[export]
    second_right_arm: Option<Gd<Arm>>,
    #[export]
    inventory: Array<Array<Gd<PackedScene>>>,
    left_current_gun: usize,
    right_current_gun: usize,
}
#[godot_api]
impl PlayerHead {
    pub fn get_left_current_gun(&self) -> usize {
        self.left_current_gun
    }
    pub fn get_right_current_gun(&self) -> usize {
        self.right_current_gun
    }
    fn init_if_guns_are_different(&mut self, gun: Gd<Gun>, is_right: bool) {
        if self.get_right_current_gun() != self.get_left_current_gun() {
            self.init_gun(gun, is_right);
        } else {
            self.init_if_guns_are_the_same(gun, is_right)
        }
    }
    fn init_if_guns_are_the_same(&mut self, gun: Gd<Gun>, is_right: bool) {
        let stats = gun.bind().get_stats();
        if let Some(stats) = stats {
            if stats.bind().get_is_dual() {
                self.init_gun(gun, is_right)
            } else {
                let right_after_increment = self.get_right_current_gun() + 1;
                if is_right && right_after_increment < self.get_inventory().len() {
                    self.right_current_gun += 1;
                    if let Some(new_gun) =
                        self.instantiate_gun(self.get_right_current_gun(), is_right)
                    {
                        self.init_gun(new_gun, is_right);
                    }
                }
                if !is_right && self.get_left_current_gun().checked_sub(1).is_some() {
                    self.left_current_gun -= 1;
                    if let Some(new_gun) =
                        self.instantiate_gun(self.get_right_current_gun(), is_right)
                    {
                        self.init_gun(new_gun, is_right);
                    }
                }
            }
        }
    }

    fn scroll_weapons_increment(&mut self, is_right: bool) {
        let inventory_size = self.get_inventory().len();
        if is_right && (self.right_current_gun + 1) < inventory_size {
            self.right_current_gun += 1;
            if let Some(gun) = self.instantiate_gun(self.get_right_current_gun(), is_right) {
                self.init_if_guns_are_different(gun, is_right)
            }
        }
        if !is_right && (self.get_left_current_gun() + 1) < inventory_size {
            self.left_current_gun += 1;
            if let Some(gun) = self.instantiate_gun(self.get_left_current_gun(), is_right) {
                self.init_if_guns_are_different(gun, is_right)
            }
        }
    }
    fn scroll_weapons_decrement(&mut self, is_right: bool) {
        if is_right && self.get_right_current_gun().checked_sub(1).is_some() {
            self.right_current_gun -= 1;
            if let Some(gun) = self.instantiate_gun(self.get_right_current_gun(), is_right) {
                self.init_if_guns_are_different(gun, is_right)
            }
        }
        if !is_right && self.get_left_current_gun().checked_sub(1).is_some() {
            self.left_current_gun -= 1;
            if let Some(gun) = self.instantiate_gun(self.get_left_current_gun(), is_right) {
                self.init_if_guns_are_different(gun, is_right)
            }
        }
    }
    fn instantiate_gun(&mut self, gun_index: usize, is_right: bool) -> Option<Gd<Gun>> {
        let gun_orientation: usize = if is_right { 0 } else { 1 };
        self.get_inventory()
            .get(gun_index)
            .get(gun_orientation)
            .try_instantiate_as::<Gun>()
    }

    pub fn get_head_transform_basis(&mut self) -> Basis {
        self.base_mut().get_transform().basis
    }
    fn init_gun(&mut self, gun: Gd<Gun>, is_right: bool) {
        let name = gun.get_name();
        if let Some(mut arm) = if is_right {
            self.get_right_arm()
        } else {
            self.get_left_arm()
        } {
            let old_gun = arm.bind().get_gun();
            if let Some(old_gun) = old_gun {
                arm.remove_child(old_gun.upcast::<Node>())
            }
            arm.add_child(gun.upcast::<Node>());
            if let Some(gun) = arm.try_get_node_as::<Gun>(NodePath::from(name)) {
                arm.bind_mut().set_gun(Some(gun));
            }
        }
    }
    fn init_arms(&mut self) {
        if self.get_inventory().len() > 1 {
            if let Some(right_gun) = self.instantiate_gun(0, true) {
                self.init_gun(right_gun, true);
            }
            if let Some(left_gun) = self.instantiate_gun(1, false) {
                self.init_gun(left_gun, false);
            }
        }
    }
}
#[godot_api]
impl INode3D for PlayerHead {
    fn init(base: Base<Node3D>) -> Self {
        let mut pistol: Array<Gd<PackedScene>> = Array::new();
        pistol.push(load::<PackedScene>(
            "res://scenes/weapons/pistol/right_pistol.tscn",
        ));
        pistol.push(load::<PackedScene>(
            "res://scenes/weapons/pistol/left_pistol.tscn",
        ));
        let mut shotgun: Array<Gd<PackedScene>> = Array::new();
        shotgun.push(load::<PackedScene>(
            "res://scenes/weapons/shotgun/right_shotgun.tscn",
        ));
        shotgun.push(load::<PackedScene>(
            "res://scenes/weapons/shotgun/left_shotgun.tscn",
        ));
        let mut packed_scene_array = Array::new();
        packed_scene_array.push(pistol);
        packed_scene_array.push(shotgun);
        PlayerHead {
            camera: Option::None,
            base,
            left_arm: Option::None,
            second_left_arm: Option::None,
            right_arm: Option::None,
            second_right_arm: Option::None,
            inventory: packed_scene_array,
            left_current_gun: 1,
            right_current_gun: 0,
        }
    }
    fn process(&mut self, _delta: f64) {
        if let Some(right_arm) = self.get_right_arm() {
            if let Some(mut gun) = right_arm.bind().get_gun() {
                if Input::singleton().is_action_just_pressed("shoot_right".into()) {
                    gun.bind_mut().shoot();
                }
            }
        }
        if let Some(left_arm) = self.get_left_arm() {
            if let Some(mut gun) = left_arm.bind().get_gun() {
                if Input::singleton().is_action_just_pressed("shoot_left".into()) {
                    gun.bind_mut().shoot();
                }
            }
        }
        if Input::singleton().is_action_just_pressed("next_weapon_right".into()) {
            self.scroll_weapons_increment(true);
        }
        if Input::singleton().is_action_just_pressed("next_weapon_left".into()) {
            self.scroll_weapons_increment(false);
        }
        if Input::singleton().is_action_just_pressed("prev_weapon_right".into()) {
            self.scroll_weapons_decrement(true);
        }
        if Input::singleton().is_action_just_pressed("prev_weapon_left".into()) {
            self.scroll_weapons_decrement(false);
        }
    }
    fn ready(&mut self) {
        self.init_arms();
    }
}
