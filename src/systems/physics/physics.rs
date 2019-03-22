use std::sync::Mutex;
lazy_static! {
    pub static ref ENTITIES_COLLIDING: Mutex<Vec<u32>> = Mutex::new(vec![]);
}
lazy_static! {
    pub static ref COLLISION_MANIFOLDS: Mutex<Vec<Manifold>> = Mutex::new(vec![]);
}

use crate::components::*;

#[derive(Clone, Debug)]
pub struct Manifold {
    pub contacts: [AABB; 2],
    pub contact_count: u32,
}
