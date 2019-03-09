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
        ReadStorage<'s, Size>,
    );

    fn run(&mut self, (mut debug_lines_resource, aabbs, sizes): Self::SystemData) {
        for (aabb, size) in (&aabbs, &sizes).join() {
            let x = size.x as f32 / 2.0;
            let y = size.y as f32 / 2.0;
            let t_x = aabb.x;
            let t_y = aabb.y;
            let top_left_x = t_x - x;
            let top_left_y = t_y + y;
            let top_right_x = t_x + x;
            let top_right_y = t_y + y;
            let bottom_right_x = t_x + x;
            let bottom_right_y = t_y - y;
            let bottom_left_x = t_x - x;
            let bottom_left_y = t_y - y;
            debug_lines_resource.draw_line(
                [top_left_x, top_left_y, 0.0].into(),
                [top_right_x, top_right_y, 0.0].into(),
                Rgba::green(),
            );
            debug_lines_resource.draw_line(
                [top_right_x, top_right_y, 0.0].into(),
                [bottom_right_x, bottom_right_y, 0.0].into(),
                Rgba::green(),
            );
            debug_lines_resource.draw_line(
                [bottom_right_x, bottom_right_y, 0.0].into(),
                [bottom_left_x, bottom_left_y, 0.0].into(),
                Rgba::green(),
            );
            debug_lines_resource.draw_line(
                [bottom_left_x, bottom_left_y, 0.0].into(),
                [top_left_x, top_left_y, 0.0].into(),
                Rgba::green(),
            );
        }
    }
}
