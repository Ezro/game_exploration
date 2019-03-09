use amethyst::ecs::{Component, VecStorage};

#[derive(Default)]
pub struct AABB {
    pub x: f32,
    pub y: f32,
}

impl Component for AABB {
    type Storage = VecStorage<Self>;
}
