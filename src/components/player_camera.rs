use amethyst::ecs::{Component, NullStorage};
use serde::*;

#[derive(Default, Serialize, Deserialize)]
pub struct PlayerCamera;
impl Component for PlayerCamera {
    type Storage = NullStorage<Self>;
}
