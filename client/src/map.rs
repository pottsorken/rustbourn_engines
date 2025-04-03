use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use noisy_bevy::simplex_noise_2d;
use crate::common::MAP_CONFIG;

pub fn setup_tilemap(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    images: Res<Assets<Image>>, // Add this to access loaded images
) {
    // Load tile textures
    let texture_handle: Vec<Handle<Image>> = MAP_CONFIG
        .tile_textures
        .iter()
        .map(|path| asset_server.load(*path))
        .collect();


    // Load the map image
    // let map_image_handle = asset_server.load("/basicmap.png"); // Path to your map image
    let map_image_handle = asset_server.load("C:\\Users\\Denise\\Desktop\\Studier\\CINTE\\II1305 Projektkurs\\rustbourn_engines\\client\\assets\\basicmap-px.png"); // Path to your map image

    // Wait for the image to be loaded
    let map_image = images.get(&map_image_handle).expect("Map image not loaded yet");

    if let Some(map_image) = images.get(&map_image_handle.0) {
        // Extract pixel data from the image
        let pixel_data = &map_image.data;
        // let map_width = map_image.texture_descriptor.size.width;
        // let map_height = map_image.texture_descriptor.size.height;
        let map_width = 20;
        let map_height = 20;
    }

    // New map configuration
    // let map_size = MAP_CONFIG.map_size;
    // let tile_size = MAP_CONFIG.tile_size; // tiles are 16x16 pixels
    let map_size = TilemapSize { x: 20, y: 20 };

    // Predefined tile size (1 pixel per tile)
    let tile_size = TilemapTileSize { x: 1.0, y: 1.0 };
    let grid_size = tile_size.into(); // Grid size == tile size
    let map_type = TilemapType::default();

    // New tile storage
    let mut tile_storage = TileStorage::empty(map_size);

    // Spawn entity for the tilemap
    let tilemap_entity = commands.spawn_empty().id();

    // Fill the tilemap based on the image pixels
    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };

            // Get pixel color from the image (assuming RGBA8 format)
            let index = ((y * map_width + x) * 4) as usize; // RGBA8 has 4 bytes per pixel
            let r = pixel_data[index];
            let g = pixel_data[index + 1];
            let b = pixel_data[index + 2];

            // Map colors to texture indices (adjust as needed)
            let texture_index = if r == 214 && g == 39 && b == 245 {
                2 // Red -> Grass
            } else if r == 9 && g == 173 && b == 13 {
                1 // Green -> Stone
            } else {
                0 // Default -> Dirt
            };

            // Spawn the tile entity
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex(texture_index),
                    ..Default::default()
                })
                .id();

            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Vector(texture_handle),
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    });
}


// pub fn setup_tilemap(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     images: Res<Assets<Image>>,
// ) {
//     let texture_handle: Vec<Handle<Image>> = MAP_CONFIG
//         .tile_textures
//         .iter()
//         .map(|path|asset_server.load(*path))
//         .collect();

//     // Load the map image
//     let image_handle = asset_server.load("basicmap.png");

//     // Access the image data (after it's loaded)
//     let image = images.get(&image_handle).unwrap();
//     let pixel_data = &image.data;

//     // Map dimensions (example)
//     let width = image.texture_descriptor.size.width;
//     let height = image.texture_descriptor.size.height;

//     // New map with 64x64 chunks being 32x32 tiles
//     let map_size = MAP_CONFIG.map_size;
//     let tile_size = MAP_CONFIG.tile_size; // tiles are 16x16 pixels
//     let grid_size = tile_size.into(); // Grid size == tile size
//     let map_type = TilemapType::default();

//     // New tile storage
//     let mut tile_storage = TileStorage::empty(map_size);

//     // spawn entity
//     let tilemap_entity = commands.spawn_empty().id();

//     for y in 0..height {
//         for x in 0..width {
//             let tile_pos = TilePos { x, y };
//             let index = (y * width + x) as usize * 4; // RGBA8 format
//             let r = pixel_data[index];
//             let g = pixel_data[index + 1];
//             let b = pixel_data[index + 2];

//             // Map colors to tile types
//             let texture_index = if r == 214 && g == 39 && b == 245 {
//                 // Purple -> Border
//                 1
//             } else if r == 9 && g == 173 && b == 13 {
//                 // Green -> Grass tile
//                 2
//             } else {
//                 0
//             };

//             let tile_entity = commands
//             .spawn(TileBundle {
//                 position: tile_pos,
//                 tilemap_id: TilemapId(tilemap_entity),
//                 texture_index: TileTextureIndex(texture_index), // first tile in tileset
//                 ..Default::default()
//             })
//             .id();
            
//             tile_storage.set(&tile_pos, tile_entity);
//         }
//     }
// }

