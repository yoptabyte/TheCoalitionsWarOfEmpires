use bevy::prelude::*;
use std::collections::HashMap;

/// Resource to track loaded assets
#[derive(Resource, Default)]
pub struct AssetCache {
    pub models: HashMap<String, Handle<Scene>>,
}

/// Resource to track loading state
#[derive(Resource, Default)]
pub struct AssetLoadingState {
    pub is_loading: bool,
    pub loaded_count: usize,
    pub total_count: usize,
}

/// Component to mark entities that need specific assets
#[derive(Component)]
pub struct NeedsAsset {
    pub asset_path: String,
    pub loaded: bool,
}

/// System to load assets on demand
pub fn lazy_asset_loading_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut asset_cache: ResMut<AssetCache>,
    mut loading_state: ResMut<AssetLoadingState>,
    mut needs_asset_query: Query<(Entity, &mut NeedsAsset), Without<Handle<Scene>>>,
) {
    for (entity, mut needs_asset) in needs_asset_query.iter_mut() {
        if !needs_asset.loaded {
            if let Some(handle) = asset_cache.models.get(&needs_asset.asset_path) {
                // Asset already loaded, attach it
                commands.entity(entity).insert(handle.clone());
                needs_asset.loaded = true;
            } else {
                // Load the asset
                let handle: Handle<Scene> = asset_server.load(&needs_asset.asset_path);
                asset_cache.models.insert(needs_asset.asset_path.clone(), handle.clone());
                commands.entity(entity).insert(handle);
                needs_asset.loaded = true;
                loading_state.total_count += 1;
            }
        }
    }
}

/// System to preload essential assets
pub fn preload_essential_assets(
    asset_server: Res<AssetServer>,
    mut asset_cache: ResMut<AssetCache>,
) {
    // Preload only essential assets like UI elements and basic shapes
    let essential_assets = vec![
        "models/farm/factory.glb",
        "models/farm/forest.glb",
        "models/farm/mine.glb",
        "models/farm/oil_pump.glb",
    ];

    for asset_path in essential_assets {
        let handle: Handle<Scene> = asset_server.load(asset_path);
        asset_cache.models.insert(asset_path.to_string(), handle);
    }
}

/// Plugin for lazy asset loading
pub struct LazyAssetPlugin;

impl Plugin for LazyAssetPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<AssetCache>()
            .init_resource::<AssetLoadingState>()
            .add_systems(Startup, preload_essential_assets)
            .add_systems(Update, lazy_asset_loading_system);
    }
}