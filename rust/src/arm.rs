use godot::engine::Node3D;
use godot::prelude::*;

use crate::gun::Gun;

#[derive(GodotClass)]
#[class(init, base = Node3D)]
pub struct Arm {
    base: Base<Node3D>,
    #[export]
    gun: Option<Gd<Gun>>,
}
