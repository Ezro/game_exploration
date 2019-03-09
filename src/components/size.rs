use amethyst::ecs::{Component, VecStorage};

#[derive(Default)]
pub struct Size {
    pub x: f32,
    pub y: f32,
}
impl Component for Size {
    type Storage = VecStorage<Self>;
}
