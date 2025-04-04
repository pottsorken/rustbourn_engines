use crate::common::{Obstacle, Player, MAP_CONFIG, OBSTACLE_CONFIG, PLAYER_CONFIG};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::random;

pub fn setup_obstacle(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let _window = window_query.get_single().unwrap();

    let world_map_size = Vec2::new(
        MAP_CONFIG.map_size.x as f32 * MAP_CONFIG.tile_size.x,
        MAP_CONFIG.map_size.y as f32 * MAP_CONFIG.tile_size.y,
    );

    for _ in 0..OBSTACLE_CONFIG.count {
        let random_x = (random::<f32>() - 0.5) * world_map_size.x;
        let random_y = (random::<f32>() - 0.5) * world_map_size.y;
        let valid_x = random_x < MAP_CONFIG.safe_zone_size && random_x > -MAP_CONFIG.safe_zone_size;
        let valid_y = random_y < MAP_CONFIG.safe_zone_size && random_y > -MAP_CONFIG.safe_zone_size;
        if valid_x && valid_y {
            continue;
        }

        commands.spawn((
            Sprite {
                custom_size: Some(OBSTACLE_CONFIG.size),
                image: asset_server.load(OBSTACLE_CONFIG.path),
                ..default()
            },
            Transform::from_xyz(random_x, random_y, 1.0),
            Obstacle {},
        ));
    }
}

