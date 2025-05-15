use crate::common::{AttachedBlock, Block, BLOCK_CONFIG, MAP_CONFIG};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::random;
use rand::Rng;

pub fn setup_block(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let _window = window_query.get_single().unwrap();

    let world_map_size = Vec2::new(
        MAP_CONFIG.map_size.x as f32 * MAP_CONFIG.tile_size.x / 2.0,
        MAP_CONFIG.map_size.y as f32 * MAP_CONFIG.tile_size.y / 2.0,
    );

    for _ in 0..BLOCK_CONFIG.count {
        let random_x = (random::<f32>() - 0.5) * world_map_size.x;
        let random_y = (random::<f32>() - 0.5) * world_map_size.y;
        let valid_x = random_x < MAP_CONFIG.safe_zone_size && random_x > -MAP_CONFIG.safe_zone_size;
        let valid_y = random_y < MAP_CONFIG.safe_zone_size && random_y > -MAP_CONFIG.safe_zone_size;
        if valid_x && valid_y {
            continue;
        }
        let num = rand::thread_rng().gen_range(0..=3);

        commands.spawn((
            Sprite {
                custom_size: Some(BLOCK_CONFIG.size),
                image: asset_server.load(BLOCK_CONFIG.path[num]),
                ..default()
            },
            Transform::from_xyz(random_x, random_y, 1.0),
            Block {},
        ));
    }
}

pub fn update_block(
    mut _commands: Commands,
    _window_query: Query<&Window, With<PrimaryWindow>>,
    mut block_query: Query<(&mut Transform, &Block), Without<AttachedBlock>>,
    time: Res<Time>,
) {
    let mut rotation_dir = 0.0;

    for (mut transform, _player) in &mut block_query {
        rotation_dir += 0.1;

        transform.rotate_z(rotation_dir * BLOCK_CONFIG.rotation_speed * time.delta_secs());
    }
}
