use crate::{block::SpawnedBlocks, common::{AttachedBlock, Opponent, Player, PlayerGrid, CtxWrapper}, module_bindings::{update_block_owner, BlockTableAccess, OwnerType}, player};
use bevy::prelude::*;
use spacetimedb_sdk::DbContext;
use std::collections::{HashSet, VecDeque};

pub fn increment_grid_pos(grid: &mut PlayerGrid) {
    // increment grid pos
    grid.next_free_pos.0 += 1;
    if grid.next_free_pos == (0, 0) {
        grid.next_free_pos.0 += 1;
    }
    if grid.next_free_pos.0 > grid.grid_size.0 {
        grid.next_free_pos.0 = -grid.grid_size.0;
        grid.next_free_pos.1 -= 1;
    }

    grid.load += 1;
    //player.block_count += 1;
}

pub fn check_grid_connectivity(
    mut commands: Commands,
    mut grid_query: Query<(Entity, &mut PlayerGrid)>,
    mut spawned_blocks: ResMut<SpawnedBlocks>,
    ctx_wrapper: Res<CtxWrapper>,
) {
    // TODO: Does not disconnect local player blocks??? WHY?
    for (player_entity, mut player_grid) in grid_query.iter_mut() {
        let blocks_set: HashSet<(i32, i32)> = player_grid.block_position.keys().cloned().collect();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        // Check adjacent positions to the player (0,0)
        let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        for dir in &directions {
            let pos = (dir.0, dir.1);
            if blocks_set.contains(&pos) {
                visited.insert(pos);
                queue.push_back(pos);
            }
        }

        // BFS to find all connected blocks
        while let Some(current_pos) = queue.pop_front() {
            for dir in &directions {
                let next_pos = (current_pos.0 + dir.0, current_pos.1 + dir.1);
                if blocks_set.contains(&next_pos) && !visited.contains(&next_pos) {
                    visited.insert(next_pos);
                    queue.push_back(next_pos);
                }
            }
        }

        println!("----------");
        for pos in visited.clone() {
            println!("Visited block : ({}, {}) ---- ", pos.0, pos.1);
        }
        println!("----------");

        // Collect disconnected positions
        let disconnected_positions: Vec<_> = player_grid
            .block_position
            .keys()
            .filter(|pos| !visited.contains(pos))
            .cloned()
            .collect();

        // Remove disconnected blocks
        for pos in disconnected_positions {
            println!(
                "Disconnecting free blocks here at: ({}, {}) ---- ",
                pos.0, pos.1
            );
            if let Some(block_entity) = player_grid.block_position.remove(&pos) {
                commands.entity(block_entity).remove::<AttachedBlock>();
                // Optional: Add components to simulate falling, e.g., Gravity, Velocity
                // commands.entity(block_entity).insert(Gravity::default());
                //player_grid.load = player_grid.load.saturating_sub(1);
                
                // Update block ownership to none
                let server_block_id = spawned_blocks
                .entities
                .get(&block_entity)
                .expect("Failed to get block id");
            ctx_wrapper.ctx.reducers.update_block_owner(*server_block_id, OwnerType::None, pos.0, pos.1).unwrap();
            }
        } 
    }
}

// MAGIC FUNCTION :)
pub fn balance_player_grid(
    mut commands: Commands,
    mut player_query: Query<(&Player, Entity, &mut PlayerGrid), With<Player>>,
    mut block_query: Query<&mut AttachedBlock>,
    mut spawned_blocks: ResMut<SpawnedBlocks>,
    ctx_wrapper: Res<CtxWrapper>,
) {
    if let Ok((player, player_entity, mut grid)) = player_query.get_single_mut() {

        if let Some(next_pos) = grid.find_next_free_pos() {
            grid.next_free_pos = next_pos;
        }

        let mut to_detach: Vec<(i32, i32)> = Vec::new();
        let mut to_remove: Vec<(i32, i32)> = Vec::new();
        let player_identity = ctx_wrapper.ctx.identity();
        for (grid_pos, block_ent) in grid.block_position.iter_mut() {
            if let Some(block_id) = spawned_blocks.entities.get(block_ent) {
                if let Some(block_from_db) = ctx_wrapper.ctx.db.block().id().find(block_id) {
                    if let OwnerType::None = block_from_db.owner{
                        to_detach.push(*grid_pos);  
                    } else if OwnerType::Player(player_identity) != block_from_db.owner{
                        to_remove.push(*grid_pos);
                    }
                } else {
                    warn!("Block ID {:?} not fosund in DB", block_id);
                }
            } else {
                warn!("Block entity {:?} not found in spawned_blocks", block_ent);
            }
        }

        for grid_pos in to_detach {
            if let Some(block_ent) = grid.block_position.remove(&grid_pos) {
                commands.entity(block_ent).remove::<AttachedBlock>();
            }
        }

        for grid_pos in to_remove {
            grid.block_position.remove(&grid_pos);
        }

        let known_entities: std::collections::HashSet<_> =
            grid.block_position.values().copied().collect();

        for (block_entity, block_id) in spawned_blocks.entities.iter() {
            if known_entities.contains(block_entity) {
                continue;
            }

            if let Some(block_from_db) = ctx_wrapper.ctx.db.block().id().find(block_id) {
                if block_from_db.owner == OwnerType::Player(player_identity) {

                    if let Some(next_pos) = grid.find_next_free_pos() {
                        grid.block_position.insert(next_pos, *block_entity);
                        grid.load += 1;

                        // Update or insert AttachedBlock
                        if let Ok(mut attach_link) = block_query.get_mut(*block_entity) {
                            attach_link.player_entity = player_entity;
                            attach_link.grid_offset = next_pos;
                        } else {
                            commands.entity(*block_entity).insert(AttachedBlock {
                                grid_offset: next_pos,
                                player_entity,
                            });
                        }

                        increment_grid_pos(&mut grid);
                        println!(
                            "Balanced block {:?} to ({}, {}) for player",
                            block_id, next_pos.0, next_pos.1
                        );
                    }
                }
            }
        }

        if let Some(next_pos) = grid.find_next_free_pos() {
            grid.next_free_pos = next_pos;
        }
    }
}

