use godot::engine::Node3D;
use godot::obj::WithBaseField;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base = Node3D)]
pub struct PlayerHead {
    #[var]
    camera: Option<Gd<Camera3D>>,
    #[base]
    base: Base<Node3D>,
}
#[godot_api]
impl PlayerHead {
    #[func]
    pub fn get_head_transform(&mut self) -> Basis {
        self.base_mut().get_transform().basis
    }
}
