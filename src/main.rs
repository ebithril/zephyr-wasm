use hecs::World;
use macroquad::prelude::*;

mod components;
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

    // Load ship modules (Hull, Engine, Weapon, Cockpit)
    // Note: Using PNGs if they exist, otherwise fallback.
    // In production, we'd convert the .dds files to .png.
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

    // Spawn Player with Layered Visuals
    world.spawn((
        Transform {
            position: vec2(400.0, 300.0),
            rotation: 0.0,
        },
        Velocity(vec2(0.0, 0.0)),
        ShipVisuals {
            hull: hull_tex,
            engine: Some(engine_tex),
            weapon: Some(weapon_tex),
            cockpit: Some(cockpit_tex),
            sprite_width: 128.0,
            sprite_height: 128.0,
            sprite_cols: 16,
        },
        ShipAnimation {
            total_frames: 32,
            current_frame: 0,
            flipped: false,
        },
        Engine {
            acceleration: 500.0,
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

    // Loop
    loop {
        let dt = get_frame_time();
        clear_background(BLACK);

        // Control systems (set intent)
        player_input_system(&mut world);
        ai_controller_system(&mut world, dt);

        // Movement systems (read intent)
        ship_movement_system(&mut world, dt, 10.0);
        ship_animation_system(&mut world); // Update tilt frame
        physics_system(&mut world, dt);

        collision_system(&mut world);
        weaponry_system(&mut world, dt);

        render_system(&mut world);

        next_frame().await
    }
}
