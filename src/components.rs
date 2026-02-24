use macroquad::prelude::*;

#[derive(PartialOrd, Ord, PartialEq, Eq)]
pub enum RenderLayer {
    Background,
    Default,
    Foreground,
}

pub struct Transform {
    pub position: Vec2,
    pub rotation: f32, // In radians
}

pub struct Velocity(pub Vec2);

pub struct Sprite {
    pub texture: Texture2D,
    pub source_rect: Option<Rect>,
    pub dest_size: Option<Vec2>,
    pub layer: RenderLayer,
}

/// Component for layered ship visuals (Hull + Modules)
pub struct ShipVisuals {
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

/// Camera component to track a target or position
pub struct GameCamera {
    pub target: Option<hecs::Entity>,
    pub offset: Vec2,
    pub screen_width: f32,
    pub screen_height: f32,
    pub smoothing: f32,
}

pub struct Parallax {
    pub relative_speed: f32,
    pub width: f32,
    pub height: f32,
    pub layer: i32,
}
