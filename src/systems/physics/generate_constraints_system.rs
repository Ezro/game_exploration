use crate::components::*;
use crate::systems::physics::physics::COLLISION_MANIFOLDS;
use amethyst::{
    core::{
        Transform,
    },
    ecs::{ReadStorage, System},
};

pub struct GenerateConstraintsSystem;
impl<'s> System<'s> for GenerateConstraintsSystem {
    type SystemData = (
        ReadStorage<'s, AABB>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Heading>,
    );

    fn run(&mut self, (aabbs, _, _): Self::SystemData) {
        while let Some(m) = COLLISION_MANIFOLDS.lock().unwrap().pop() {
            // println!("{:?}", m);
        }
    }
}
