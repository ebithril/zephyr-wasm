use std::collections::HashMap;
use macroquad::prelude::*;
use std::cell::RefCell;

pub type AssetId = i32;

struct Asset {
    path: String,
    texture: Option<Texture2D>,
    ref_count: u32,
}

struct AssetManager {
    assets: HashMap<AssetId, Asset>,
    path_to_id: HashMap<String, AssetId>,
    load_queue: Vec<AssetId>,
    next_id: AssetId,
    placeholder: Texture2D,
}

thread_local! {
    static INSTANCE: RefCell<AssetManager> = RefCell::new(AssetManager::new());
}

impl AssetManager {
    fn new() -> Self {
        // Create a 1x1 magenta placeholder
        let placeholder = Texture2D::from_rgba8(1, 1, &[255, 0, 255, 255]);
        Self {
            assets: HashMap::new(),
            path_to_id: HashMap::new(),
            load_queue: Vec::new(),
            next_id: 1,
            placeholder,
        }
    }
}

pub fn get_texture_ref(path: &str) -> AssetId {
    INSTANCE.with(|am| {
        let mut am = am.borrow_mut();
        if let Some(&id) = am.path_to_id.get(path) {
            if let Some(asset) = am.assets.get_mut(&id) {
                asset.ref_count += 1;
                return id;
            }
        }
        
        let id = am.next_id;
        am.next_id += 1;
        
        am.assets.insert(id, Asset {
            path: path.to_string(),
            texture: None,
            ref_count: 1,
        });
        am.path_to_id.insert(path.to_string(), id);
        am.load_queue.push(id);
        id
    })
}

pub fn free_asset(id: AssetId) {
    INSTANCE.with(|am| {
        let mut am = am.borrow_mut();
        let mut remove = false;
        if let Some(asset) = am.assets.get_mut(&id) {
            asset.ref_count -= 1;
            if asset.ref_count == 0 {
                remove = true;
            }
        }
        
        if remove {
            if let Some(asset) = am.assets.remove(&id) {
                am.path_to_id.remove(&asset.path);
            }
        }
    })
}

pub async fn flush_queue() {
    let to_load: Vec<(AssetId, String)> = INSTANCE.with(|am| {
        let mut am = am.borrow_mut();
        let queue = am.load_queue.drain(..).collect::<Vec<_>>();
        queue.into_iter().filter_map(|id| {
            am.assets.get(&id).map(|a| (id, a.path.clone()))
        }).collect()
    });
    
    for (id, path) in to_load {
        match load_texture(&path).await {
            Ok(tex) => {
                tex.set_filter(FilterMode::Nearest);
                INSTANCE.with(|am| {
                    if let Some(asset) = am.borrow_mut().assets.get_mut(&id) {
                        asset.texture = Some(tex);
                    }
                });
            }
            Err(e) => {
                eprintln!("Failed to load texture: {}: {}", path, e);
            }
        }
    }
}

pub fn get_texture(id: AssetId) -> Texture2D {
    INSTANCE.with(|am| {
        let am = am.borrow();
        am.assets.get(&id)
            .and_then(|a| a.texture.clone())
            .unwrap_or_else(|| am.placeholder.clone())
    })
}
