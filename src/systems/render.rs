use crate::components::*;
use hecs::World;
use macroquad::prelude::*;

/// Main render system.
pub fn render_system(world: &mut World) {
    // 1. Get the camera offset (assuming one active camera)
    let screen_mid = Vec2::new(screen_width() / 2., screen_height() / 2.);
    let mut camera =
        Camera2D::from_display_rect(Rect::new(0., 0., screen_width(), -screen_height()));
    camera.target = screen_mid;
    for game_camera in world.query_mut::<&GameCamera>() {
        camera.target = game_camera.offset + screen_mid;
        break;
    }

    set_camera(&camera);

    let mut sprites: Vec<(&Transform, &Sprite)> = world
        .query_mut::<(&Transform, &Sprite)>()
        .into_iter()
        .collect();

    sprites.sort_by(|(_, s1), (_, s2)| s1.layer.cmp(&s2.layer));

    for (transform, sprite) in sprites {
        draw_texture_ex(
            &sprite.texture,
            transform.position.x,
            transform.position.y,
            WHITE,
            DrawTextureParams {
                dest_size: sprite.dest_size,
                source: sprite.source_rect,
                rotation: transform.rotation,
                ..DrawTextureParams::default()
            },
        );
    }

    set_default_camera();

    // --- GUI ---
    draw_fps();
}
