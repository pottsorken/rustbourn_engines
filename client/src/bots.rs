use crate::common::{
    AttachedBlock, Block, Bot, CtxWrapper, Obstacle, Opponent, PlayerGrid, BLOCK_CONFIG,
    BOT_CONFIG, GRID_CONFIG, OBSTACLE_CONFIG,
};
use crate::db_connection::{load_bots, update_bot_position};
use crate::grid::increment_grid_pos;
//use crate::module_bindings::Block as BlockDB;
use crate::block::SpawnedBlocks;
use crate::module_bindings::{BlockTableAccess, BotsTableAccess, OwnerType};
use crate::player_attach::check_collision;
use bevy::prelude::*;
use spacetimedb_sdk::{Identity, Table};
use std::collections::{HashMap, HashSet};

pub fn spawn_bots(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ctx_wrapper: Res<CtxWrapper>,
    query: Query<&Bot>,
) {
    if query.is_empty() {
        // Check if the query is empty, meaning no bots are in the world yet

        let bots = load_bots(&ctx_wrapper);
        //println!("[BOTS] Loaded {} bots", bots.len());

        for (x, y, bot_id) in bots {
            //println!("[BOTS] Spawning bot {} at ({}, {})", bot_id, x, y);

            let bot_entity = commands.spawn((
                Sprite {
                    custom_size: Some(BOT_CONFIG.size),
                    image: asset_server.load(BOT_CONFIG.path),
                    ..default()
                },
                Transform::from_xyz(x, y, 2.0),
                Bot {
                    id: bot_id,
                    spawn_point: Vec2 { x, y },
                    movement_speed: BOT_CONFIG.movement_speed,
                },
                PlayerGrid {
                    block_position: HashMap::new(),
                    grid_size: GRID_CONFIG.grid_size,
                    cell_size: GRID_CONFIG.cell_size,
                    next_free_pos: GRID_CONFIG.next_free_pos,
                    capacity: 5,
                    load: GRID_CONFIG.load,
                },
            ));
        }
    }
}

// bots.rs
pub fn render_bots_from_db(
    mut query: Query<(&mut Transform, &Bot), Without<Obstacle>>,
    ctx_wrapper: Res<CtxWrapper>,
    obstacle_query: Query<&Transform, With<Obstacle>>,
    time: Res<Time>, // Time resource for movement speed calculation
) {
    for (mut transform, bot) in query.iter_mut() {
        if let Some(server_bot) = ctx_wrapper.ctx.db.bots().id().find(&bot.id) {
            let server_dir = server_bot.movement_dir;
            let bevy_dir = Vec3::new(server_dir.x, server_dir.y, server_dir.z);

            let server_rotation = server_bot.position.rotation;

            transform.rotation = Quat::from_rotation_z(server_rotation);
            transform.translation = Vec3::new(
                server_bot.position.coordinates.x,
                server_bot.position.coordinates.y,
                transform.translation.z,
            );

            let mut movement_dir = transform.rotation * bevy_dir;
            let mut new_pos = transform.translation
                + movement_dir * BOT_CONFIG.movement_speed * time.delta_secs();

            let mut rotation_dir = server_bot.rotation_dir;

            let front_direction = transform.rotation * Vec3::new(1.0, 0.0, 0.0);
            let front_pos = transform.translation + front_direction * BOT_CONFIG.size.x; // Adjust distance

            if !will_collide(front_pos.truncate(), &obstacle_query) {
                // If no collision, update the bot's position
                transform.translation = new_pos;
                //println!(
                //    "[BOT] {} at ({}, {}) and rotation: {}",
                //    bot.id,
                //    server_bot.position.coordinates.x,
                //    server_bot.position.coordinates.y,
                //    server_bot.position.rotation
                //);
            } else {
                // Try to look left and right
                let left_direction =
                    transform.rotation * Quat::from_rotation_z(0.7).mul_vec3(Vec3::X);
                let right_direction =
                    transform.rotation * Quat::from_rotation_z(-0.7).mul_vec3(Vec3::X);

                let left_pos = transform.translation + left_direction * BOT_CONFIG.size.x;
                let right_pos = transform.translation + right_direction * BOT_CONFIG.size.x;

                let left_clear = !will_collide(left_pos.truncate(), &obstacle_query);
                let right_clear = !will_collide(right_pos.truncate(), &obstacle_query);

                // Decide which direction to go
                if left_clear && !right_clear {
                    rotation_dir = 1.0;
                    rotation_dir = rotation_dir * BOT_CONFIG.rotation_speed * time.delta_secs();
                } else if right_clear && !left_clear {
                    rotation_dir = -1.0;
                    rotation_dir = rotation_dir * BOT_CONFIG.rotation_speed * time.delta_secs();
                } else if left_clear && right_clear {
                    rotation_dir = if rand::random::<bool>() { 1.0 } else { -1.0 };
                    rotation_dir = rotation_dir * BOT_CONFIG.rotation_speed * time.delta_secs();
                } else {
                    // Nowhere to go: turn around
                    rotation_dir = std::f32::consts::PI; // 180°
                }
                transform.rotate_z(rotation_dir);
            }
            update_bot_position(&ctx_wrapper, &transform, bot.id, rotation_dir);
        }
    }
}

