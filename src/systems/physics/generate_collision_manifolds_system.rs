use crate::components::*;
use crate::systems::physics::physics::Manifold;
use crate::systems::physics::physics::COLLISION_MANIFOLDS;
use crate::systems::physics::physics::ENTITIES_COLLIDING;
use amethyst::{
    core::{
        math::{Vector3},
        Transform,
    },
    ecs::{Entities, Join, ReadStorage, System, WriteStorage},
};

pub struct GenerateCollisionManifoldsSystem;
impl<'s> System<'s> for GenerateCollisionManifoldsSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, AABB>,
        WriteStorage<'s, Position>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (entities, raabbs, mut position, mut transforms): Self::SystemData) {
        for (ae, a, p, transform) in (&entities, &raabbs, &mut position, &mut transforms).join() {
            for (be, b) in (&entities, &raabbs).join().next() {
                let ae_id = ae.id();
                let be_id = be.id();
                if ae_id != be_id {
                    let overlap = aabbs_overlap(a.clone(), b.clone());
                    if overlap {
                        if !ENTITIES_COLLIDING.lock().unwrap().contains(&ae_id) {
                            ENTITIES_COLLIDING.lock().unwrap().push(ae_id);
                        }
                        if !ENTITIES_COLLIDING.lock().unwrap().contains(&be_id) {
                            ENTITIES_COLLIDING.lock().unwrap().push(be_id);
                        }
                        let m = Manifold {
                            contacts: [a.clone(), b.clone()],
                            contact_count: 2,
                        };
                        let t = create_translation_vector(m.clone());
                        transform.append_translation(-t);
                        COLLISION_MANIFOLDS.lock().unwrap().push(m);
                    } else {
                        if ENTITIES_COLLIDING.lock().unwrap().contains(&ae_id) {
                            let index = ENTITIES_COLLIDING
                                .lock()
                                .unwrap()
                                .iter()
                                .position(|e| *e == ae_id)
                                .unwrap();
                            ENTITIES_COLLIDING.lock().unwrap().remove(index);
                        }
                        if ENTITIES_COLLIDING.lock().unwrap().contains(&be_id) {
                            let index = ENTITIES_COLLIDING
                                .lock()
                                .unwrap()
                                .iter()
                                .position(|e| *e == be_id)
                                .unwrap();
                            ENTITIES_COLLIDING.lock().unwrap().remove(index);
                        }
                    }
                }
            }
                        p.x = transform.translation().x;
                        p.y = transform.translation().y;
        }
    }
}

fn aabbs_overlap(a: AABB, b: AABB) -> bool {
    if a.min.x > b.max.x || a.min.y > b.max.y || a.max.x < b.min.x || a.max.y < b.min.y {
        return false;
    } else {
        true
    }
}

fn create_translation_vector(m: Manifold) -> Vector3<f32> {
    let a_min_x = m.contacts[0].min.x;
    let a_max_x = m.contacts[0].max.x;
    let a_min_y = m.contacts[0].min.y;
    let a_max_y = m.contacts[0].max.y;
    let b_min_x = m.contacts[1].min.x;
    let b_max_x = m.contacts[1].max.x;
    let b_min_y = m.contacts[1].min.y;
    let b_max_y = m.contacts[1].max.y;
    // Calculate edges
    let left_edge = Vector3::new(a_min_x - b_max_x, 0.0, 0.0);
    let right_edge = Vector3::new(a_max_x - b_min_x, 0.0, 0.0);
    let top_edge = Vector3::new(0.0, a_min_y - b_max_y, 0.0);
    let bottom_edge = Vector3::new(0.0, a_max_y - b_min_y, 0.0);
    // Get the smallest difference
    let t: Vector3<f32> = {
        let left_edge_mag = (left_edge.x * left_edge.x + left_edge.y * left_edge.y).sqrt();
        let right_edge_mag = (right_edge.x * right_edge.x + right_edge.y * right_edge.y).sqrt();
        let top_edge_mag = (top_edge.x * top_edge.x + top_edge.y * top_edge.y).sqrt();
        let bottom_edge_mag =
            (bottom_edge.x * bottom_edge.x + bottom_edge.y * bottom_edge.y).sqrt();
        // Compare
        let first_cmp = {
            if left_edge_mag < right_edge_mag {
                left_edge_mag
            } else {
                right_edge_mag
            }
        };
        let second_cmp = {
            if top_edge_mag < bottom_edge_mag {
                top_edge_mag
            } else {
                bottom_edge_mag
            }
        };
        // Resolve
        if first_cmp < second_cmp {
            if first_cmp == left_edge_mag {
                Vector3::new(left_edge.x, left_edge.y, 0.0)
            } else {
                Vector3::new(right_edge.x, right_edge.y, 0.0)
            }
        } else {
            if second_cmp == top_edge_mag {
                Vector3::new(top_edge.x, top_edge.y, 0.0)
            } else {
                Vector3::new(bottom_edge.x, bottom_edge.y, 0.0)
            }
        }
        // Vector2::new(0.0,0.0)
    };
    t
}
