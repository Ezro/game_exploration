use amethyst::ecs::{Component, VecStorage};
use serde::*;

#[derive(Default, Serialize, Deserialize)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}
impl Component for Position {
    type Storage = VecStorage<Self>;
}
