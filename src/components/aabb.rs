use amethyst::{
    core::math::Vector2,
    ecs::{Component, VecStorage},
};

pub struct AABB {
    pub min: Vector2<f32>,
    pub max: Vector2<f32>,
}

impl Component for AABB {
    type Storage = VecStorage<Self>;
}
