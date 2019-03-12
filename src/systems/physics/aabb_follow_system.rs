use amethyst::{
    core::Transform,
    ecs::{Join, ReadStorage, System, WriteStorage},
};

use crate::components::*;

pub struct AABBFollowSystem;
impl<'s> System<'s> for AABBFollowSystem {
    type SystemData = (WriteStorage<'s, AABB>, ReadStorage<'s, Position>, ReadStorage<'s, Size>);

    fn run(&mut self, (mut aabbs, positions, size): Self::SystemData) {
        for (aabb, position, size) in (&mut aabbs, &positions, &size).join() {
            let halfsize_x = size.x as f32 / 2.0;
            let halfsize_y = size.y as f32 / 2.0;
            aabb.min.x = position.x - halfsize_x;
            aabb.max.x = position.x + halfsize_x;
            aabb.min.y = position.y - halfsize_y;
            aabb.max.y = position.y + halfsize_y;
        }
    }
}