// pub fn setup_tilemap(mut commands: Commands, asset_server: Res<AssetServer>, images: Res<Assets<Image>>) {
//     let texture_handle: Vec<Handle<Image>> = MAP_CONFIG
//         .tile_textures
//         .iter()
//         .map(|path|asset_server.load(*path))
//         .collect();

//     let map_image_handler = asset_server.load("basicmap.png");
//     let map_image = images.get(&map_image_handler).unwrap();
//     let map_pixel_data = &map_image.data;

//     // New map with 64x64 chunks being 32x32 tiles
//     let map_size = MAP_CONFIG.map_size;
//     let tile_size = MAP_CONFIG.tile_size; // tiles are 16x16 pixels
//     let grid_size = tile_size.into(); // Grid size == tile size
//     let map_type = TilemapType::default();

//     // New tile storage
//     let mut tile_storage = TileStorage::empty(map_size);

//     // spawn entity
//     let tilemap_entity = commands.spawn_empty().id();


//     // Fill the tilemap with some tiles
//     for x in 0..map_size.x {
//         for y in 0..map_size.y {

//             let index = (y * map_image.texture_descriptor.size.width + x) as usize * 4; // Assuming RGBA8 format
//             let r = map_pixel_data[index];
//             let g = map_pixel_data[index + 1];
//             let b = map_pixel_data[index + 2];
//             let tile_pos = TilePos { x, y };
    
//             // Map colors to tile types
//             let texture_index = if r == 214 && g == 39 && b == 245 {
//                 // Purple -> Border
//                 1
//             } else if r == 9 && g == 173 && b == 13 {
//                 // Green -> Grass tile
//                 2
//             } else {
//                 0
//             };

//             // // Determine tile texture
//             // let noise_value = simplex_noise_2d(Vec2::new(
//             //     x as f32 * MAP_CONFIG.noise_scale, 
//             //     y as f32 * MAP_CONFIG.noise_scale,
//             // ));

//             // let texture_index = if noise_value > 0.5 {
//             //     2 // grass
//             // } else if noise_value > 0.0 {
//             //     1 // stone
//             // } else {
//             //     0 // dirt
//             // };

//             let tile_entity = commands
//                 .spawn(TileBundle {
//                     position: tile_pos,
//                     tilemap_id: TilemapId(tilemap_entity),
//                     texture_index: TileTextureIndex(texture_index), // first tile in tileset
//                     ..Default::default()
//                 })
//                 .id();
            
//             tile_storage.set(&tile_pos, tile_entity);
//         }
//     }

//     commands.entity(tilemap_entity).insert(TilemapBundle {
//         grid_size,
//         map_type,
//         size: map_size,
//         storage: tile_storage,
//         texture: TilemapTexture::Vector(texture_handle),
//         tile_size,
//         transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
//         ..Default::default()
//     });
// }


// pub fn setup_tilemap(mut commands: Commands, asset_server: Res<AssetServer>) {
//     let texture_handle: Vec<Handle<Image>> = MAP_CONFIG
//         .tile_textures
//         .iter()
//         .map(|path|asset_server.load(*path))
//         .collect();

//     // New map with 64x64 chunks being 32x32 tiles
//     let map_size = MAP_CONFIG.map_size;
//     let tile_size = MAP_CONFIG.tile_size; // tiles are 16x16 pixels
//     let grid_size = tile_size.into(); // Grid size == tile size
//     let map_type = TilemapType::default();

//     // New tile storage
//     let mut tile_storage = TileStorage::empty(map_size);

//     // spawn entity
//     let tilemap_entity = commands.spawn_empty().id();

//     let noise_scale = 0.1;

//     // Fill the tilemap with some tiles
//     for x in 0..map_size.x {
//         for y in 0..map_size.y {
//             let tile_pos = TilePos { x, y };

//             // Determine tile texture
//             let noise_value = simplex_noise_2d(Vec2::new(
//                 x as f32 * MAP_CONFIG.noise_scale, 
//                 y as f32 * MAP_CONFIG.noise_scale,
//             ));

//             let texture_index = if noise_value > 0.5 {
//                 2 // grass
//             } else if noise_value > 0.0 {
//                 1 // stone
//             } else {
//                 0 // dirt
//             };

//             let tile_entity = commands
//                 .spawn(TileBundle {
//                     position: tile_pos,
//                     tilemap_id: TilemapId(tilemap_entity),
//                     texture_index: TileTextureIndex(texture_index), // first tile in tileset
//                     ..Default::default()
//                 })
//                 .id();
            
//             tile_storage.set(&tile_pos, tile_entity);
//         }
//     }

//     commands.entity(tilemap_entity).insert(TilemapBundle {
//         grid_size,
//         map_type,
//         size: map_size,
//         storage: tile_storage,
//         texture: TilemapTexture::Vector(texture_handle),
//         tile_size,
//         transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
//         ..Default::default()
//     });
// }