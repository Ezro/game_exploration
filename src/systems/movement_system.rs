use amethyst::{
    core::Transform,
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
    input::InputHandler,
};

use crate::components::*;

pub struct MovementSystem;
impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        ReadStorage<'s, Player>,
        WriteStorage<'s, Position>,
        ReadStorage<'s, Speed>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<String, String>>,
    );

    fn run(&mut self, (players, mut positions, speeds, mut transforms, input): Self::SystemData) {
        let x_move = input.axis_value("entity_x").unwrap();
        let y_move = input.axis_value("entity_y").unwrap();

        for (_, position, speed, transform) in
            (&players, &mut positions, &speeds, &mut transforms).join()
        {
            transform.translate_x(x_move as f32 * speed.speed);
            transform.translate_y(y_move as f32 * speed.speed);
            position.x = transform.translation().x;
            position.y = transform.translation().y;
        }
    }
}
