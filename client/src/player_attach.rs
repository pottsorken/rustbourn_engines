use bevy::prelude::*;

use crate::block::SpawnedBlocks;
use crate::module_bindings::*;
use crate::common::{AttachedBlock, Bot, Hook, Player, PlayerAttach, PlayerGrid, PLAYER_CONFIG, CtxWrapper, Opponent, Block};
use spacetimedb_sdk::{
    credentials, DbContext, Error, Event, Identity, Status, Table, TableWithPrimaryKey,
};

/// Finds all players' and bots' transform and playergrid components, and performs a update to
/// block positions if their owner is one of the players or bots
pub fn attach_objects(
    player_query: Query<(Entity, &Transform, &PlayerGrid), (With<Player>, Without<AttachedBlock>)>,
    bot_query: Query<
        (Entity, &Transform, &PlayerGrid),
        (With<Bot>, (Without<Player>, Without<AttachedBlock>)),
    >,
    opponent_query: Query<
        (Entity, &Transform, &PlayerGrid),
        (With<Opponent>, Without<Player>, Without<Bot>),
    >,
    mut param_set: ParamSet<(
        Query<(&AttachedBlock, &mut Transform), (Without<Player>, Without<Bot>, Without<Opponent>)>,
    )>,
) {
    let mut block_query = param_set.p0();
    for (attach, mut transform) in block_query.iter_mut() {
        for (player_entity, player_transform, player_grid) in player_query.iter() {
            if attach.player_entity == player_entity {
                update_slave_pos(&player_transform, &player_grid, &mut transform, &attach);
            }
        }
        for (bot_entity, bot_transform, bot_grid) in bot_query.iter() {
            if attach.player_entity == bot_entity {
                update_slave_pos(&bot_transform, &bot_grid, &mut transform, &attach);
            }
        }
        for (opp_entity, opp_transform, opp_grid) in opponent_query.iter() {
            if attach.player_entity == opp_entity {
                update_slave_pos(&opp_transform, &opp_grid, &mut transform, &attach);
            }
        }
    }
}

/// Change block position based the player's core position, player's grid size and
/// the block grid offset found in 'slave_attach'.
fn update_slave_pos(
    owner_transform: &Transform,
    owner_grid: &PlayerGrid,
    slave_transform: &mut Transform,
    slave_attach: &AttachedBlock,
) {
    // Calculate the rotated offset
    let rotated_offset = owner_transform.rotation
        * bevy::prelude::Vec3::new(
            slave_attach.grid_offset.0 as f32 * owner_grid.cell_size,
            slave_attach.grid_offset.1 as f32 * owner_grid.cell_size,
            5.0,
        );

    // Update position and rotation
    slave_transform.translation = owner_transform.translation + rotated_offset;
    slave_transform.rotation = owner_transform.rotation;
}

pub fn update_block_owner(
    mut block_query: Query<(Entity, &mut AttachedBlock)>,
    mut opponent_query: Query<
        (Entity, &Opponent, &mut PlayerGrid),
        (Without<AttachedBlock>, Without<Player>),
    >,
    mut player_query: Query<(Entity, &mut PlayerGrid), (With<Player>, Without<AttachedBlock>)>,
    ctx_wrapper: Res<CtxWrapper>,
    mut spawned_blocks: ResMut<SpawnedBlocks>,
) {
    for (block_entity, mut attach_link) in block_query.iter_mut() {
        let Some(server_block_id) = spawned_blocks.entities.get(&block_entity) else {
            warn!("Block entity {:?} not found in spawned_blocks", block_entity);
            continue;
        };

        let Some(block_from_db) = ctx_wrapper
            .ctx
            .db
            .block()
            .id()
            .find(server_block_id)
        else {
            warn!("Block with ID {:?} not found in DB", server_block_id);
            continue;
        };

        let owner_identity_type = block_from_db.owner;
        let block_pos = (block_from_db.offset_x, block_from_db.offset_y);

        if let OwnerType::Player(owner_identity) = owner_identity_type {
            let owner_info = if owner_identity == ctx_wrapper.ctx.identity() {
                player_query.get_single_mut().ok()
            } else {
                opponent_query
                    .iter_mut()
                    .find(|(_, opponent, _)| opponent.id == owner_identity)
                    .map(|(e, _, g)| (e, g))
            };

            let Some((owner_entity, mut grid)) = owner_info else {
                warn!(
                    "No matching owner found for block {:?} with owner ID {:?}",
                    server_block_id, owner_identity
                );
                continue;
            };

            attach_link.player_entity = owner_entity;

            if !grid.block_position.contains_key(&block_pos) {
                grid.block_position.insert(block_pos, block_entity);
            }

            attach_link.grid_offset = block_pos;
        }
    }
}

pub fn attach_items(
    player_query: Query<(&Transform, &PlayerGrid), With<Player>>,
    mut items_query: Query<(&PlayerAttach, &mut Transform), Without<Player>>,
    ctx_wrapper: Res<CtxWrapper>,
) {
    //if let Ok(player_transform) = player_query.get_single() {
    for (player_transform, player_grid) in player_query.iter() {
        for (attach, mut transform) in items_query.iter_mut() {
            // Calculate the rotated offset

            let rotated_offset = player_transform.rotation
                * bevy::prelude::Vec3::new(attach.offset.x as f32, attach.offset.y as f32, 5.0);

            // Update position and rotation
            transform.translation = player_transform.translation + rotated_offset;
            transform.rotation = player_transform.rotation;

            let x = transform.translation.x;
            let y = transform.translation.y;
            let rotation = transform.rotation.to_euler(EulerRot::XYZ).2;

            ctx_wrapper
                .ctx
                .reducers()
                .update_hook_position(
                    ctx_wrapper.ctx.identity(),
                    vec_2_type::Vec2 { x: x, y: y },
                    rotation,
                )
                .unwrap();
        }
    }
}

pub fn check_collision<T: Component>(
    new_pos: bevy::prelude::Vec2,
    targets: &Query<&Transform, With<T>>,
    origin_size: bevy::prelude::Vec2,
    target_size: bevy::prelude::Vec2,
) -> bool {
    let origin_radius = origin_size.x.min(origin_size.y) / 2.0;
    let target_radius = target_size.x.min(target_size.y) / 2.0;
    let collision_distance = origin_radius + target_radius;

    targets
        .iter()
        .any(|transform| new_pos.distance(transform.translation.truncate()) < collision_distance)
}
