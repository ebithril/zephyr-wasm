use crate::components::*;
use hecs::World;
use macroquad::prelude::*;

pub fn camera_system(world: &mut World, dt: f32) {
    let level_size = 8192.;
    let mut target_position = Vec2::default();
    let mut target_velocity = Vec2::default();

    for (transform, velocity, _) in world
        .query::<(&Transform, &Velocity, &PlayerControlled)>()
        .iter()
    {
        target_position = transform.position.clone();
        target_velocity = velocity.0.clone();
        break;
    }

    for camera in world.query_mut::<&mut GameCamera>() {
        let mut relative_position = camera.offset.clone();

        let half_level_size = level_size / 2.;
        let x_diff = target_position.x - relative_position.x;
        if x_diff > half_level_size {
            relative_position.x += level_size;
        } else if x_diff < -half_level_size {
            relative_position.x -= level_size;
        }

        camera.offset = relative_position.lerp(
            (target_position + target_velocity).lerp(target_position, 0.6),
            5. * dt,
        );

        let half_screen_height = screen_height() / 2.;
        if camera.offset.y - half_screen_height < 0. {
            camera.offset.y = half_screen_height;
        } else if camera.offset.y + half_screen_height > level_size {
            camera.offset.y = level_size - half_screen_height;
        }

        break;
    }
}
