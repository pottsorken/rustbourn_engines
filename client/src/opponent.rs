use bevy::prelude::*;

use crate::{
    block::SpawnedBlocks, common::AttachedBlock, common::Block as BevyBlock, common::Opponent,
    common::PlayerGrid, common::BLOCK_CONFIG, common::GRID_CONFIG, db_connection::CtxWrapper,
    grid::increment_grid_pos, module_bindings::*,
};
use spacetimedb_sdk::{Identity, Table};
use std::collections::HashMap;

pub fn spawn_opponent(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    query: &Query<(&mut Transform, &Opponent)>,
    id: &Identity,
    x: f32,
    y: f32,
    rotation: f32,
    local_player_id: &Identity,
) {
    if id == local_player_id {
        //println!("Skipping spawn for local player {:?}", id);
        return;
    }

    for (_transf, opp) in &mut query.iter() {
        if opp.id == *id {
            return;
        }
    }
    commands.spawn((
        Sprite {
            custom_size: Some(bevy::prelude::Vec2::new(80.0, 80.0)), // Square size 100x100 pixels
            image: asset_server.load("sprites/top-view/robot_3Dred.png"),
            ..default()
        },
        //TextureAtlas {
        //    layout: asset_server.load("sprites/top-view/robot_3Dblue.png"),
        //    index: 0,
        //}, -- NOTE: If asset-chart is ever used
        Transform::from_xyz(x, y, 2.0)
            .with_scale(bevy::prelude::Vec3::new(1.0, 1.0, 1.0))
            .with_rotation(Quat::from_rotation_z(rotation)),
        Opponent {
            movement_speed: 300.0,                  // meters per second
            rotation_speed: f32::to_radians(180.0), // degrees per second
            id: *id,
        },
        PlayerGrid {
            block_position: HashMap::new(),
            grid_size: GRID_CONFIG.grid_size,
            cell_size: GRID_CONFIG.cell_size,
            next_free_pos: GRID_CONFIG.next_free_pos,
            capacity: GRID_CONFIG.capacity,
            load: GRID_CONFIG.load,
        },
    ));
}

pub fn update_opponent(
    query: &mut Query<(&mut Transform, &Opponent)>,
    id: &Identity,
    x: f32,
    y: f32,
    rotation: f32,
) {
    for (mut transform, opponent) in query.iter_mut() {
        if opponent.id == *id {
            transform.translation.x = x;
            transform.translation.y = y;
            transform.rotation = Quat::from_rotation_z(rotation).normalize();
            //let temp_id = opponent.identity.to_u256() % 10_000;
            //if temp_id == 9573 {
            //    print!("{}: {}   ", temp_id, player.position.rotation);
            //}
            //println!("Updated opponent {:?} to position ({}, {})", id, x, y);
        }
    }
}

pub fn despawn_opponents(
    mut commands: Commands,
    ctx_wrapper: Res<CtxWrapper>,
    query: Query<(Entity, &Opponent)>,
) {
    // List all online players
    let online_players: Vec<Identity> = ctx_wrapper
        .ctx
        .db
        .player()
        .iter()
        .map(|player| player.identity)
        .collect();

    for (entity, opponent) in query.iter() {
        if !online_players.contains(&opponent.id) {
            commands.entity(entity).despawn();
        }
    }
}

pub fn setup_blocks_opponent(
    mut commands: Commands,
    mut opponent_query: Query<(Entity, &mut PlayerGrid, &Opponent)>, // Will yield empty since
    // opps do not have playergrid
    asset_server: Res<AssetServer>,
    ctx: Res<CtxWrapper>,
    mut spawned_blocks: ResMut<SpawnedBlocks>,
) {
    for (opponent_entity, mut grid, opponent) in opponent_query.iter_mut() {
        for block in ctx.ctx.db.block().iter() {
            if !spawned_blocks.ids.contains(&block.id) {
                if let OwnerType::Player(owner) = block.owner {
                    if owner == opponent.id {
                        println!("Spawning block for opponent: {}", &opponent.id);
                        //println!("Spawning for Player: {}", player_entity,);
                        let block_entity = commands.spawn((
                            Sprite {
                                custom_size: Some(BLOCK_CONFIG.size),
                                image: asset_server.load(BLOCK_CONFIG.path),
                                ..default()
                            },
                            Transform::from_xyz(0., 0., 1.0),
                            BevyBlock {},
                            AttachedBlock {
                                grid_offset: (block.offset_x, block.offset_y), //bot_grid.next_free_pos,
                                player_entity: opponent_entity,
                            },
                        ));
                        // Increase next free position when loading from server
                        increment_grid_pos(&mut grid);
                        spawned_blocks.ids.insert(block.id);
                        spawned_blocks.entities.insert(block_entity.id(), block.id);
                    }
                }
            }
        }
    }
}