pub fn spawn_bot_blocks(
    mut bots_query: Query<(Entity, &mut PlayerGrid), With<Bot>>,
    mut commands: Commands,
    ctx_wrapper: Res<CtxWrapper>,
    asset_server: Res<AssetServer>,
    mut spawned_blocks: ResMut<SpawnedBlocks>,
) {
    //println§!("spawn_bot_blocks run here --------------");
    for block in ctx_wrapper.ctx.db.block().iter() {
        let mut i = 0;
        for (bot_entity, mut bot_grid) in bots_query.iter_mut() {
            if !spawned_blocks.ids.contains(&block.id) {
                if let OwnerType::Bot(owner) = block.owner {
                    if owner == i {
                        println!("Spawning for bot: {}, blockid: {}", bot_entity, i);
                        commands.spawn((
                            Sprite {
                                custom_size: Some(BLOCK_CONFIG.size),
                                image: asset_server.load(BLOCK_CONFIG.path[1]),
                                ..default()
                            },
                            Transform::from_xyz(0., 0., 1.0),
                            Block {},
                            AttachedBlock {
                                grid_offset: (block.offset_x, block.offset_y), //bot_grid.next_free_pos,
                                player_entity: bot_entity,
                            },
                        ));
                        increment_grid_pos(&mut bot_grid);
                        spawned_blocks.ids.insert(block.id);
                    }
                }
            }
            i += 1;
        }
    }
}

/*
pub fn update_bots(
    mut query: Query<(&mut Transform, &Bot), Without<Obstacle>>, // Query for both Transform and Bot
    obstacle_query: Query<&Transform, With<Obstacle>>,
    ctx_wrapper: Res<CtxWrapper>,
    time: Res<Time>, // Time resource for movement speed calculation
) {
    //let bots = load_bots(&ctx_wrapper);

    for (mut transform, _bot) in query.iter_mut() {
        // Movement is based on the bot's rotation (direction)
        let movement_direction = transform.rotation * Vec3::new(1.0, 0.0, 0.0); // Move right initially (in the x direction)

        // Calculate new position based on movement direction
        let new_pos = transform.translation
            + movement_direction * BOT_CONFIG.movement_speed * time.delta_secs();
        let mut rotation_dir = 0.0;

        let front_direction = transform.rotation * Vec3::new(1.0, 0.0, 0.0);
        let front_pos = transform.translation + front_direction * BOT_CONFIG.size.x; // Adjust distance

        if !check_collision(
            front_pos.truncate(),
            &obstacle_query,
            BOT_CONFIG.size,
            OBSTACLE_CONFIG.size,
        ) {
            // If no collision, update the bot's position
            transform.translation = new_pos;
        } else {
            // println!("Hello");
            //println!("[BOT] {} collided at ({}, {})", _bot.id, transform.translation.x, transform.translation.y);
            //println!(" ");

            // Try to look left and right
            let left_direction = transform.rotation * Quat::from_rotation_z(0.7).mul_vec3(Vec3::X);
            let right_direction =
                transform.rotation * Quat::from_rotation_z(-0.7).mul_vec3(Vec3::X);

            let left_pos = transform.translation + left_direction * BOT_CONFIG.size.x;
            let right_pos = transform.translation + right_direction * BOT_CONFIG.size.x;

            let left_clear = !check_collision(
                left_pos.truncate(),
                &obstacle_query,
                BOT_CONFIG.size,
                OBSTACLE_CONFIG.size,
            );
            let right_clear = !check_collision(
                right_pos.truncate(),
                &obstacle_query,
                BOT_CONFIG.size,
                OBSTACLE_CONFIG.size,
            );

            // Decide which direction to go
            if left_clear && !right_clear {
                rotation_dir = 1.0;
            } else if right_clear && !left_clear {
                rotation_dir = -1.0;
            } else if left_clear && right_clear {
                rotation_dir = if rand::random::<bool>() { 1.0 } else { -1.0 };
            } else {
                // Nowhere to go: turn around
                rotation_dir = std::f32::consts::PI; // 180°
                transform.rotate_z(rotation_dir);
                return; // Skip below rotation
            }

            let smooth_angle = rotation_dir * BOT_CONFIG.rotation_speed * time.delta_secs();
            transform.rotate_z(smooth_angle);
        }
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
        update_bot_position(&ctx_wrapper, &transform, _bot.id, );
        println!("[BOT] {} collided at ({}, {})", _bot.id, transform.translation.x, transform.translation.y);
=======
        update_bot_position(&ctx_wrapper, &transform, _bot.id);
        // println!("[BOT] {} collided at ({}, {})", _bot.id, transform.translation.x, transform.translation.y);
>>>>>>> d016088 (Functioning title screen and somewhat functioning edit mode)
=======
        update_bot_position(&ctx_wrapper, &transform, _bot.id);
        // println!("[BOT] {} collided at ({}, {})", _bot.id, transform.translation.x, transform.translation.y);
>>>>>>> 9c97630ad3e76b8ea67c27e80562b77f0e8f88c7


    }
}*/

pub fn will_collide(
    new_pos: bevy::prelude::Vec2,
    obstacles: &Query<&Transform, With<Obstacle>>,
) -> bool {
    let player_radius = BOT_CONFIG.size.x.min(BOT_CONFIG.size.y) / 2.0;
    let obstacle_radius = OBSTACLE_CONFIG.size.x.min(OBSTACLE_CONFIG.size.y) / 2.0;
    let collision_distance = player_radius + obstacle_radius;

    obstacles
        .iter()
        .any(|transform| new_pos.distance(transform.translation.truncate()) < collision_distance)
}
