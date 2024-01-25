use godot::engine::{EditorPlugin, Engine, IEditorPlugin};
use godot::prelude::*;

use crate::player_variables::PlayerVariables;

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
        Engine::singleton().register_singleton(
            GlobalState::PLAYER_VARS.into(),
            PlayerVariables::new_alloc().upcast::<Object>(),
        )
    }
    fn exit_tree(&mut self) {
        Engine::singleton().unregister_singleton(GlobalState::PLAYER_VARS.into())
    }
}
