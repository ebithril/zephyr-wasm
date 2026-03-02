use crate::components::*;
use hecs::World;
use macroquad::prelude::*;

pub fn ship_movement_system(world: &mut World, dt: f32, global_air_resistance: f32) {
    for (transform, velocity, engine, cockpit, intent) in world.query_mut::<(
        &mut Transform,
        &mut Velocity,
        &Engine,
        &Cockpit,
        &mut ShipIntent,
    )>() {
        transform.rotation += intent.rotation_input * engine.turn_rate * dt;

        let dir = vec2(transform.rotation.sin(), -transform.rotation.cos());
        if intent.thrusting {
            velocity.0 += dir * engine.acceleration * dt;
        }

        let resistance = global_air_resistance * cockpit.air_resistance_mod;
        if velocity.0.length() > 0.0 {
            let opp_dir = -velocity.0.normalize();
            velocity.0 += opp_dir * resistance * dt;
        }

        if velocity.0.length() > engine.max_speed {
            velocity.0 = velocity.0.normalize() * engine.max_speed;
        }

        let mut delta_vector = velocity.0 * dt;

        let next_position = delta_vector + transform.position;

        if next_position.y > 8000. {
            velocity.0.y *= -1.;

            velocity.0.y *= 0.5;

            if velocity.0.y < 10. {
                velocity.0.y -= 250.;
            }

            delta_vector = velocity.0 * dt;

            let velocity_heading = delta_vector.x.atan2(delta_vector.y);
            let ship_heading = dir.x.atan2(dir.y);

            transform.rotation = ship_heading - velocity_heading;

            //nextPosition = deltaVector + mySpace.GetPosition();
        } else if next_position.y < 1000. {
            intent.thrusting = false; // TODO: This won't work atm but signals intent

            if next_position.y < -50. {
                velocity.0.y += 0.2;
            }

            delta_vector = velocity.0 * dt;

            //nextPosition = deltaVector + mySpace.GetPosition();
        }
    }
}

pub fn ship_animation_system(world: &mut World) {
    for (transform, anim, visuals, sprite) in
        world.query_mut::<(&Transform, &mut ShipAnimation, &ShipVisuals, &mut Sprite)>()
    {
        let nose_dir = vec2(-transform.rotation.sin(), transform.rotation.cos());

        anim.flipped = nose_dir.x < 0.0;

        let tilt_val = nose_dir.y;

        let t = (tilt_val + 1.0) / 2.0;

        let frame_idx = (t * (anim.total_frames as f32 - 1.0)).round() as u32;
        anim.current_frame = frame_idx.clamp(0, anim.total_frames - 1);

        let frame_x = (anim.current_frame % visuals.sprite_cols) as f32 * visuals.sprite_width;
        let frame_y = (anim.current_frame / visuals.sprite_cols) as f32 * visuals.sprite_height;

        sprite.source_rect = Some(Rect::new(
            frame_x,
            frame_y,
            visuals.sprite_width,
            visuals.sprite_height,
        ));
    }
}
