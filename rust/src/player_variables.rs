use godot::engine::Resource;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Resource)]
pub struct PlayerVariables {
    #[base]
    base: Base<Resource>,
    #[export]
    #[init(default = -30.0)]
    gravity: real,
    #[export]
    #[init(default = 8.0)]
    max_speed: real,
    #[export]
    #[init(default = 0.002)]
    mouse_sensitivity: real,
}
