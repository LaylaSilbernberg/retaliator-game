use std::collections::HashMap;
use std::usize;

use crate::arm::Arm;
use crate::gun::Gun;
use crate::player_variables::PlayerVariables;
use godot::engine::utilities::{clampf, deg_to_rad};
use godot::engine::{InputEvent, InputEventMouseMotion, Node3D, NodeExt};
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
    #[export]
    second_left_arm: Option<Gd<Arm>>,
    #[export]
    right_arm: Option<Gd<Arm>>,
    #[export]
    pivot: Option<Gd<Node3D>>,
    #[export]
    second_right_arm: Option<Gd<Arm>>,
    inventory: HashMap<usize, Vec<Gd<PackedScene>>>,
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
            let stats = gun.bind().get_stats();
            let Some(stats) = stats else {
                return;
            };
            if stats.bind().get_is_quadruple() || stats.bind().get_is_triple() {
                self.init_gun(gun, is_right, true);
            } else {
                self.init_gun(gun, is_right, false);
            }
        } else {
            self.init_if_guns_are_the_same(gun, is_right)
        }
    }
    fn init_if_guns_are_the_same(&mut self, gun: Gd<Gun>, is_right: bool) {
        let stats = gun.bind().get_stats();
        let Some(stats) = stats else {
            return;
        };
        if stats.bind().get_is_quadruple() {
            self.init_quadruple(gun, is_right)
        } else if stats.bind().get_is_triple() {
            self.init_triple(gun, is_right)
        } else if stats.bind().get_is_dual() {
            self.init_gun(gun, is_right, false)
        } else {
            let right_after_increment = self.get_right_current_gun() + 1;
            if is_right && right_after_increment <= self.inventory.len() {
                self.right_current_gun += 1;
                let index = self.right_current_gun;
                let Some(new_gun) = self.instantiate_gun(&index, is_right, false) else {
                    return;
                };
                self.init_gun(new_gun, is_right, false);
            }
            if !is_right && self.get_left_current_gun().checked_sub(2).is_some() {
                self.left_current_gun -= 1;
                let index = self.left_current_gun;
                let Some(new_gun) = self.instantiate_gun(&index, is_right, false) else {
                    return;
                };
                self.init_gun(new_gun, is_right, false);
            }
        }
    }

    fn init_quadruple(&mut self, gun: Gd<Gun>, is_right: bool) {
        self.init_gun(gun, is_right, true);
    }
    fn init_triple(&mut self, gun: Gd<Gun>, is_right: bool) {
        let arm = if !is_right {
            self.get_right_arm()
        } else {
            self.get_left_arm()
        };
        let second_arm = if !is_right {
            self.get_second_right_arm()
        } else {
            self.get_second_left_arm()
        };
        let Some(arm) = arm else {
            return;
        };
        let Some(second_arm) = second_arm else {
            return;
        };
        if arm.bind().get_gun().is_some() && second_arm.bind().get_gun().is_some() {
            self.init_gun(gun, is_right, false)
        } else {
            self.init_gun(gun, is_right, true)
        }
    }

    fn scroll_weapons_increment(&mut self, is_right: bool) {
        let inventory_size = self.inventory.len();
        if is_right && (self.right_current_gun + 1) <= inventory_size {
            self.right_current_gun += 1;
            let index = self.right_current_gun;
            let Some(gun) = self.instantiate_gun(&index, is_right, false) else {
                return;
            };
            self.init_if_guns_are_different(gun, is_right)
        }
        if !is_right && (self.get_left_current_gun() + 1) <= inventory_size {
            self.left_current_gun += 1;
            let index = self.left_current_gun;
            let Some(gun) = self.instantiate_gun(&index, is_right, false) else {
                return;
            };
            self.init_if_guns_are_different(gun, is_right)
        }
    }
    fn scroll_weapons_decrement(&mut self, is_right: bool) {
        if is_right && self.get_right_current_gun().checked_sub(2).is_some() {
            self.right_current_gun -= 1;
            let index = self.right_current_gun;
            let Some(gun) = self.instantiate_gun(&index, is_right, false) else {
                return;
            };
            self.init_if_guns_are_different(gun, is_right)
        }
        if !is_right && self.get_left_current_gun().checked_sub(2).is_some() {
            self.left_current_gun -= 1;
            let index = self.left_current_gun;
            let Some(gun) = self.instantiate_gun(&index, is_right, false) else {
                return;
            };
            self.init_if_guns_are_different(gun, is_right)
        }
    }
    fn instantiate_gun(
        &mut self,
        gun_index: &usize,
        is_right: bool,
        is_quadruple: bool,
    ) -> Option<Gd<Gun>> {
        let mut gun_orientation: usize = if is_right { 0 } else { 1 };
        if is_quadruple {
            gun_orientation += 2;
        }
        self.inventory
            .get(gun_index)
            .expect("No array here")
            .get(gun_orientation)
            .expect("Vec is empty")
            .try_instantiate_as::<Gun>()
    }

    pub fn get_head_transform_basis(&mut self) -> Basis {
        self.base_mut().get_transform().basis
    }
    fn init_gun(&mut self, gun: Gd<Gun>, is_right: bool, has_extra_arm: bool) {
        let name = gun.get_name();
        if let Some(mut upper_arm) = if is_right {
            self.get_second_right_arm()
        } else {
            self.get_second_left_arm()
        } {
            let possible_gun = upper_arm.bind().get_gun();
            let Some(mut possible_second_gun) = possible_gun else {
                return;
            };
            possible_second_gun.queue_free();
            upper_arm.bind_mut().set_gun(None)
        }
        if let Some(mut arm) = if is_right {
            self.get_right_arm()
        } else {
            self.get_left_arm()
        } {
            let old_gun = arm.bind().get_gun();
            let Some(mut old_gun) = old_gun else {
                return;
            };
            old_gun.queue_free();
            arm.bind_mut().set_gun(None);
            arm.add_child(gun.upcast::<Node>());
            let Some(gun) = arm.try_get_node_as::<Gun>(NodePath::from(name)) else {
                return;
            };
            arm.bind_mut().set_gun(Some(gun));

            if has_extra_arm {
                if let Some(mut second_arm) = if is_right {
                    self.get_second_right_arm()
                } else {
                    self.get_second_left_arm()
                } {
                    let actual_index = if is_right {
                        self.right_current_gun
                    } else {
                        self.left_current_gun
                    };
                    let Some(second_gun) = self.instantiate_gun(&actual_index, is_right, true)
                    else {
                        return;
                    };
                    let name = second_gun.get_name();
                    second_arm.add_child(second_gun.upcast::<Node>());
                    let Some(second_gun) = second_arm.try_get_node_as::<Gun>(NodePath::from(name))
                    else {
                        return;
                    };

                    second_arm.bind_mut().set_gun(Some(second_gun));
                }
            }
        }
    }
    fn init_arms(&mut self) {
        if self.inventory.len() > 1 {
            let Some(right_gun) = self.instantiate_gun(&1, true, false) else {
                return;
            };
            self.init_gun(right_gun, true, false);

            let Some(left_gun) = self.instantiate_gun(&2, false, false) else {
                return;
            };
            self.init_gun(left_gun, false, false);
        }
    }
    fn instatiate_gun_by_index(&mut self, index: &usize, is_right: bool) {
        let checked_number = if is_right {
            self.right_current_gun
        } else {
            self.left_current_gun
        };
        if self.inventory.get(index).is_some() && index != &checked_number {
            if is_right {
                self.right_current_gun = *index;
            } else {
                self.left_current_gun = *index;
            }
            let Some(gun) = self.instantiate_gun(index, is_right, false) else {
                return;
            };
            self.init_if_guns_are_different(gun, is_right)
        }
    }
}
#[godot_api]
impl INode3D for PlayerHead {
    fn init(base: Base<Node3D>) -> Self {
        let pistol: Vec<Gd<PackedScene>> = vec![
            load::<PackedScene>("res://scenes/weapons/pistol/right_pistol.tscn"),
            load::<PackedScene>("res://scenes/weapons/pistol/left_pistol.tscn"),
            load::<PackedScene>("res://scenes/weapons/pistol/top_right_pistol.tscn"),
            load::<PackedScene>("res://scenes/weapons/pistol/top_left_pistol.tscn"),
        ];
        let shotgun: Vec<Gd<PackedScene>> = vec![
            load::<PackedScene>("res://scenes/weapons/shotgun/right_shotgun.tscn"),
            load::<PackedScene>("res://scenes/weapons/shotgun/left_shotgun.tscn"),
        ];
        let mut packed_scene_map: HashMap<usize, Vec<Gd<PackedScene>>> = HashMap::new();
        packed_scene_map.insert(1, pistol);
        packed_scene_map.insert(2, shotgun);
        PlayerHead {
            camera: Option::None,
            base,
            left_arm: Option::None,
            second_left_arm: Option::None,
            right_arm: Option::None,
            pivot: Option::None,
            second_right_arm: Option::None,
            inventory: packed_scene_map,
            left_current_gun: 2,
            right_current_gun: 1,
        }
    }
    fn process(&mut self, _delta: f64) {
        let Some(right_arm) = self.get_right_arm() else {
            return;
        };
        let Some(mut gun) = right_arm.bind().get_gun() else {
            return;
        };
        if Input::singleton().is_action_just_pressed("shoot_right".into()) {
            gun.bind_mut().shoot();
        }
        let Some(left_arm) = self.get_left_arm() else {
            return;
        };
        let Some(mut gun) = left_arm.bind().get_gun() else {
            return;
        };
        if Input::singleton().is_action_just_pressed("shoot_left".into()) {
            gun.bind_mut().shoot();
        }

        if let Some(second_right_arm) = self.get_second_right_arm() {
            if let Some(mut gun) = second_right_arm.bind().get_gun() {
                if Input::singleton().is_action_just_released("shoot_right".into()) {
                    gun.bind_mut().shoot();
                }
            }
        }
        let Some(second_left_arm) = self.get_second_left_arm() else {
            return;
        };
        let Some(mut gun) = second_left_arm.bind().get_gun() else {
            return;
        };
        if Input::singleton().is_action_just_released("shoot_left".into()) {
            gun.bind_mut().shoot();
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
    fn unhandled_input(&mut self, input_event: Gd<InputEvent>) {
        let Some(mut pivot) = self.get_pivot() else {
            return;
        };
        let sensitivity = self
            .base()
            .get_node_as::<PlayerVariables>("/root/PlayerVars")
            .bind()
            .get_mouse_sensitivity();
        let Ok(event_motion) = input_event.try_cast::<InputEventMouseMotion>() else {
            return;
        };
        pivot.rotate_x(-event_motion.get_relative().y * sensitivity);
        pivot.get_rotation().x = clampf(
            pivot.get_rotation().x as f64,
            deg_to_rad(-40.0),
            deg_to_rad(60.0),
        ) as f32;
    }
    fn input(&mut self, input_event: Gd<InputEvent>) {
        if input_event.is_action_pressed("right_weapon_1".into()) {
            self.instatiate_gun_by_index(&1, true)
        }
        if input_event.is_action_pressed("right_weapon_2".into()) {
            self.instatiate_gun_by_index(&2, true)
        }
        if input_event.is_action_pressed("right_weapon_3".into()) {
            self.instatiate_gun_by_index(&3, true)
        }
        if input_event.is_action_pressed("left_weapon_1".into()) {
            self.instatiate_gun_by_index(&1, false)
        }
        if input_event.is_action_pressed("left_weapon_2".into()) {
            self.instatiate_gun_by_index(&2, false)
        }
        if input_event.is_action_pressed("left_weapon_3".into()) {
            self.instatiate_gun_by_index(&3, false)
        }
    }
}
