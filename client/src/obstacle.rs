use crate::common::{Obstacle, MAP_CONFIG, OBSTACLE_CONFIG};
use crate::db_connection::{load_obstacles, CtxWrapper};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::random;
use std::collections::HashSet;

#[derive(Resource, Default)]
pub struct SpawnedObstacles {
    pub ids: HashSet<u64>,
}

pub fn setup_obstacle(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    ctx_wrapper: Res<CtxWrapper>,
    mut query: Query<(Entity, &mut Transform, &Obstacle)>,
    mut spawned: ResMut<SpawnedObstacles>,
) {
    let _window = window_query.get_single().unwrap();
    let obstacles = load_obstacles(&ctx_wrapper);

    for obstacle in obstacles {
        let random_x = obstacle.0;
        let random_y = obstacle.1;
        let obstacle_id = obstacle.2;
        let hp = obstacle.3;

        // Clean up if it's already spawned but now has 0 HP
        if spawned.ids.contains(&obstacle_id) && hp <= 0 {
            for (entity, _, existing_obstacle) in &mut query {
                if existing_obstacle.id == obstacle_id {
                    commands.entity(entity).despawn();
                    spawned.ids.remove(&obstacle_id);
                    break;
                }
            }
            continue;
        }

        // Skip spawning dead obstacles
        if hp <= 0 {
            continue;
        }

        // Skip if already spawned
        if spawned.ids.contains(&obstacle_id) {
            continue;
        }

        let valid_x = random_x < MAP_CONFIG.safe_zone_size && random_x > -MAP_CONFIG.safe_zone_size;
        let valid_y = random_y < MAP_CONFIG.safe_zone_size && random_y > -MAP_CONFIG.safe_zone_size;
        if valid_x && valid_y {
            continue;
        }

        // Mark as spawned
        spawned.ids.insert(obstacle_id);

        commands.spawn((
            Sprite {
                custom_size: Some(OBSTACLE_CONFIG.size),
                image: asset_server.load(OBSTACLE_CONFIG.path),
                ..default()
            },
            Transform::from_xyz(random_x, random_y, 1.0),
            Obstacle { id: obstacle_id },
        ));
    }
}
