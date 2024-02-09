use godot::{engine::Object, obj::Gd};

use crate::{health::Health, prod_grunt::ProdGrunt};

pub trait Damageable: Health {
    fn damage(&mut self, damage: i32) {
        self.get_health_component().bind_mut().take_damage(damage);
    }
}
pub enum Damageables {
    PROD(Gd<ProdGrunt>),
    None,
}

impl Damageables {
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    pub fn is_some(&self) -> bool {
        !matches!(self, Self::None)
    }

    pub fn do_damage(&mut self, value: i32) {
        match self {
            Damageables::PROD(d) => d.bind_mut().damage(value),
            Damageables::None => {}
        }
    }
    pub fn try_cast_damageable(node: Gd<Object>) -> Damageables {
        if let Ok(a) = node.try_cast::<ProdGrunt>() {
            return Damageables::PROD(a);
        }
        Damageables::None
    }
}
