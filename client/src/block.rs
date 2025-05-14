use crate::common::CtxWrapper;
use crate::common::{AttachedBlock, Block, BLOCK_CONFIG, MAP_CONFIG};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::random;
use rand::Rng;

use crate::module_bindings::*;
use spacetimedb_sdk::{credentials, DbContext, Error, Identity, Table};
use std::collections::{HashMap, HashSet};

#[derive(Resource, Default)]
pub struct SpawnedBlocks {
    pub ids: HashSet<u64>,
    pub entities: HashMap<Entity, u64>,
    pub identities: HashMap<u64, Entity>,
}

// pub fn setup_block(
//     mut commands: Commands,
//     window_query: Query<&Window, With<PrimaryWindow>>,
//     asset_server: Res<AssetServer>,
// ) {
//     commands.insert_resource(SpawnedBlocks {
//         ids: HashSet::new(),
//         entities: HashMap::new(),
//     });

//     //let _window = window_query.get_single().unwrap();
//     //
//     //let world_map_size = Vec2::new(
//     //    MAP_CONFIG.map_size.x as f32 * MAP_CONFIG.tile_size.x / 2.0,
//     //    MAP_CONFIG.map_size.y as f32 * MAP_CONFIG.tile_size.y / 2.0,
//     //);
//     //
//     //for _ in 0..BLOCK_CONFIG.count {
//     //    let random_x = (random::<f32>() - 0.5) * world_map_size.x;
//     //    let random_y = (random::<f32>() - 0.5) * world_map_size.y;
//     //    let valid_x = random_x < MAP_CONFIG.safe_zone_size && random_x > -MAP_CONFIG.safe_zone_size;
//     //    let valid_y = random_y < MAP_CONFIG.safe_zone_size && random_y > -MAP_CONFIG.safe_zone_size;
//     //    if valid_x && valid_y {
//     //        continue;
//     //    }
//     //
//     //    commands.spawn((
//     //        Sprite {
//     //            custom_size: Some(BLOCK_CONFIG.size),
//     //            image: asset_server.load(BLOCK_CONFIG.path),
//     //            ..default()
//     //        },
//     //        Transform::from_xyz(random_x, random_y, 1.0),
//     //        Block {},
//     //    ));
//     //}
// }

pub fn update_block(
    mut _commands: Commands,
    _window_query: Query<&Window, With<PrimaryWindow>>,
    mut block_query: Query<(&mut Transform, &Block), Without<AttachedBlock>>,
    time: Res<Time>,
) {
    let mut rotation_dir = 0.0;

    for (mut transform, _player) in &mut block_query {
        rotation_dir += 0.1;

        //transform.rotate_z(rotation_dir * BLOCK_CONFIG.rotation_speed * time.delta_secs());
    }
}

pub fn despawn_opponents_blocks(
    mut commands: Commands,
    ctx_wrapper: Res<CtxWrapper>,
    mut spawned_blocks: ResMut<SpawnedBlocks>,
    existing_entities: Query<Entity>,
){
    let online_players: Vec<Identity> = ctx_wrapper
        .ctx
        .db
        .player()
        .iter()
        .map(|player| player.identity)
        .collect();

    // Store block IDs to be removed after the loop
    let mut blocks_to_remove = Vec::new();

    // Iterate over the block IDs
    for block_id in spawned_blocks.ids.clone() {
        if let Some(block) = ctx_wrapper.ctx.db.block().id().find(&block_id) {
            if let OwnerType::Player(owner_identity) = block.owner {
                if online_players.contains(&owner_identity) {
                    // Ensure the entity exists in the world before attempting to despawn
                    if let Some(&block_entity) = spawned_blocks.identities.get(&block_id) {
                        // Check if the entity exists
                        if existing_entities.get(block_entity).is_ok() {
                            // Mark block for removal
                            blocks_to_remove.push(block_id);
                            commands.entity(block_entity).despawn();
                        }
                    }
                }
            }
        }
    }

    // Remove the IDs of the blocks that were despawned
    for block_id in blocks_to_remove {
        spawned_blocks.ids.remove(&block_id);
    }
}
