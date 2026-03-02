use hecs::World;
use macroquad::prelude::*;

mod asset_manager;
mod components;
mod level_loader;
mod systems;

use asset_manager::{flush_queue, get_texture_ref};
use components::*;
use systems::*;

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

    // 2. Queue ship modules
    let hull_id = get_texture_ref("Resources/Gfx/playerTurnSpriteSheetDefault.png");
    let _engine_id =
        get_texture_ref("Resources/Gfx/PlayerShipModules/playerTurnSpriteSheetEngineNeedle.png");
    let _weapon_id = get_texture_ref(
        "Resources/Gfx/PlayerShipModules/playerTurnSpriteSheetWeaponSpreadshot.png",
    );
    let _cockpit_id =
        get_texture_ref("Resources/Gfx/PlayerShipModules/playerTurnSpriteSheetHeadHammer.png");

    // Load queued assets
    flush_queue().await;

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
            texture_id: hull_id,
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
        current_shake_length: 0.,
        shake_length: 0.,
        shake_magnitude: 0.,
    },));

    // Loop
    loop {
        let dt = if get_frame_time() > 1. {
            1. / 60.
        } else {
            get_frame_time()
        };

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
