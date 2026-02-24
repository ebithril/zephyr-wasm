use macroquad::prelude::*;
use hecs::World;
use crate::components::*;
use std::collections::HashMap;

pub fn camera_system(world: &mut World, dt: f32) {
    // 1. Collect all potential target positions first.
    // We explicitly ask for Entity ID in this query.
    let mut target_positions = HashMap::new();
    for (id, transform) in world.query::<(hecs::Entity, &Transform)>().iter() {
        target_positions.insert(id, transform.position);
    }

    // 2. Update cameras. 
    // We do NOT ask for Entity ID here, so we just get the GameCamera component.
    for camera in world.query_mut::<&mut GameCamera>() {
        if let Some(target_id) = camera.target {
            if let Some(target_pos) = target_positions.get(&target_id) {
                // Desired camera top-left to center the target
                let desired_pos = vec2(
                    target_pos.x - camera.screen_width / 2.0,
                    target_pos.y - camera.screen_height / 2.0,
                );

                // Apply smoothing
                camera.offset += (desired_pos - camera.offset) * camera.smoothing * dt;
            }
        }
    }
}
