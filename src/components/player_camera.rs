use amethyst::ecs::{Component, NullStorage};

#[derive(Default)]
pub struct PlayerCamera;
impl Component for PlayerCamera {
    type Storage = NullStorage<Self>;
}
