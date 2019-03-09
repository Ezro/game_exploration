use amethyst::{
    ecs::{Join, Read, System, WriteStorage},
    input::InputHandler,
};

use crate::components::*;

pub struct HeadingSystem;
impl<'s> System<'s> for HeadingSystem {
    type SystemData = (
        WriteStorage<'s, Heading>,
        Read<'s, InputHandler<String, String>>,
    );

    fn run(&mut self, (mut headings, input): Self::SystemData) {
        if let Some((x, y)) = input.mouse_position() {
            for heading in (&mut headings).join() {
                heading.x = x as f32;
                heading.y = y as f32;
            }
        }
    }
}
