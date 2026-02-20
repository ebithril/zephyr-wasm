use macroquad::prelude::*;
use hecs::World;
use crate::components::*;

/// Resolve CircleVSCircle collisions.
pub fn collision_system(world: &mut World) {
    let mut collisions = Vec::new();

    {
        let mut query = world.query::<(hecs::Entity, &Transform, &CircleCollider)>();
        let colliders: Vec<_> = query
            .iter()
            .map(|(id, transform, collider)| (id, transform.position, collider.radius))
            .collect();

        for i in 0..colliders.len() {
            for j in i + 1..colliders.len() {
                let (id1, pos1, rad1) = colliders[i];
                let (id2, pos2, rad2) = colliders[j];

                if pos1.distance(pos2) < (rad1 + rad2) {
                    collisions.push((id1, id2));
                }
            }
        }
    }
    // TODO: Process collisions
}
