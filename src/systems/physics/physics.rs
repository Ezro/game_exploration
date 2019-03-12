use std::sync::Mutex;
lazy_static! {
    pub static ref ENTITIES_COLLIDING: Mutex<Vec<u32>> = Mutex::new(vec![]);
}
lazy_static! {
    pub static ref COLLISION_MANIFOLDS: Mutex<Vec<Manifold>> = Mutex::new(vec![]);
}

use amethyst::{
    core::{math::{Vector, Vector2}, Transform},
    ecs::{Entity, Entities, Join, ReadStorage, System, Write, WriteStorage},
    renderer::{DebugLines, Rgba},
};
use crate::components::*;

#[derive(Debug)]
pub struct Contact {
    pub position: Vector2<f32>,
    pub normal: Vector2<f32>,
    pub penetration: f32
}

#[derive(Debug)]
pub struct Manifold {
    pub contacts: [Contact; 2],
    pub contact_count: u32,
}
// pub struct PhysicsSystem;
// impl<'s> System<'s> for PhysicsSystem {
//     type SystemData = (
//         Entities<'s>,
//         ReadStorage<'s, AABB>,
//         WriteStorage<'s, Transform>,
//     );

//     fn run(&mut self, (entities, raabbs, mut transforms): Self::SystemData) {
//         for (ae, a) in (&entities, &raabbs).join() {
//             for (be, b) in (&entities, &raabbs).join().next() {
//                 let ae_id = ae.id();
//                 let be_id = be.id();
//                 if ae_id != be_id {
//                     let overlap = aabbs_overlap(a.clone(), b.clone());
//                     if overlap {
//                         if !ENTITIES_COLLIDING.lock().unwrap().contains(&ae_id) {
//                             ENTITIES_COLLIDING.lock().unwrap().push(ae_id);

//                         }
//                         if !ENTITIES_COLLIDING.lock().unwrap().contains(&be_id) {
//                             ENTITIES_COLLIDING.lock().unwrap().push(be_id);
//                         }
//                     }
//                     else {
//                         if ENTITIES_COLLIDING.lock().unwrap().contains(&ae_id) {
//                             let index = ENTITIES_COLLIDING.lock().unwrap().iter().position(|e| *e == ae_id).unwrap();
//                             ENTITIES_COLLIDING.lock().unwrap().remove(index);
//                         }
//                         if ENTITIES_COLLIDING.lock().unwrap().contains(&be_id) {
//                             let index = ENTITIES_COLLIDING.lock().unwrap().iter().position(|e| *e == be_id).unwrap();
//                             ENTITIES_COLLIDING.lock().unwrap().remove(index);
//                         }
//                     }
//                 }
//             }
//         }
//     }
// }

// fn aabbs_overlap(a: AABB, b: AABB) -> bool {
//     if a.min.x > b.max.x
//     || a.min.y > b.max.y
//     || a.max.x < b.min.x
//     || a.max.y < b.min.y {
//         return false
//     }
//     else {
//         true
//     }
// }

// pub struct Contact {
//     position: Vector2<f32>,
//     normal: Vector2<f32>,
//     penetration: f32
// }

// pub struct Manifold {
//     contacts: [Contact; 2],
//     contact_count: u32,
// }

// fn abs(v: f32) -> f32 {
//     if v < 0.0 {
//         v * -1.0
//     }
//     else {
//         v
//     }
// }

// fn add_collision_for_entity(e_id: u32) {

// }

// fn generate_collision_manifold(a: AABB, b: AABB) -> Manifold {
//     let c1: Contact = {
//         let x_translation = (b.max.x - ((b.max.x - b.min.x) / 2.0)) - (a.max.x - ((a.max.x - a.min.x) / 2.0));
//         Contact {
//             position: { Vector2::new(a.max.x - b.min.x, a.max.y - b.min.y) },
//                     normal: {
//                         if x_translation > 0.0 {
//                             Vector2::new(1.0, 0.0)
//                         }
//                         else {
//                             Vector2::new(-1.0, 0.0)
//                         }
//                     },
//                     penetration: a.max.x - b.min.x,
//                 }
//         };
//         let c2: Contact = {
//                 let y_translation = (b.max.y - ((b.max.y - b.min.y) / 2.0)) - (a.max.y - ((a.max.y - a.min.y) / 2.0));
//                 Contact {
//                     position: { Vector2::new(a.max.x - b.min.x, a.max.y - b.min.y) },
//                     normal: {
//                         if y_translation > 0.0 {
//                             Vector2::new(0.0, 1.0)
//                         }
//                         else {
//                             Vector2::new(0.0, -1.0)
//                         }
//                     },
//                     penetration: a.max.y - b.min.y,
//                 }
//         };
//         Manifold {
//             contacts: [c1, c2],
//             contact_count: 2
//         }
// }