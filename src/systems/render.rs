use crate::components::*;
use hecs::World;
use macroquad::prelude::*;

/// Main render system.
pub fn render_system(world: &mut World) {
    // 1. Render simple sprites
    for (transform, sprite) in world.query_mut::<(&Transform, &Sprite)>() {
        let params = DrawTextureParams {
            dest_size: Some(vec2(sprite.texture.width(), sprite.texture.height())),
            source: sprite.source_rect,
            rotation: transform.rotation,
            ..Default::default()
        };

        draw_texture_ex(
            &sprite.texture,
            transform.position.x - sprite.texture.width() / 2.0,
            transform.position.y - sprite.texture.height() / 2.0,
            WHITE,
            params,
        );
    }

    // 2. Render Animated Ship Visuals (Layered)
    for (transform, visuals, anim) in
        world.query_mut::<(&Transform, &ShipVisuals, &ShipAnimation)>()
    {
        // Calculate frame grid position (e.g. 16 columns per row)
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

        let pos = vec2(
            transform.position.x - visuals.sprite_width / 2.0,
            transform.position.y - visuals.sprite_height / 2.0,
        );

        // Draw Layers: Hull -> Engine -> Weapon -> Cockpit
        draw_texture_ex(&visuals.hull, pos.x, pos.y, WHITE, params.clone());

        if let Some(ref tex) = visuals.engine {
            draw_texture_ex(tex, pos.x, pos.y, WHITE, params.clone());
        }
        if let Some(ref tex) = visuals.weapon {
            draw_texture_ex(tex, pos.x, pos.y, WHITE, params.clone());
        }
        if let Some(ref tex) = visuals.cockpit {
            draw_texture_ex(tex, pos.x, pos.y, WHITE, params.clone());
        }
    }

    // --- Debug Info ---
    draw_text(
        "WASD/Arrows to Move | Space to Fire",
        10.0,
        20.0,
        20.0,
        WHITE,
    );
    if is_key_down(KeyCode::W)
        || is_key_down(KeyCode::A)
        || is_key_down(KeyCode::S)
        || is_key_down(KeyCode::D)
    {
        draw_text("INPUT DETECTED", 10.0, 40.0, 20.0, GREEN);
    }
}
