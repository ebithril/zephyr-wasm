use crate::components::*;
use hecs::World;
use macroquad::prelude::*;
use quick_xml::de::from_str;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LevelData {
    pub background: Background,
    #[serde(rename = "bottomForeground")]
    pub bottom_foreground: Option<Background>,
    #[serde(rename = "topForeground")]
    pub top_foreground: Option<Background>,
    #[serde(rename = "parallaxFix")]
    pub parallax_fix: Option<Background>,
    pub gravity: f32,
    #[serde(rename = "airResistance")]
    pub air_resistance: f32,
    pub parallaxes: Option<Parallaxes>,
    pub foregrounds: Option<Parallaxes>,
}

#[derive(Debug, Deserialize)]
pub struct Background {
    #[serde(rename = "@picture")]
    pub picture: String,
}

#[derive(Debug, Deserialize)]
pub struct Parallaxes {
    #[serde(rename = "parallax")]
    pub layers: Vec<ParallaxLayer>,
}

#[derive(Debug, Deserialize)]
pub struct ParallaxLayer {
    #[serde(rename = "@relativeSpeed")]
    pub relative_speed: f32,
    #[serde(rename = "@texture")]
    pub texture: String,
}

pub async fn load_level(
    path: &str,
    world: &mut World,
) -> Result<LevelSettings, Box<dyn std::error::Error>> {
    let xml_str = load_string(path).await?;
    let data: LevelData = from_str(&xml_str)?;

    // 1. Sky Background (Furthest)
    let bg_id = crate::asset_manager::get_texture_ref(&data.background.picture.replace("../", ""));
    world.spawn((
        Transform {
            position: vec2(0.0, 0.0),
            rotation: 0.0,
        },
        Sprite {
            texture_id: bg_id,
            source_rect: None,
            dest_size: Some(Vec2::new(8192., 8192.)),
            layer: RenderLayer::Background,
        },
    ));

    // 2. Parallax Fix
    if let Some(fix) = data.parallax_fix {
        let id = crate::asset_manager::get_texture_ref(&fix.picture.replace("../", ""));
        world.spawn((
            Transform {
                position: vec2(0.0, 0.0),
                rotation: 0.0,
            },
            Sprite {
                texture_id: id,
                source_rect: None,
                dest_size: Some(Vec2::new(8192., 8192.)),
                layer: RenderLayer::ParallaxFix,
            },
            Parallax {
                relative_speed: 0.0,
                width: 8192.0,
                height: 8192.0,
                layer: -9,
            },
        ));
    }

    // 3. Parallax Background Layers (e.g. Clouds)
    if let Some(parallaxes) = data.parallaxes {
        for (i, layer) in parallaxes.layers.into_iter().enumerate() {
            let id = crate::asset_manager::get_texture_ref(&layer.texture.replace("../", ""));
            // Map XML speed (drift) to Rust speed (scroll)
            // C++: 0.0 means Ground (1.0 scroll), 1.0 means Infinity (0.0 scroll)
            let rust_speed = 1.0 - layer.relative_speed;
            world.spawn((
                Transform {
                    position: vec2(0.0, 0.0),
                    rotation: 0.0,
                },
                Sprite {
                    texture_id: id,
                    source_rect: None,
                    dest_size: Some(Vec2::new(8192., 8192.)),
                    layer: if i == 0 {
                        RenderLayer::Parallax1
                    } else {
                        RenderLayer::Parallax2
                    },
                },
                Parallax {
                    relative_speed: rust_speed,
                    width: 8192.0,
                    height: 8192.0,
                    layer: -5 + (i as i32),
                },
            ));
        }
    }

    // 4. Bottom Foreground (Map Floor)
    if let Some(fg) = data.bottom_foreground {
        let id = crate::asset_manager::get_texture_ref(&fg.picture.replace("../", ""));
        world.spawn((
            Transform {
                position: vec2(0.0, 8192.0 - 512.0),
                rotation: 0.0,
            },
            Sprite {
                texture_id: id,
                source_rect: None,
                dest_size: None,
                layer: RenderLayer::Foreground,
            },
            Parallax {
                relative_speed: 1.0, // Ground speed
                width: 8192.0,
                height: 512.0,
                layer: -1,
            },
        ));
    }

    // 5. Top Foreground
    if let Some(fg) = data.top_foreground {
        let id = crate::asset_manager::get_texture_ref(&fg.picture.replace("../", ""));
        world.spawn((
            Transform {
                position: vec2(0.0, 0.0),
                rotation: 0.0,
            },
            Sprite {
                texture_id: id,
                source_rect: None,
                dest_size: None,
                layer: RenderLayer::Foreground,
            },
            Parallax {
                relative_speed: 1.0,
                width: 8192.0,
                height: 512.0,
                layer: 10,
            },
        ));
    }

    // 6. Parallax Foreground Layers (Clouds above player)
    if let Some(foregrounds) = data.foregrounds {
        for (i, layer) in foregrounds.layers.into_iter().enumerate() {
            let id = crate::asset_manager::get_texture_ref(&layer.texture.replace("../", ""));
            let rust_speed = 1.0 - layer.relative_speed; // e.g. 1.0 - (-0.6) = 1.6
            world.spawn((
                Transform {
                    position: vec2(0.0, 0.0),
                    rotation: 0.0,
                },
                Sprite {
                    texture_id: id,
                    source_rect: None,
                    dest_size: Some(Vec2::new(8192., 8192.)),
                    layer: RenderLayer::Foreground,
                },
                Parallax {
                    relative_speed: rust_speed,
                    width: 8192.0,
                    height: 8192.0,
                    layer: 5 + (i as i32),
                },
            ));
        }
    }

    crate::asset_manager::flush_queue().await;

    Ok(LevelSettings {
        gravity: data.gravity,
        air_resistance: data.air_resistance,
    })
}

pub struct LevelSettings {
    pub gravity: f32,
    pub air_resistance: f32,
}
