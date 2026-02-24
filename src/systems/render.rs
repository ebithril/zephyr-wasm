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

    // --- DRAWING LAYERS ---
    // Helper for drawing tiled parallax
    let draw_parallax = |transform: &Transform, sprite: &Sprite, parallax: &Parallax| {
        let screen_pos = transform.position;
        let world_width = parallax.width;
        let world_height = parallax.height;

        let mut draw_x = (screen_pos.x % world_width + world_width) % world_width;
        if draw_x > 0.0 {
            draw_x -= world_width;
        }
        let mut draw_y = (screen_pos.y % world_height + world_height) % world_height;
        if draw_y > 0.0 {
            draw_y -= world_height;
        }

        for ox in 0..2 {
            for oy in 0..2 {
                draw_texture_ex(
                    &sprite.texture,
                    draw_x + (ox as f32 * world_width),
                    draw_y + (oy as f32 * world_height),
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(world_width, world_height)),
                        ..Default::default()
                    },
                );
            }
        }
    };

    // A. Collect and sort Parallax layers
    {
        let mut parallaxes: Vec<_> = world
            .query_mut::<(&Transform, &Sprite, &Parallax)>()
            .into_iter()
            .map(|(t, s, p)| (t, s, p))
            .collect();

        // Sort primarily by layer, then by relative_speed for sub-layer depth
        parallaxes.sort_by(|(_, _, p1), (_, _, p2)| {
            p1.layer
                .cmp(&p2.layer)
                .then_with(|| p1.relative_speed.partial_cmp(&p2.relative_speed).unwrap())
        });

        // B. Draw Background Parallaxes (layer < 0)
        for (transform, sprite, parallax) in parallaxes.iter().filter(|(_, _, p)| p.layer < 0) {
            draw_parallax(transform, sprite, parallax);
        }
    }

    // C. Simple Sprites (Projectiles, etc.)
    for (transform, sprite) in world
        .query_mut::<(&Transform, &Sprite)>()
        .without::<&Parallax>()
        .without::<&ShipVisuals>()
    {
        let params = DrawTextureParams {
            dest_size: Some(vec2(sprite.texture.width(), sprite.texture.height())),
            source: sprite.source_rect,
            rotation: transform.rotation,
            ..Default::default()
        };

        let screen_pos = transform.position;
        draw_texture_ex(
            &sprite.texture,
            screen_pos.x - sprite.texture.width() / 2.0,
            screen_pos.y - sprite.texture.height() / 2.0,
            WHITE,
            params,
        );
    }

    // D. Animated Ship Visuals (Layered)
    for (transform, visuals, anim) in
        world.query_mut::<(&Transform, &ShipVisuals, &ShipAnimation)>()
    {
        let frame_x = (anim.current_frame % visuals.sprite_cols) as f32 * visuals.sprite_width;
        let frame_y = (anim.current_frame / visuals.sprite_cols) as f32 * visuals.sprite_height;

        let source_rect = Some(Rect::new(
            frame_x,
            frame_y,
            visuals.sprite_width,
            visuals.sprite_height,
        ));

        let params = DrawTextureParams {
            dest_size: Some(vec2(visuals.sprite_width, visuals.sprite_height)),
            source: source_rect,
            rotation: transform.rotation,
            flip_x: anim.flipped,
            ..Default::default()
        };

        let screen_pos = transform.position;
        let draw_x = screen_pos.x - visuals.sprite_width / 2.0;
        let draw_y = screen_pos.y - visuals.sprite_height / 2.0;

        draw_texture_ex(&visuals.hull, draw_x, draw_y, WHITE, params.clone());

        if let Some(ref tex) = visuals.engine {
            draw_texture_ex(tex, draw_x, draw_y, WHITE, params.clone());
        }
        if let Some(ref tex) = visuals.weapon {
            draw_texture_ex(tex, draw_x, draw_y, WHITE, params.clone());
        }
        if let Some(ref tex) = visuals.cockpit {
            draw_texture_ex(tex, draw_x, draw_y, WHITE, params.clone());
        }
    }

    // E. Draw Foreground Parallaxes (layer >= 0)
    /*for (transform, sprite, parallax) in parallaxes.iter().filter(|(_, _, p)| p.layer >= 0) {
        draw_parallax(transform, sprite, parallax);
    }*/

    set_default_camera();

    // --- GUI ---
    draw_fps();
}
