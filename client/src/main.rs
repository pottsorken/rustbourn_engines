use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

#[derive(Component)]
struct Player {
    /// linear speed in meters per second
    movement_speed: f32,
    /// rotation speed in radians per second
    rotation_speed: f32,
}
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TilemapPlugin)
        .add_systems(Startup, (setup_camera, setup_player, setup_tilemap))
        .add_systems(Update, (player_movement, camera_follow))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 999.9),
        ..default()
    });
}

fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn a player sprite at position (0, 0) at a higher z-index than map
    commands.spawn((
        Sprite {
            custom_size: Some(Vec2::new(100.0, 100.0)), // Square size 100x100 pixels
            image: asset_server.load("sprites/top-view/robot_3Dblue.png"),
            ..default()
        },
        //TextureAtlas {
        //    layout: asset_server.load("sprites/top-view/robot_3Dblue.png"),
        //    index: 0,
        //}, -- NOTE: If asset-chart is ever used
        Transform::from_xyz(0.0, 0.0, 1.0),
        Player {
            movement_speed: 300.0,                  // meters per second
            rotation_speed: f32::to_radians(180.0), // degrees per second
        },
    ));
}

fn setup_tilemap(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture_handle: Handle<Image> = asset_server.load("sprites/tiles.png");

    // New map with 10x10 chunks being 32x32 tiles
    let map_size = TilemapSize { x: 64, y: 64 };
    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 }; // tiles are 16x16 pixels
    let grid_size = tile_size.into(); // Grid size == tile size
    let map_type = TilemapType::default();

    // New tile storage
    let mut tile_storage = TileStorage::empty(map_size);

    // spawn entity
    let tilemap_entity = commands.spawn_empty().id();

    // Fill the tilemap with some tiles
    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    //texture_index: TileTextureIndex(0), // first tile in tileset
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
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    });

    // Add atlas to array texture loader so it's preprocessed before we need to use it.
    // Only used when the atlas feature is off and we are using array textures.
    #[cfg(all(not(feature = "atlas"), feature = "render"))]
    {
        array_texture_loader.add(TilemapArrayTexture {
            texture: TilemapTexture::Single(asset_server.load("sprites/tiles.png")),
            tile_size,
            ..Default::default()
        });
    }
}

fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &Player)>,
    time: Res<Time>,
) {
    for (mut transform, player) in &mut query {
        // Handle rotation with A/D keys
        let mut rotation_dir = 0.0;
        if keyboard_input.pressed(KeyCode::KeyA) {
            rotation_dir += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            rotation_dir -= 1.0;
        }

        // Apply rotation
        if rotation_dir != 0.0 {
            transform.rotate_z(rotation_dir * player.rotation_speed * time.delta_secs());
        }

        // Handle movement with W/S keys (forward/backward relative to rotation)
        let mut move_dir = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::KeyW) {
            move_dir.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            move_dir.y -= 1.0;
        }

        // Apply movement relative to player's rotation
        if move_dir != Vec3::ZERO {
            let move_direction = transform.rotation * move_dir.normalize();
            transform.translation += move_direction * player.movement_speed * time.delta_secs();
        }
    }
}

fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for mut camera_transform in camera_query.iter_mut() {
            camera_transform.translation = player_transform.translation;
            camera_transform.translation.z = 999.9; // Ensure camera z-pos unchanged
        }
    }
}
