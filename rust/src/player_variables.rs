use godot::engine::Node3D;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(tool, init, base=Node3D)]
pub struct PlayerVariables {
    #[base]
    base: Base<Node3D>,
    #[export]
    #[var(get, set)]
    #[init(default = -30.0)]
    gravity: real,
    #[export]
    #[var(get, set)]
    #[init(default = 8.0)]
    max_speed: real,
    #[export]
    #[var(get, set)]
    #[init(default = 0.002)]
    mouse_sensitivity: real,
}
