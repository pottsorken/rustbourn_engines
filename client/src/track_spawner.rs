use crate::common::{Despawned, LastTrackPos, OpponentTrack, Player, Track, TRACK_CONFIG};
use bevy::prelude::Resource;
use bevy::prelude::*;
use spacetimedb_sdk::{
    credentials, DbContext, Error, Event, Identity, Status, Table, TableWithPrimaryKey,
};

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
                    timer: Timer::from_seconds(TRACK_CONFIG.fade_time, TimerMode::Once),
                    has_extended: false,
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
                    timer: Timer::from_seconds(TRACK_CONFIG.fade_time, TimerMode::Once),
                    has_extended: false,
                },
            ));

            last_track_pos.0 = current_pos;
        }
    }
}

pub fn track_lifetime_system(
    mut commands: Commands,
    time: Res<Time>,
    player_query: Query<&Player>,
    mut query: Query<(Entity, &mut Track), Without<Despawned>>,
    track_query: Query<Entity, With<Track>>,
    despawn_queue: Query<(Entity, &Despawned), With<Despawned>>,
) {
    let block_count = if let Ok(player) = player_query.get_single() {
        player.block_count
    } else {
        0
    };

    // Despawn from queue
    //for (entity, _despawned) in despawn_queue.iter() {
    //    //commands.entity(entity).remove::<Despawned>();
    //    commands.entity(entity).despawn();
    //}

    let lifetime_modifier = 2.0; // seconds per block

    for (entity, mut track) in query.iter_mut() {
        track.timer.tick(time.delta());

        if track.timer.finished() {
            if lifetime_modifier > 0.0 && !track.has_extended {
                // Extend timer just once
                let extended_duration =
                    track.timer.duration().as_secs_f32() + lifetime_modifier * block_count as f32;

                track
                    .timer
                    .set_duration(std::time::Duration::from_secs_f32(extended_duration));
                track.timer.reset();
                track.has_extended = true; // <-- Mark as extended
            } else {
                // fix for despawning non-existent entity
                // Add to despawn queue
                //commands.entity(entity).insert(Despawned);
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}
