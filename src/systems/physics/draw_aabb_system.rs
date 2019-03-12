use amethyst::{
    ecs::{Entities, Join, ReadStorage, System, Write},
    renderer::{DebugLines, Rgba},
};

use crate::components::*;
use crate::systems::physics::physics::ENTITIES_COLLIDING;

pub struct DrawAABBSystem;
impl<'s> System<'s> for DrawAABBSystem {
    type SystemData = (
        Write<'s, DebugLines>,
        Entities<'s>,
        ReadStorage<'s, AABB>,
    );

    fn run(&mut self, (mut debug_lines_resource, entities, aabbs): Self::SystemData) {
        for (entity, aabb) in (&entities, &aabbs).join() {
            let color = {
                if ENTITIES_COLLIDING.lock().unwrap().contains(&entity.id()) {
                    Rgba::red()
                }
                else {
                    Rgba::green()
                }
            };
            debug_lines_resource.draw_line(
                [aabb.min.x, aabb.max.y, 0.0].into(),
                [aabb.max.x, aabb.max.y, 0.0].into(),
                color,
            );
            debug_lines_resource.draw_line(
                [aabb.max.x, aabb.max.y, 0.0].into(),
                [aabb.max.x, aabb.min.y, 0.0].into(),
                color,
            );
            debug_lines_resource.draw_line(
                [aabb.max.x, aabb.min.y, 0.0].into(),
                [aabb.min.x, aabb.min.y, 0.0].into(),
                color,
            );
            debug_lines_resource.draw_line(
                [aabb.min.x, aabb.min.y, 0.0].into(),
                [aabb.min.x, aabb.max.y, 0.0].into(),
                color,
            );
        }
    }
}
