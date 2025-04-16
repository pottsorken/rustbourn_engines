use crate::common::{Player, Track, OpponentTrack, LastTrackPos, TRACK_CONFIG};
use bevy::prelude::*;
use spacetimedb_sdk::{
    credentials, DbContext, Error, Event, Status, Table, TableWithPrimaryKey, Identity
};
use bevy::prelude::Resource;

#[derive(Resource, Clone)]
pub struct LocalPlayerId(pub Identity);

pub fn spawn_tracks_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut query: Query<(&Transform, &mut LastTrackPos), With<Player>>,
) {
    //let track_texture = asset_server.load(TRACK_CONFIG.path);

    for (transform, mut last_track_pos) in query.iter_mut() {
        let forward = transform.rotation * Vec3::Y;
        let right = transform.rotation * Vec3::X;

        let left_offset = transform.translation - right * (TRACK_CONFIG.track_spacing / 2.0);
        let right_offset = transform.translation + right * (TRACK_CONFIG.track_spacing / 2.0);

        let current_pos = transform.translation.truncate();

        if current_pos.distance(last_track_pos.0) >= TRACK_CONFIG.spawn_distance {
            // Spawn a track sprite
            commands.spawn((
                Sprite {
                    custom_size: Some(TRACK_CONFIG.size),
                    image: asset_server.load(TRACK_CONFIG.path),
                    ..default()
                },
                Transform {
                    translation: Vec3::new(left_offset.x, left_offset.y, 1.0),
                    rotation: transform.rotation,
                    scale: Vec3::splat(1.0),
                },
                Track { 
                    timer: 
                    Timer::from_seconds(
                        TRACK_CONFIG.fade_time, 
                        TimerMode::Once
                    )
                },
            ));

            commands.spawn((
                Sprite {
                    custom_size: Some(TRACK_CONFIG.size),
                    image: asset_server.load(TRACK_CONFIG.path),
                    ..default()
                },
                Transform {
                    translation: Vec3::new(right_offset.x, right_offset.y, 1.0),
                    rotation: transform.rotation,
                    scale: Vec3::splat(1.0),
                },
                Track { 
                    timer: 
                    Timer::from_seconds(
                        TRACK_CONFIG.fade_time, 
                        TimerMode::Once
                    ),
                },
            ));

            last_track_pos.0 = current_pos;
        }
    }
}

pub fn track_lifetime_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Track)>,
) {
    for (entity, mut track) in query.iter_mut() {
        track.timer.tick(time.delta());

        if track.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}
