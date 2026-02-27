use hecs::World;
use macroquad::prelude::*;

mod components;
mod level_loader;
mod systems;

use components::*;
use systems::*;

async fn load_ship_texture(path: &str, color: Color) -> Texture2D {
    match load_texture(path).await {
        Ok(tex) => {
            tex.set_filter(FilterMode::Nearest);
            tex
        }
        Err(_) => {
            let mut bytes = vec![0u8; 32 * 32 * 4];
            for i in 0..32 * 32 {
                bytes[i * 4] = (color.r * 255.0) as u8;
                bytes[i * 4 + 1] = (color.g * 255.0) as u8;
                bytes[i * 4 + 2] = (color.b * 255.0) as u8;
                bytes[i * 4 + 3] = (color.a * 255.0) as u8;
            }
            Texture2D::from_rgba8(32, 32, &bytes)
        }
    }
}

#[macroquad::main("Zephyr")]
async fn main() {
    let mut world = World::new();

    // 1. Load Level Data
    let level_settings =
        match level_loader::load_level("Resources/Data/level0.xml", &mut world).await {
            Ok(settings) => settings,
            Err(e) => {
                eprintln!("Failed to load level: {}", e);
                level_loader::LevelSettings {
                    gravity: 400.0,
                    air_resistance: 240.0,
                }
            }
        };

    // 2. Load ship modules
    let hull_tex = load_ship_texture("Resources/Gfx/playerTurnSpriteSheetDefault.png", WHITE).await;
    let engine_tex = load_ship_texture(
        "Resources/Gfx/PlayerShipModules/playerTurnSpriteSheetEngineNeedle.png",
        BLUE,
    )
    .await;
    let weapon_tex = load_ship_texture(
        "Resources/Gfx/PlayerShipModules/playerTurnSpriteSheetWeaponSpreadshot.png",
        RED,
    )
    .await;
    let cockpit_tex = load_ship_texture(
        "Resources/Gfx/PlayerShipModules/playerTurnSpriteSheetHeadHammer.png",
        YELLOW,
    )
    .await;

    // 3. Spawn Player
    let player = world.spawn((
        Transform {
            position: vec2(4000.0, 7700.0),
            rotation: 0.0,
        },
        Velocity(vec2(0.0, 0.0)),
        ShipVisuals {
            sprite_width: 128.0,
            sprite_height: 128.0,
            sprite_cols: 16,
        },
        Sprite {
            texture: hull_tex,
            source_rect: None,
            dest_size: None,
            layer: RenderLayer::Default,
        },
        ShipAnimation {
            total_frames: 32,
            current_frame: 0,
            flipped: false,
        },
        Engine {
            acceleration: 3000.0,
            max_speed: 400.0,
            turn_rate: 3.0,
        },
        Cockpit {
            air_resistance_mod: 1.0,
            health_regen_rate: 1.0,
        },
        ShipIntent::default(),
        PlayerControlled,
        CircleCollider { radius: 32.0 },
    ));

    // 4. Spawn Camera targeting the player
    world.spawn((GameCamera {
        target: Some(player),
        offset: vec2(4000.0, 7700.0),
        screen_width: screen_width(),
        screen_height: screen_height(),
        smoothing: 5.0,
    },));

    // Loop
    loop {
        let dt = get_frame_time();
        clear_background(BLACK);

        // Control systems
        player_input_system(&mut world);
        ai_controller_system(&mut world, dt);

        // Movement systems
        // Use values from level_loader
        ship_movement_system(&mut world, dt, level_settings.air_resistance);
        ship_animation_system(&mut world);

        // Physics system (gravity)
        for (transform, velocity) in world.query_mut::<(&mut Transform, &mut Velocity)>() {
            velocity.0.y += level_settings.gravity * dt;
            transform.position += velocity.0 * dt;
            const WORLD_WIDTH: f32 = 8192.0;
            transform.position.x = (transform.position.x % WORLD_WIDTH + WORLD_WIDTH) % WORLD_WIDTH;
        }

        camera_system(&mut world, dt);

        collision_system(&mut world);
        weaponry_system(&mut world, dt);

        render_system(&mut world);

        next_frame().await
    }
}
