use crate::common::{AttachedBlock, PlayerGrid};
use bevy::prelude::*;
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

//pub fn balance_owner_grid(
//    query
//) {
//
//}

pub fn check_grid_connectivity(
    mut commands: Commands,
    mut grid_query: Query<(Entity, &mut PlayerGrid), Changed<PlayerGrid>>,
) {
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
            }
        }
    }
}
