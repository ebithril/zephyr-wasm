use crate::components::*;
use hecs::World;
use macroquad::prelude::*;

/// Handle player inputs and map them to ShipIntent.
pub fn player_input_system(world: &mut World) {
    let mut apply_shake = false;

    for (intent, _player) in world.query_mut::<(&mut ShipIntent, &PlayerControlled)>() {
        intent.rotation_input = 0.0;
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            intent.rotation_input -= 1.0;
        }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            intent.rotation_input += 1.0;
        }

        let was_thrusting = intent.thrusting;
        intent.thrusting = is_key_down(KeyCode::W) || is_key_down(KeyCode::Up);

        if !was_thrusting && intent.thrusting {
            apply_shake = true;
        }
        intent.firing = is_key_down(KeyCode::Space);
        break;
    }

    if apply_shake {
        match world.query_mut::<&mut GameCamera>().into_iter().next() {
            Some(camera) => {
                camera.current_shake_length = 0.;
                camera.shake_magnitude = 3.;
                camera.shake_length = 0.2;
            }
            None => panic!("Expected there to be a camera"),
        }
    }
}

/// Run AI logic for hostile entities and write to ShipIntent.
pub fn ai_controller_system(world: &mut World, _dt: f32) {
    for (_intent, ai) in world.query_mut::<(&mut ShipIntent, &AiBehavior)>() {
        match ai.state {
            AiState::Attacker => {
                // TODO: Set intent.rotation_input and intent.thrusting based on player pos
            }
            AiState::Kamikaze => {
                // TODO
            }
        }
    }
}
