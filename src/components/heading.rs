use amethyst::ecs::{Component, VecStorage};
use serde::*;

#[derive(Default, Serialize, Deserialize)]
pub struct Heading {
    pub x: f32,
    pub y: f32,
}
impl Component for Heading {
    type Storage = VecStorage<Self>;
}
