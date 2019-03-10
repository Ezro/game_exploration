use amethyst::{
    ecs::{Join, ReadStorage, System, Write},
    renderer::{DebugLines, Rgba},
};

use crate::components::*;

pub struct DrawHeadingSystem;
impl<'s> System<'s> for DrawHeadingSystem {
    type SystemData = (
        Write<'s, DebugLines>,
        ReadStorage<'s, Position>,
        ReadStorage<'s, Heading>,
    );

    fn run(
        &mut self,
        (mut debug_lines_resource, positions, headings): Self::SystemData) {
            for (position, heading) in (&positions, &headings).join() {
                // println!("position x: {}, position y: {}", position.x, position.y);
                debug_lines_resource.draw_line(
                    [position.x, position.y, 0.0].into(),
                    [heading.x, heading.y, 0.0].into(),
                    Rgba::red(),
                );
            }
    }
}
