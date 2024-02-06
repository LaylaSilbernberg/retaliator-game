use godot::engine::Node;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(tool, init, base=Node)]
pub struct PlayerVariables {
    base: Base<Node>,
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
