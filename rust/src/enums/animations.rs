use godot::prelude::*;
pub enum Animations {
    DEFAULT,
    DEATH,
    ATTACK,
    HIT,
    EXPLODE,
}

impl Animations {
    pub fn as_string_name(&self) -> StringName {
        match self {
            Animations::DEFAULT => StringName::from("default"),
            Animations::DEATH => StringName::from("death"),
            Animations::EXPLODE => StringName::from("explode"),
            Animations::ATTACK => StringName::from("attack"),
            Animations::HIT => StringName::from("hit"),
        }
    }
}
