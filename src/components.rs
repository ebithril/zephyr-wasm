use macroquad::prelude::*;

pub struct Transform {
    pub position: Vec2,
    pub rotation: f32, // In radians
}

pub struct Velocity(pub Vec2);

pub struct Sprite {
    pub texture: Texture2D,
    pub source_rect: Option<Rect>,
}

/// Component for layered ship visuals (Hull + Modules)
pub struct ShipVisuals {
    pub hull: Texture2D,
    pub engine: Option<Texture2D>,
    pub weapon: Option<Texture2D>,
    pub cockpit: Option<Texture2D>,
    pub sprite_width: f32,
    pub sprite_height: f32,
    pub sprite_cols: u32,
}

/// Component for handling the tilt animation based on rotation
pub struct ShipAnimation {
    pub total_frames: u32,
    pub current_frame: u32,
    pub flipped: bool,
}

pub struct Health {
    pub current: f32,
    pub max: f32,
}

pub struct Shield {
    pub current: f32,
    pub max: f32,
    pub regen_rate: f32,
}

pub struct CircleCollider {
    pub radius: f32,
}

/// The "Interface" component that controllers (Player/AI) write to.
pub struct ShipIntent {
    pub rotation_input: f32, // -1.0 (left) to 1.0 (right)
    pub thrusting: bool,
    pub firing: bool,
}

impl Default for ShipIntent {
    fn default() -> Self {
        Self {
            rotation_input: 0.0,
            thrusting: false,
            firing: false,
        }
    }
}

pub struct Engine {
    pub acceleration: f32,
    pub max_speed: f32,
    pub turn_rate: f32, // radians per second
}

pub struct Cockpit {
    pub air_resistance_mod: f32,
    pub health_regen_rate: f32,
}

pub struct Weapon {
    pub fire_rate: f32,
    pub cooldown: f32,
}

pub struct AiBehavior {
    pub state: AiState,
}

#[derive(Clone, Copy, Debug)]
pub enum AiState {
    Attacker,
    Kamikaze,
}

// Tag Components
pub struct PlayerControlled;
pub struct TeamFriendly;
pub struct TeamHostile;
