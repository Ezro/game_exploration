use amethyst::{
    core::{math::{Vector, Vector2}, Transform},
    ecs::{Entity, Entities, Join, ReadStorage, System, Write, WriteStorage},
    renderer::{DebugLines, Rgba},
};
use crate::components::*;
use crate::systems::physics::physics::COLLISION_MANIFOLDS;

pub struct GenerateConstraintsSystem;
impl<'s> System<'s> for GenerateConstraintsSystem {
    type SystemData = (
        ReadStorage<'s, AABB>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Heading>,
    );

    fn run(&mut self, _: Self::SystemData) {
    }
}