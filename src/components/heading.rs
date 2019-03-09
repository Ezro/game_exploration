use amethyst::ecs::{Component, VecStorage};

#[derive(Default)]
pub struct Heading {
    pub x: f32,
    pub y: f32,
}
impl Component for Heading {
    type Storage = VecStorage<Self>;
}
