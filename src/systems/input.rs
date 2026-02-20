use macroquad::prelude::*;
use hecs::World;
use crate::components::*;

/// Handle player inputs and map them to ShipIntent.
pub fn player_input_system(world: &mut World) {
    for (intent, _player) in world.query_mut::<(&mut ShipIntent, &PlayerControlled)>() {
        intent.rotation_input = 0.0;
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) { intent.rotation_input -= 1.0; }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) { intent.rotation_input += 1.0; }

        intent.thrusting = is_key_down(KeyCode::W) || is_key_down(KeyCode::Up);
        intent.firing = is_key_down(KeyCode::Space);
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
