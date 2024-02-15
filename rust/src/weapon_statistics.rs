use godot::engine::Resource;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base = Resource)]
pub struct WeaponStatistics {
    #[export]
    damage: i32,
    #[export]
    ammo_pool: i32,
    #[export]
    max_ammo: i32,
    #[export]
    is_dual: bool,
    #[export]
    is_triple: bool,
    #[export]
    is_quadruple: bool,
    base: Base<Resource>,
}
