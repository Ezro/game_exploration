use amethyst::{
    core::math::Vector2,
    ecs::{Join, ReadStorage, System, Write},
    renderer::{DebugLines, Rgba},
};

use crate::components::*;

pub struct PhysicsSystem;
impl<'s> System<'s> for PhysicsSystem {
    type SystemData = (
        ReadStorage<'s, AABB>,
        ReadStorage<'s, Size>,
    );

    fn run(&mut self, (aabbs, sizes): Self::SystemData) {
        let mut previous_aabb: AABB;
        let mut is_first_iter = true;
        for (aabb, _) in (&aabbs, &sizes).join() {
            if (is_first_iter) {
                is_first_iter = false;
            }
            else {
                // let a_x_extent = (previous_aabb.max.x - previous_aabb.min.x) / 2.0;
                // let b_x_extent = (aabb.max.x - aabb.min.x) / 2.0;
                // let x_translation = (aabb.max.x - aabb.min.x) - (previous_aabb.max.x - previous_aabb.min.x);
                // let x_overlap = a_x_extent + b_x_extent - abs(x_translation);

                // let a_y_extent = (previous_aabb.max.y - previous_aabb.min.y) / 2.0;
                // let b_y_extent = (aabb.max.y - aabb.min.y) / 2.0;
                // let y_translation = (aabb.max.y - aabb.min.y) - (previous_aabb.max.y - previous_aabb.min.y);
                // let y_overlap = a_y_extent + b_y_extent - abs(y_translation);

                // if x_overlap > 0.0 || y_overlap > 0.0 {
                //     println!("{:?}, {:?}", x_overlap, y_overlap);
                // }
            }
            previous_aabb = aabb.clone();
        }
    }
}

struct Contact {
    position: Vector2<f32>,
    normal: Vector2<f32>,
    penetration: f32
}

struct Manifold {
    contacts: [Contact; 2],
    contact_count: u32,
}

fn abs(v: f32) -> f32 {
    if v < 0.0 {
        v * -1.0
    }
    else {
        v
    }
}