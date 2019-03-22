use amethyst::ecs::{Component, NullStorage};
use serde::*;

#[derive(Default, Serialize, Deserialize)]
pub struct Player;
impl Component for Player {
    type Storage = NullStorage<Self>;
}