// MAGIC FUNCTION NR 2 :)
pub fn balance_opponents_grid(
    mut commands: Commands,
    mut opp_query: Query<(&Opponent,Entity, &mut PlayerGrid)>,
    mut block_query: Query<&mut AttachedBlock>,
    mut spawned_blocks: ResMut<SpawnedBlocks>,
    ctx_wrapper: Res<CtxWrapper>,
){
    for (opp, opp_entity, mut grid) in opp_query.iter_mut(){
        let player_identity = opp.id;
        let mut to_remove: Vec<(i32, i32)> = Vec::new();
        for (grid_pos, block_ent) in grid.block_position.iter_mut() {
            if let Some(block_id) = spawned_blocks.entities.get(block_ent) {
                if let Some(block_from_db) = ctx_wrapper.ctx.db.block().id().find(block_id) {
                    if OwnerType::Player(player_identity) != block_from_db.owner{
                        to_remove.push(*grid_pos);  
                    }
                } else {
                    warn!("Block ID {:?} not found in DB", block_id);
                }
            }
        }

        for grid_pos in to_remove {
            grid.block_position.remove(&grid_pos);
        }

        // Second pass: Find all blocks owned by the opponent but not in their grid
        // Clone known block entities to avoid borrow conflict
        let known_entities: std::collections::HashSet<_> = grid.block_position.values().copied().collect();

        // Insert missing owned blocks into grid
        for (block_ent, block_id) in &spawned_blocks.entities {
            if known_entities.contains(block_ent) {
                continue; // Already in grid
            }

            if let Some(block_from_db) = ctx_wrapper.ctx.db.block().id().find(block_id) {
                if block_from_db.owner == OwnerType::Player(player_identity) {
                    if grid.load >= grid.capacity {
                        break;
                    }

                    if let Some(next_pos) = grid.find_next_free_pos() {
                        grid.block_position.insert(next_pos, *block_ent);
                        grid.load += 1;

                        if let Ok(mut attach_link) = block_query.get_mut(*block_ent) {
                            attach_link.player_entity = opp_entity;
                            attach_link.grid_offset = next_pos;
                        } else {
                            commands.entity(*block_ent).insert(AttachedBlock {
                                grid_offset: next_pos,
                                player_entity: opp_entity,
                            });
                        }

                        increment_grid_pos(&mut grid);
                        println!(
                            "Rebalanced: inserted block {:?} at ({}, {})",
                            block_id, next_pos.0, next_pos.1
                        );
                    }
                }
            }
        }

        if let Some(next_pos) = grid.find_next_free_pos() {
            grid.next_free_pos = next_pos;
        }


    }
}

impl PlayerGrid {
    pub fn find_next_free_pos(&self) -> Option<(i32, i32)> {
        println!(
            "Grid size: ({}, {}) --------------",
            -self.grid_size.0, self.grid_size.0
        );
        for y in 0..=self.grid_size.1 {
            for x in -self.grid_size.0..=self.grid_size.0 {
                let pos = (x, -y); // -y since blocks increment backwards

                // skip if center block
                if pos == (0, 0) {
                    continue;
                }

                println!("Checking if free for ({}, {})", pos.0, pos.1);
                // return position if it is free
                if !self.block_position.contains_key(&pos) {
                    return Some(pos);
                }
            }
        }
        None
    }
}
