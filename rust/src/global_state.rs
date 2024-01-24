use godot::engine::{EditorPlugin, IEditorPlugin};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(tool, init, editor_plugin, base=EditorPlugin)]
pub struct GlobalState {
    #[base]
    base: Base<EditorPlugin>,
}

#[godot_api]
impl GlobalState {
    const PLAYER_VARS: &'static str = "PlayerVariables";
}

#[godot_api]
impl IEditorPlugin for GlobalState {
    fn enter_tree(&mut self) {
        self.base_mut().add_autoload_singleton(
            GlobalState::PLAYER_VARS.into(),
            "/root/player_variables".into(),
        )
    }
    fn exit_tree(&mut self) {
        self.base_mut()
            .remove_autoload_singleton(GlobalState::PLAYER_VARS.into())
    }
}
