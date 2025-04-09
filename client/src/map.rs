use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use noisy_bevy::simplex_noise_2d;
use crate::common::MAP_CONFIG;
use image::{io::Reader as ImageReader, GenericImageView};

pub fn setup_tilemap(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture_handle: Vec<Handle<Image>> = MAP_CONFIG
        .tile_textures
        .iter()
        .map(|path|asset_server.load(*path))
        .collect();

    // New map with 64x64 chunks being 32x32 tiles
    let map_size = MAP_CONFIG.map_size;
    let tile_size = MAP_CONFIG.tile_size; // tiles are 16x16 pixels
    let grid_size = tile_size.into(); // Grid size == tile size
    let map_type = TilemapType::default();

    // New tile storage
    let mut tile_storage = TileStorage::empty(map_size);

    // spawn entity
    let tilemap_entity = commands.spawn_empty().id();


    // Specify the path to your PNG file
    let image_path = r"\\?\C:\Users\Denise\Desktop\Studier\CINTE\II1305 Projektkurs\rustbourn_engines\client\assets\tribasicmap.png";

    // Load the image using the `image` crate
    let img = ImageReader::open(image_path)
        .expect("Failed to open image")
        .decode()
        .expect("Failed to decode image");

    // Get the dimensions of the image
    let (width, height) = img.dimensions();
    println!("Image dimensions: {}x{}", width, height);

    

    // // Specify the pixel you want to query (x, y)
    // let x = 10;
    // let y = 20;

    // // Ensure the requested pixel is within bounds
    // if x < width && y < height {
    //     // Get the pixel at (x, y)
    //     let pixel = img.get_pixel(x, y);
    //     println!(
    //         "Pixel at ({}, {}): R={}, G={}, B={}, A={}",
    //         x, y, pixel[0], pixel[1], pixel[2], pixel[3]
    //     );
    // } else {
    //     println!("Pixel coordinates ({}, {}) are out of bounds!", x, y);
    // }

    for y in 0..height {
        for x in 0..width {
            let tile_pos = TilePos { x, y };

            let pixel = img.get_pixel(x, y);
            // println!(
            //     "Pixel at ({}, {}): R={}, G={}, B={}, A={}",
            //     x, y, pixel[0], pixel[1], pixel[2], pixel[3]
            // );
            
                let r = pixel[0];
                let g = pixel[1];
                let b = pixel[2];

                // let texture_index = if r == 0 && g == 255 && b == 0 {
                //     println!("Found grass tile at ({}, {})", x, y);
                //     2 // Green -> Grass
                // } else if r == 255 && g == 0 && b == 0 {
                //     println!("Found stone tile at ({}, {})", x, y);
                //     1 // Red -> Stone
                // } else {
                //     println!("Found dirt tile at ({}, {})", x, y);
                //     0 // Default -> Dirt
                // };

                let texture_index = if g == 255 {
                    // println!("Found grass tile at ({}, {})", x, y);
                    2 // Green -> Grass
                } else if r == 255 {
                    // println!("Found stone tile at ({}, {})", x, y);
                    1 // Red -> Stone
                } else {
                    // println!("Found dirt tile at ({}, {})", x, y);
                    0 // Default -> Dirt
                };


                let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex(texture_index), // first tile in tileset
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


// // pub fn setup_tilemap(mut commands: Commands, asset_server: Res<AssetServer>) {
// //     let texture_handle: Vec<Handle<Image>> = MAP_CONFIG
// //         .tile_textures
// //         .iter()
// //         .map(|path|asset_server.load(*path))
// //         .collect();

// //     // New map with 64x64 chunks being 32x32 tiles
// //     let map_size = MAP_CONFIG.map_size;
// //     let tile_size = MAP_CONFIG.tile_size; // tiles are 16x16 pixels
// //     let grid_size = tile_size.into(); // Grid size == tile size
// //     let map_type = TilemapType::default();

// //     // New tile storage
// //     let mut tile_storage = TileStorage::empty(map_size);

// //     // spawn entity
// //     let tilemap_entity = commands.spawn_empty().id();

// //     let noise_scale = 0.1;

// //     // Fill the tilemap with some tiles
// //     for x in 0..map_size.x {
// //         for y in 0..map_size.y {
// //             let tile_pos = TilePos { x, y };

// //             // Determine tile texture
// //             let noise_value = simplex_noise_2d(Vec2::new(
// //                 x as f32 * MAP_CONFIG.noise_scale, 
// //                 y as f32 * MAP_CONFIG.noise_scale,
// //             ));

// //             let texture_index = if noise_value > 0.5 {
// //                 2 // grass
// //             } else if noise_value > 0.0 {
// //                 1 // stone
// //             } else {
// //                 0 // dirt
// //             };

// //             let tile_entity = commands
// //                 .spawn(TileBundle {
// //                     position: tile_pos,
// //                     tilemap_id: TilemapId(tilemap_entity),
// //                     texture_index: TileTextureIndex(texture_index), // first tile in tileset
// //                     ..Default::default()
// //                 })
// //                 .id();
            
// //             tile_storage.set(&tile_pos, tile_entity);
// //         }
// //     }

// //     commands.entity(tilemap_entity).insert(TilemapBundle {
// //         grid_size,
// //         map_type,
// //         size: map_size,
// //         storage: tile_storage,
// //         texture: TilemapTexture::Vector(texture_handle),
// //         tile_size,
// //         transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
// //         ..Default::default()
// //     });
// // }















// #[derive(Resource, Default)]
// pub struct MapState {
//     map_loaded: bool,
//     image_handle: Handle<Image>,
// }

// pub fn setup_map_state(mut commands: Commands, asset_server: Res<AssetServer>) {
//     println!("Setting up map state...");
    
//     let image_handle: Handle<Image> = asset_server.load("basicmap.png");
//     println!("Loading image: basicmap.png");
    
//     // Try to get the full path to the asset
//     let asset_path = std::path::Path::new("assets").join("basicmap.png");
//     println!("Looking for asset at: {:?}", asset_path);
//     if asset_path.exists() {
//         println!("Asset file exists!");
//         if let Ok(metadata) = std::fs::metadata(&asset_path) {
//             println!("Asset file size: {} bytes", metadata.len());
//         }
//     } else {
//         println!("Asset file not found!");
//     }
    
//     commands.insert_resource(MapState {
//         map_loaded: false,
//         image_handle,
//     });
// }

// pub fn setup_tilemap(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     images: Res<Assets<Image>>,
//     mut map_state: ResMut<MapState>,
// ) {
//     if map_state.map_loaded {
//         return;
//     }

//     // Load tile textures
//     let texture_handle: Vec<Handle<Image>> = MAP_CONFIG
//         .tile_textures
//         .iter()
//         .map(|path| asset_server.load(*path))
//         .collect();

//     // Check if the map image is loaded
//     if let Some(map_image) = images.get(&map_state.image_handle) {
//         println!("Map image loaded successfully!");
//         let pixel_data = &map_image.data;
        
//         // Verify the image data
//         println!("Image data details:");
//         println!("Raw texture size: {:?}", map_image.texture_descriptor.size);
//         println!("Data length: {}", pixel_data.len());
//         println!("Format: {:?}", map_image.texture_descriptor.format);

//         // Testa printa skit
//         let descriptor = image.texture_descriptor(); // Access metadata

//         let width = descriptor.size.width;
//         let height = descriptor.size.height;
    
//         println!("Image dimensions: {}x{}", width, height);

//         // let old_color = map_image.get_color_at(1, 1).unwrap();
//         // println!("GET THE COLOR: {}", old_color);

//         let map_image_handle = asset_server.load("basicmap.png");

//     // fn read_pixel_color(images: Res<Assets<Image>>, image_handle: Handle<Image>) {
//         if let Some(map_image) = images.get(&map_image_handle) {
//             // Specify pixel coordinates
//             let x = 10;
//             let y = 15;

//             // Get the color at the specified pixel
//             match map_image.get_color_at(x, y) {
//                 Ok(color) => {
//                     println!("Pixel color at ({}, {}): {:?}", x, y, color);
//                 }
//                 Err(err) => {
//                     println!("Failed to get color at ({}, {}): {:?}", x, y, err);
//                 }
//             }
//         } else {
//             println!("Image not loaded yet.");
//         }
//     // }

        
//         // Force the correct dimensions
//         let map_width = 32;
//         let map_height = 32;
        
//         if pixel_data.len() != (map_width * map_height * 4) as usize {
//             println!("Warning: Unexpected pixel data length!");
//             println!("Expected: {}, Got: {}", map_width * map_height * 4, pixel_data.len());
//             return;
//         }

//         let map_size = TilemapSize { x: map_width, y: map_height };
//         let tile_size = TilemapTileSize { x: 128.0, y: 128.0 };
//         let grid_size = tile_size.into();
//         let map_type = TilemapType::default();

//         // Create tile storage
//         let mut tile_storage = TileStorage::empty(map_size);
//         let tilemap_entity = commands.spawn_empty().id();

//         // Fill the tilemap based on the image pixels
//         for y in 0..map_height {
//             for x in 0..map_width {
//                 let tile_pos = TilePos { x, y };
//                 let index = ((y * map_width + x) * 4) as usize;
                
//                 if index + 2 >= pixel_data.len() {
//                     println!("Warning: Pixel index out of bounds at ({}, {})", x, y);
//                     continue;
//                 }

//                 let r = pixel_data[index];
//                 let g = pixel_data[index + 1];
//                 let b = pixel_data[index + 2];

//                 println!("Checking pixel at ({}, {}): RGB({}, {}, {})", x, y, r, g, b);

//                 let texture_index = if r == 0 && g == 255 && b == 0 {
//                     println!("Found grass tile at ({}, {})", x, y);
//                     2 // Green -> Grass
//                 } else if r == 255 && g == 0 && b == 0 {
//                     println!("Found stone tile at ({}, {})", x, y);
//                     1 // Red -> Stone
//                 } else {
//                     println!("Found dirt tile at ({}, {})", x, y);
//                     0 // Default -> Dirt
//                 };

//                 let tile_entity = commands
//                     .spawn(TileBundle {
//                         position: tile_pos,
//                         tilemap_id: TilemapId(tilemap_entity),
//                         texture_index: TileTextureIndex(texture_index),
//                         ..Default::default()
//                     })
//                     .id();

//                 tile_storage.set(&tile_pos, tile_entity);
//             }
//         }

//         commands.entity(tilemap_entity).insert(TilemapBundle {
//             grid_size,
//             map_type,
//             size: map_size,
//             storage: tile_storage,
//             texture: TilemapTexture::Vector(texture_handle),
//             tile_size,
//             transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
//             ..Default::default()
//         });
//         println!("Tilemap entity spawned!");

//         map_state.map_loaded = true;
//     } else {
//         println!("Waiting for map image to load...");
//     }
// }

// // pub fn setup_tilemap(
// //     mut commands: Commands,
// //     asset_server: Res<AssetServer>,
// //     images: Res<Assets<Image>>,
// // ) {
// //     let texture_handle: Vec<Handle<Image>> = MAP_CONFIG
// //         .tile_textures
// //         .iter()
// //         .map(|path|asset_server.load(*path))
// //         .collect();

// //     // Load the map image
// //     let image_handle = asset_server.load("basicmap.png");

// //     // Access the image data (after it's loaded)
// //     let image = images.get(&image_handle).unwrap();
// //     let pixel_data = &image.data;

// //     // Map dimensions (example)
// //     let width = image.texture_descriptor.size.width;
// //     let height = image.texture_descriptor.size.height;

// //     // New map with 64x64 chunks being 32x32 tiles
// //     let map_size = MAP_CONFIG.map_size;
// //     let tile_size = MAP_CONFIG.tile_size; // tiles are 16x16 pixels
// //     let grid_size = tile_size.into(); // Grid size == tile size
// //     let map_type = TilemapType::default();

// //     // New tile storage
// //     let mut tile_storage = TileStorage::empty(map_size);

// //     // spawn entity
// //     let tilemap_entity = commands.spawn_empty().id();

// //     for y in 0..height {
// //         for x in 0..width {
// //             let tile_pos = TilePos { x, y };
// //             let index = (y * width + x) as usize * 4; // RGBA8 format
// //             let r = pixel_data[index];
// //             let g = pixel_data[index + 1];
// //             let b = pixel_data[index + 2];

// //             // Map colors to tile types
// //             let texture_index = if r == 214 && g == 39 && b == 245 {
// //                 // Purple -> Border
// //                 1
// //             } else if r == 9 && g == 173 && b == 13 {
// //                 // Green -> Grass tile
// //                 2
// //             } else {
// //                 0
// //             };

// //             let tile_entity = commands
// //             .spawn(TileBundle {
// //                 position: tile_pos,
// //                 tilemap_id: TilemapId(tilemap_entity),
// //                 texture_index: TileTextureIndex(texture_index), // first tile in tileset
// //                 ..Default::default()
// //             })
// //             .id();
            
// //             tile_storage.set(&tile_pos, tile_entity);
// //         }
// //     }
// // }

// // pub fn setup_tilemap(mut commands: Commands, asset_server: Res<AssetServer>, images: Res<Assets<Image>>) {
// //     let texture_handle: Vec<Handle<Image>> = MAP_CONFIG
// //         .tile_textures
// //         .iter()
// //         .map(|path|asset_server.load(*path))
// //         .collect();

// //     let map_image_handler = asset_server.load("basicmap.png");
// //     let map_image = images.get(&map_image_handler).unwrap();
// //     let map_pixel_data = &map_image.data;

// //     // New map with 64x64 chunks being 32x32 tiles
// //     let map_size = MAP_CONFIG.map_size;
// //     let tile_size = MAP_CONFIG.tile_size; // tiles are 16x16 pixels
// //     let grid_size = tile_size.into(); // Grid size == tile size
// //     let map_type = TilemapType::default();

// //     // New tile storage
// //     let mut tile_storage = TileStorage::empty(map_size);

// //     // spawn entity
// //     let tilemap_entity = commands.spawn_empty().id();


// //     // Fill the tilemap with some tiles
// //     for x in 0..map_size.x {
// //         for y in 0..map_size.y {

// //             let index = (y * map_image.texture_descriptor.size.width + x) as usize * 4; // Assuming RGBA8 format
// //             let r = map_pixel_data[index];
// //             let g = map_pixel_data[index + 1];
// //             let b = map_pixel_data[index + 2];
// //             let tile_pos = TilePos { x, y };
    
// //             // Map colors to tile types
// //             let texture_index = if r == 214 && g == 39 && b == 245 {
// //                 // Purple -> Border
// //                 1
// //             } else if r == 9 && g == 173 && b == 13 {
// //                 // Green -> Grass tile
// //                 2
// //             } else {
// //                 0
// //             };

// //             // // Determine tile texture
// //             // let noise_value = simplex_noise_2d(Vec2::new(
// //             //     x as f32 * MAP_CONFIG.noise_scale, 
// //             //     y as f32 * MAP_CONFIG.noise_scale,
// //             // ));

// //             // let texture_index = if noise_value > 0.5 {
// //             //     2 // grass
// //             // } else if noise_value > 0.0 {
// //             //     1 // stone
// //             // } else {
// //             //     0 // dirt
// //             // };

// //             let tile_entity = commands
// //                 .spawn(TileBundle {
// //                     position: tile_pos,
// //                     tilemap_id: TilemapId(tilemap_entity),
// //                     texture_index: TileTextureIndex(texture_index), // first tile in tileset
// //                     ..Default::default()
// //                 })
// //                 .id();
            
// //             tile_storage.set(&tile_pos, tile_entity);
// //         }
// //     }

// //     commands.entity(tilemap_entity).insert(TilemapBundle {
// //         grid_size,
// //         map_type,
// //         size: map_size,
// //         storage: tile_storage,
// //         texture: TilemapTexture::Vector(texture_handle),
// //         tile_size,
// //         transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
// //         ..Default::default()
// //     });
// // }


// // pub fn setup_tilemap(mut commands: Commands, asset_server: Res<AssetServer>) {
// //     let texture_handle: Vec<Handle<Image>> = MAP_CONFIG
// //         .tile_textures
// //         .iter()
// //         .map(|path|asset_server.load(*path))
// //         .collect();

// //     // New map with 64x64 chunks being 32x32 tiles
// //     let map_size = MAP_CONFIG.map_size;
// //     let tile_size = MAP_CONFIG.tile_size; // tiles are 16x16 pixels
// //     let grid_size = tile_size.into(); // Grid size == tile size
// //     let map_type = TilemapType::default();

// //     // New tile storage
// //     let mut tile_storage = TileStorage::empty(map_size);

// //     // spawn entity
// //     let tilemap_entity = commands.spawn_empty().id();

// //     let noise_scale = 0.1;

// //     // Fill the tilemap with some tiles
// //     for x in 0..map_size.x {
// //         for y in 0..map_size.y {
// //             let tile_pos = TilePos { x, y };

// //             // Determine tile texture
// //             let noise_value = simplex_noise_2d(Vec2::new(
// //                 x as f32 * MAP_CONFIG.noise_scale, 
// //                 y as f32 * MAP_CONFIG.noise_scale,
// //             ));

// //             let texture_index = if noise_value > 0.5 {
// //                 2 // grass
// //             } else if noise_value > 0.0 {
// //                 1 // stone
// //             } else {
// //                 0 // dirt
// //             };

// //             let tile_entity = commands
// //                 .spawn(TileBundle {
// //                     position: tile_pos,
// //                     tilemap_id: TilemapId(tilemap_entity),
// //                     texture_index: TileTextureIndex(texture_index), // first tile in tileset
// //                     ..Default::default()
// //                 })
// //                 .id();
            
// //             tile_storage.set(&tile_pos, tile_entity);
// //         }
// //     }

// //     commands.entity(tilemap_entity).insert(TilemapBundle {
// //         grid_size,
// //         map_type,
// //         size: map_size,
// //         storage: tile_storage,
// //         texture: TilemapTexture::Vector(texture_handle),
// //         tile_size,
// //         transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
// //         ..Default::default()
// //     });
// // }
