use amethyst::{
    core::{Transform},
    ecs::{Join, ReadStorage, System, WriteStorage}
};

use crate::components::*;

pub struct AABBFollowSystem;
impl<'s> System<'s> for AABBFollowSystem {
    type SystemData = (WriteStorage<'s, AABB>, ReadStorage<'s, Transform>);

    fn run(&mut self, (mut aabbs, transforms): Self::SystemData) {
        for (aabb, t) in (&mut aabbs, &transforms).join() {
            aabb.x = t.translation().x;
            aabb.y = t.translation().y;
        }
    }
}