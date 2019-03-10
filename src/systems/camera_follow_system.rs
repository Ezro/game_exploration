use amethyst::{
    core::Transform,
    ecs::{Join, ReadStorage, System, WriteStorage},
};

use crate::components::*;

pub struct CameraFollowSystem;
impl<'s> System<'s> for CameraFollowSystem {
    type SystemData = (
        ReadStorage<'s, Player>,
        ReadStorage<'s, PlayerCamera>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (players, cameras, mut transforms): Self::SystemData) {
        let player_transform = {
            let mut player_transforms = (&transforms, &players).join();
            let player_transform = player_transforms
                .next()
                .map(|(transform, _)| transform.clone());
            if player_transforms.next().is_some() {
                // you probably want to do something to handle this case properly, but
                println!("WARN: More than one player entity!")
            }
            player_transform
        };

        if let Some(player_transform) = player_transform {
            for (camera_transform, _) in (&mut transforms, &cameras).join() {
                camera_transform.set_translation_x(player_transform.translation().x);
                camera_transform.set_translation_y(player_transform.translation().y);
            }
        }
    }
}
