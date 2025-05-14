use crate::block::SpawnedBlocks;
use crate::common::{CtxWrapper, Player, CAMERA_CONFIG};
use crate::grid::get_block_count;
use bevy::prelude::*;
use spacetimedb_sdk::DbContext;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Transform::from_xyz(0.0, 0.0, 999.9)));
}

pub fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    time: Res<Time>,
) {
    let follow_speed = 3.0;

    if let Ok(player_transform) = player_query.get_single() {
        for mut camera_transform in camera_query.iter_mut() {
            camera_transform.translation = camera_transform.translation.lerp(
                Vec3::new(
                    player_transform.translation.x,
                    player_transform.translation.y,
                    camera_transform.translation.z,
                ),
                follow_speed * time.delta_secs(),
            );
        }
    }
}

pub fn camera_zoom(
    player_query: Query<&Player>,
    mut camera_query: Query<&mut OrthographicProjection, With<Camera2d>>,
    time: Res<Time>,
    ctx_wrapper: Res<CtxWrapper>,
    mut spawned_blocks: ResMut<SpawnedBlocks>,
) {
    //Get local player's ID
    let player_block_count =
        get_block_count(ctx_wrapper.ctx.identity(), &ctx_wrapper, &spawned_blocks);

    if let Ok(player) = player_query.get_single() {
        let target_zoom = CAMERA_CONFIG.zoom_base
            + (player_block_count / CAMERA_CONFIG.zoom_after_blocks) as f32
                * CAMERA_CONFIG.zoom_per_blocks;

        for mut projection in camera_query.iter_mut() {
            projection.scale = projection.scale.lerp(target_zoom, time.delta_secs() * 1.5);
        }
    }
}
