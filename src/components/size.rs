use amethyst::ecs::{Component, VecStorage};

#[derive(Default)]
pub struct Size {
    pub x: u32,
    pub y: u32,
}
impl Component for Size {
    type Storage = VecStorage<Self>;
}
