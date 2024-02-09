use godot::engine::{IOmniLight3D, OmniLight3D, Timer};
use godot::obj::WithBaseField;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base = OmniLight3D)]
pub struct MuzzleFlash {
    base: Base<OmniLight3D>,
    #[export]
    timer: Option<Gd<Timer>>,
}
#[godot_api]
impl IOmniLight3D for MuzzleFlash {
    fn ready(&mut self) {
        if let Some(mut timer) = self.get_timer() {
            timer.connect(
                "timeout".into(),
                Callable::from_object_method(
                    &self.base().to_godot(),
                    StringName::from("on_timeout"),
                ),
            );
        }
    }
}
#[godot_api]
impl MuzzleFlash {
    #[func]
    fn on_timeout(&mut self) {
        self.base_mut().queue_free();
    }
}
