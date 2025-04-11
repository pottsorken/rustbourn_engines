use crate::common::{Obstacle, MAP_CONFIG, OBSTACLE_CONFIG};
use crate::db_connection::{load_obstacles, CtxWrapper};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn setup_obstacle(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    ctx_wrapper: Res<CtxWrapper>,
    mut _query: Query<(&mut Transform, &Obstacle)>,
) {
    let _window = window_query.get_single().unwrap();
    let obstacles = load_obstacles(&ctx_wrapper);

    for obstacle in obstacles {
        // TODO: May need reducer callback for on
        // obstacle update

        // Do not spawn if obstacle with same id is already spawned
        let obstacle_id = obstacle.2;
        //for (transf, obst) in &mut query.iter() {
        //    if obst.id == obstacle_id {
        //        return;
        //    }
        //}

        let random_x = obstacle.0;
        let random_y = obstacle.1;
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
            Obstacle { id: obstacle_id },
        ));
    }
}
