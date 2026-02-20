use hecs::World;
use crate::components::*;

/// Manage weapon firing based on intent.
pub fn weaponry_system(world: &mut World, dt: f32) {
    for (_transform, weapon, intent) in world.query_mut::<(&Transform, &mut Weapon, &ShipIntent)>() {
        if weapon.cooldown > 0.0 {
            weapon.cooldown -= dt;
        }

        if intent.firing && weapon.cooldown <= 0.0 {
            // TODO: Spawn bullet
            weapon.cooldown = 1.0 / weapon.fire_rate;
        }
    }
}
