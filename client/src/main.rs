use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use noisy_bevy::simplex_noise_2d; // For map generation. May be temporary

mod player;

use player::Player;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window : Some(Window{
                    title : String::from("Rustbourn Engines"),
                    position : WindowPosition::Centered(MonitorSelection::Primary),
                    ..Default::default()
                }),
                ..Default::default()
            })
        )
        .add_plugins(TilemapPlugin)
        .add_systems(Startup, (setup_camera, player::setup_player, setup_tilemap))
        .add_systems(Update, (player_movement, camera_follow))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 999.9),
        ..default()
    });
}

// player::setup_player(Commands, Res<AssetServer>);

fn setup_tilemap(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture_handle: Vec<Handle<Image>> = vec![
        asset_server.load("sprites/td_tanks/dirt.png"),
        asset_server.load("sprites/td_tanks/stone.png"),
        asset_server.load("sprites/td_tanks/grass.png"),
    ];
    // New map with 64x64 chunks being 32x32 tiles
    let map_size = TilemapSize { x: 64, y: 64 };
    let tile_size = TilemapTileSize { x: 128.0, y: 128.0 }; // tiles are 16x16 pixels
    let grid_size = tile_size.into(); // Grid size == tile size
    let map_type = TilemapType::default();

    // New tile storage
    let mut tile_storage = TileStorage::empty(map_size);

    // spawn entity
    let tilemap_entity = commands.spawn_empty().id();

    let noise_scale = 0.1;

    // Fill the tilemap with some tiles
    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };

            // Determine tile texture
            let noise_value =
                simplex_noise_2d(Vec2::new(x as f32 * noise_scale, y as f32 * noise_scale));
            let texture_index = if noise_value > 0.5 {
                2 // grass
            } else if noise_value > 0.0 {
                1 // stone
            } else {
                0 // dirt
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
    time: Res<Time>,
) {
    let follow_speed = 3.0;

    if let Ok(player_transform) = player_query.get_single() {
        for mut camera_transform in camera_query.iter_mut() {
            camera_transform.translation = camera_transform.translation.lerp(
                Vec3::new(
                    player_transform.translation.x,
                    player_transform.translation.y,
                    camera_transform.translation.z, // Ensure camera z-pos unchanged
                ),
                follow_speed * time.delta_secs(),
            );
        }
    }
}
