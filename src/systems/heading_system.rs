use amethyst::{
    core::{math::Point2, GlobalTransform},
    ecs::{Join, Read, ReadExpect, ReadStorage, System, WriteStorage},
    input::InputHandler,
    renderer::{Camera, ScreenDimensions},
};

use crate::components::*;

pub struct HeadingSystem;
impl<'s> System<'s> for HeadingSystem {
    type SystemData = (
        ReadStorage<'s, PlayerCamera>,
        ReadStorage<'s, Camera>,
        ReadStorage<'s, GlobalTransform>,
        ReadExpect<'s, ScreenDimensions>,
        ReadStorage<'s, Player>,
        WriteStorage<'s, Heading>,
        Read<'s, InputHandler<String, String>>,
    );

    fn run(
        &mut self,
        (player_cameras, cameras, transforms, screen_dimensions, players, mut headings, input): Self::SystemData,
    ) {
        if let Some((x, y)) = input.mouse_position() {
            let mouse_pos = Point2::new(x as f32, y as f32);
            for (camera, transform, _) in (&cameras, &transforms, &player_cameras).join() {
                let world_position =
                    camera.position_from_screen(mouse_pos, transform, &screen_dimensions);
                for (heading, _) in (&mut headings, &players).join() {
                    heading.x = world_position.x;
                    heading.y = world_position.y;
                }
            }
        }
    }
}
