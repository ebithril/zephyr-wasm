use macroquad::prelude::*;
use hecs::World;
use crate::components::*;

/// The core Ship movement logic (Rotation, Thrust, Air Resistance).
pub fn ship_movement_system(world: &mut World, dt: f32, global_air_resistance: f32) {
    for (transform, velocity, engine, cockpit, intent) in world.query_mut::<(
        &mut Transform,
        &mut Velocity,
        &Engine,
        &Cockpit,
        &ShipIntent,
    )>() {
        // 1. Handle Rotation
        transform.rotation += intent.rotation_input * engine.turn_rate * dt;

        // 2. Handle Thrust
        if intent.thrusting {
            let dir = vec2(transform.rotation.sin(), -transform.rotation.cos());
            velocity.0 += dir * engine.acceleration * dt;
        }

        // 3. Handle Air Resistance
        let resistance = global_air_resistance * cockpit.air_resistance_mod;
        if velocity.0.length() > 0.0 {
            let opp_dir = -velocity.0.normalize();
            velocity.0 += opp_dir * resistance * dt;
        }

        // 4. Adjust Speed (Clamp to Max)
        if velocity.0.length() > engine.max_speed {
            velocity.0 = velocity.0.normalize() * engine.max_speed;
        }
    }
}

/// Update entity positions and handle gravity + level wrapping.
pub fn physics_system(world: &mut World, dt: f32) {
    let gravity = vec2(0.0, 98.1); 

    for (transform, velocity) in world.query_mut::<(&mut Transform, &mut Velocity)>() {
        velocity.0 += gravity * dt;
        transform.position += velocity.0 * dt;

        const WORLD_WIDTH: f32 = 8192.0;
        transform.position.x = (transform.position.x % WORLD_WIDTH + WORLD_WIDTH) % WORLD_WIDTH;
    }
}

/// Update Ship tilt animation frame based on its rotation.
pub fn ship_animation_system(world: &mut World) {
    for (transform, anim) in world.query_mut::<(&Transform, &mut ShipAnimation)>() {
        // Forward vector: 0 rad is Right (1, 0). 
        // We want the vector pointing out the "nose" of the ship.
        // If 0 rad in texture is Up, Nose is (sin, -cos).
        // Let's use the rotation to find the "forward" vector components.
        let nose_dir = vec2(transform.rotation.sin(), -transform.rotation.cos());

        // Flip check: If pointing left, flip the sprite.
        anim.flipped = nose_dir.x < 0.0;

        // Tilt calculation: Use the vertical component (y) to choose a frame.
        // nose_dir.y goes from -1.0 (straight up) to 1.0 (straight down).
        // We want to map this to 0..total_frames-1.
        
        // If flipped, we still want the same tilt logic relative to the vertical axis.
        let tilt_val = nose_dir.y; // -1 (up) to 1 (down)
        
        // Normalize -1..1 to 0..1
        let t = (tilt_val + 1.0) / 2.0;
        
        let frame_idx = (t * (anim.total_frames as f32 - 1.0)).round() as u32;
        anim.current_frame = frame_idx.clamp(0, anim.total_frames - 1);
    }
}
