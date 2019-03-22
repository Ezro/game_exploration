use amethyst::ecs::{Component, VecStorage};
use serde::*;

#[derive(Default, Serialize, Deserialize)]
pub struct Speed {
    pub speed: f32,
}
impl Component for Speed {
    type Storage = VecStorage<Self>;
}
