use godot::engine::{EditorPlugin, IEditorPlugin};
use godot::obj::WithBaseField;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(tool, init, editor_plugin, base=EditorPlugin)]
pub struct GlobalState {
    #[base]
    base: Base<EditorPlugin>,
}

#[godot_api]
impl IEditorPlugin for GlobalState {
    fn enter_tree(&mut self) {
        Self::to_gd(self)
            .add_autoload_singleton("player_variables".into(), "./player_variables.rs".into())
    }
    fn exit_tree(&mut self) {
        Self::to_gd(self).remove_autoload_singleton("player_variables".into())
    }
}
