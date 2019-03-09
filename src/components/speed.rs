use amethyst::ecs::{Component, VecStorage};

#[derive(Default)]
pub struct Speed {
    pub speed: f32,
}
impl Component for Speed {
    type Storage = VecStorage<Self>;
}
