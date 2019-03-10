use amethyst::{
    ecs::{Join, ReadStorage, System, Write},
    renderer::{DebugLines, Rgba},
};

use crate::components::*;

pub struct DrawAABBSystem;
impl<'s> System<'s> for DrawAABBSystem {
    type SystemData = (
        Write<'s, DebugLines>,
        ReadStorage<'s, AABB>,
    );

    fn run(&mut self, (mut debug_lines_resource, aabbs): Self::SystemData) {
        for aabb in (&aabbs).join() {
            debug_lines_resource.draw_line(
                [aabb.min.x, aabb.max.y, 0.0].into(),
                [aabb.max.x, aabb.max.y, 0.0].into(),
                Rgba::green(),
            );
            debug_lines_resource.draw_line(
                [aabb.max.x, aabb.max.y, 0.0].into(),
                [aabb.max.x, aabb.min.y, 0.0].into(),
                Rgba::green(),
            );
            debug_lines_resource.draw_line(
                [aabb.max.x, aabb.min.y, 0.0].into(),
                [aabb.min.x, aabb.min.y, 0.0].into(),
                Rgba::green(),
            );
            debug_lines_resource.draw_line(
                [aabb.min.x, aabb.min.y, 0.0].into(),
                [aabb.min.x, aabb.max.y, 0.0].into(),
                Rgba::green(),
            );
        }
    }
}
