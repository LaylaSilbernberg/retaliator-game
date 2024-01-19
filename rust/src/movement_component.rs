use godot::engine::Resource;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Resource)]
pub struct MovementComponent {
    #[export]
    speed: f64,
    #[base]
    base: Base<Resource>,
}
impl MovementComponent {
    const GLOBAL: Node3D = get_node("/root/PlayerVariables");
    let vector: Vector3 = Vector3::new(0.0, 0.0, 0.0);

    fn get_input(){

    }
}
