use crate::module_bindings::*;
use crate::{
    block::SpawnedBlocks,
    common::{
        AttachedBlock, Block, CtxWrapper, Hook, HookCharge, HookRange, Obstacle, OpponentHook,
        Player, PlayerAttach, PlayerGrid, BLOCK_CONFIG, HOOK_CONFIG, OBSTACLE_CONFIG,HookAttach,
        PLAYER_CONFIG, HookHead, HookTimer,
    },
    db_connection::load_obstacles,
    grid::{increment_grid_pos, get_block_count, get_bot_block_count},
    opponent,
};
use bevy::prelude::{Vec2, Vec3};
use rand::Rng;
use bevy::{prelude::*, transform};
use spacetimedb_sdk::{
    credentials, DbContext, Error, Event, Identity, Status, Table, TableWithPrimaryKey,
};

pub fn setup_hook(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite {
            custom_size: Some(HOOK_CONFIG.hook_size),
            image: asset_server.load(HOOK_CONFIG.hook_path),
            anchor: bevy::sprite::Anchor::BottomCenter,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 8.0),
        Hook {
            hook_speed: HOOK_CONFIG.hook_speed,
            hook_max_range: HOOK_CONFIG.hook_max_range,
        },
        HookCharge {
            time_held: 0.0,
            target_length: 0.0,
        },
        PlayerAttach {
            offset: HOOK_CONFIG.player_attach_offset,
        },
    ));

    commands.spawn((
        Sprite {
            custom_size: Some(HOOK_CONFIG.hook_head_size),
            image: asset_server.load(HOOK_CONFIG.hook_head),
            anchor: bevy::sprite::Anchor::BottomCenter,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        HookHead,
       
        HookAttach  {
            offset: Vec2::new(3.0, -15.0),
        },
    ));

    commands.spawn((
        Sprite {
            custom_size: Some(Vec2::new(12.0, 26.0)),
            color: Color::srgba(1.0, 0.0, 0.0, 0.4),
            anchor: bevy::sprite::Anchor::BottomCenter,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 7.0),
        HookRange,
    ));
}

pub fn hook_controls(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: ParamSet<(
        Query<(&mut Sprite, &mut Transform, &Hook, &mut HookCharge)>,
        Query<(&mut Sprite, &mut Transform), With<HookRange>>,
        Query<(&mut Transform, &HookAttach), With<HookHead>>, 
    )>,
   

    //mut head_query: Query<(&mut Transform, &HookAttach), With<HookHead>>,
    time: Res<Time>,
    ctx: Res<CtxWrapper>,
) {
    let mut range_update_info: Option<(Vec3, Quat, f32)> = None;

    let mut rope_tip_position: Option<(Vec3, Quat, f32)> = None;//new shit


    for (mut sprite, mut transform, hook, mut charge) in query.p0().iter_mut() {
        if keyboard_input.pressed(KeyCode::Space) && charge.target_length == 0.0 {
            charge.time_held += time.delta_secs();
            let estimated_range =
                (charge.time_held / 2.0 * hook.hook_speed).min(hook.hook_max_range);

            let start_pos = transform.translation;
            let rotation = transform.rotation;

            range_update_info = Some((start_pos, rotation, estimated_range));
            

        }

        if keyboard_input.just_released(KeyCode::Space) {
            charge.target_length =
                (charge.time_held / 2.0 * hook.hook_speed).min(hook.hook_max_range);
            charge.time_held = 0.0;
        }

        let current_height = sprite.custom_size.unwrap().y;
        let mut new_height = current_height;

        if charge.target_length > 0.0 {
            let next_height = (current_height + HOOK_CONFIG.extend_speed * time.delta_secs())
                .min(charge.target_length);

            let rotation = transform.rotation;
            let offset = rotation * Vec3::Y * ((next_height - current_height) / 2.0);
            transform.translation += offset;

            sprite.custom_size = Some(Vec2::new(sprite.custom_size.unwrap().x, next_height));

            let old_size = sprite.custom_size.unwrap();
            ctx.ctx
                .reducers()
                .update_hook_movement(ctx.ctx.identity(), old_size.x, next_height)
                .unwrap();

            if (next_height - charge.target_length).abs() < 0.1 {
                charge.target_length = 0.0;
            }
        } else if current_height > 0.0 && !keyboard_input.pressed(KeyCode::Space) {
            let next_height =
                (current_height - HOOK_CONFIG.retract_speed * time.delta_secs()).max(0.0);

            let rotation = transform.rotation;
            let offset = rotation * Vec3::Y * ((current_height - next_height) / 2.0);
            transform.translation -= offset;

            sprite.custom_size = Some(Vec2::new(sprite.custom_size.unwrap().x, next_height));

            let old_size = sprite.custom_size.unwrap();
            ctx.ctx
                .reducers()
                .update_hook_movement(ctx.ctx.identity(), old_size.x, next_height)
                .unwrap();

                rope_tip_position = Some((
                    transform.translation,
                    transform.rotation,
                    sprite.custom_size.unwrap().y,
                ));
        }
        rope_tip_position = Some((
            transform.translation,
            transform.rotation,
            sprite.custom_size.unwrap().y,
        ));
    
    }
    if let Some((base_pos, rotation, rope_length)) = rope_tip_position {
        let rope_tip = base_pos + rotation * Vec3::Y * rope_length;
        for (mut head_transform, attach) in query.p2().iter_mut() {
            head_transform.translation = rope_tip + rotation * Vec3::from((attach.offset, 0.0));
            head_transform.rotation = rotation;
        }
    }


    for (mut range_sprite, mut range_transform) in query.p1().iter_mut() {
        if let Some((base_pos, rotation, length)) = range_update_info {
            range_transform.translation = base_pos;
            range_transform.rotation = rotation;
            range_sprite.custom_size = Some(Vec2::new(25.0, length));
        } else {
            range_sprite.custom_size = Some(Vec2::ZERO);
        }
    }
}

pub fn hook_collision_system(
    hook_query: Query<(&Transform, &Sprite), (With<Hook>, Without<Block>)>,
    mut block_query: Query<(Entity, &mut Transform, Option<&mut AttachedBlock>), (With<Block>)>,
    mut player_query: Query<
        (Entity, &Transform, &mut Player, &mut PlayerGrid),
        (Without<Hook>, Without<Block>),
    >,
    mut grid_query: Query<&mut PlayerGrid, Without<Player>>,
    attachable_blocks: Query<&PlayerAttach>,
    mut commands: Commands,
    ctx_wrapper: Res<CtxWrapper>,
    mut spawned_blocks: ResMut<SpawnedBlocks>,
    time: Res<Time>,
    mut hook_timer: ResMut<HookTimer>,
) {
    // Tick cooldown
    hook_timer.0.tick(time.delta());
    let (hook_transform, sprite) = if let Ok(val) = hook_query.get_single() {
        val
    } else {
        return;
    };

    let hook_tip = hook_transform.translation
        + hook_transform.rotation
            * bevy::prelude::Vec3::new(0.0, sprite.custom_size.unwrap().y, 0.0);

    if let Ok((player_entity, _player_transform, mut player, mut grid)) =
        player_query.get_single_mut()
    {
        for (block_entity, mut block_transform, mut attach_link_option) in block_query.iter_mut() {
            let block_radius = BLOCK_CONFIG.size.x.min(BLOCK_CONFIG.size.y) / 2.0;
            let hook_radius = 5.0; // Hook tip radius
            let collision_distance = block_radius + hook_radius;

            if hook_tip
                .truncate()
                .distance(block_transform.translation.truncate())
                < collision_distance
            {
                // Check if block already attached
                if grid.load < grid.capacity
                    && attachable_blocks.get(block_entity).is_err()
                    && player.block_count < PLAYER_CONFIG.max_block_count
                {
                    let nextpos = grid.find_next_free_pos().expect("No position found");

                    // NOTE: If block is attached or not. (Option<AttachedBlock>)
                    if let Some(mut attach_link) = attach_link_option {
                        // Remove grid_pos/block entity tuple from target grid
                        //if let Ok((_target_ent, _target_tran, _target_play, mut target_grid)) =
                        //    player_query.get(attach_link.player_entity)
                        //{
                        //    target_grid.block_position.remove(&attach_link.grid_offset);
                        //    println!(
                        //        "Stole block at: ({}, {}) from player",
                        //        attach_link.grid_offset.0, attach_link.grid_offset.1
                        //    );
                        //     } else

                        // 1 second cooldown
                        if !hook_timer.0.finished(){
                            return;
                        }

                        // Weighted combat mechanism
                        if let Some(block_id) = spawned_blocks.entities.get(&block_entity){
                            if let Some(block_from_db) = ctx_wrapper.ctx.db.block().id().find(&block_id){
                                // Get block owner
                                let block_owner = block_from_db.owner;
                                // If player
                                if let OwnerType::Player(owner_identity) = block_owner{
                                    // Get block counts
                                    let opp_block_count = get_block_count(owner_identity, &ctx_wrapper, &spawned_blocks);
                                    let player_block_count = get_block_count(ctx_wrapper.ctx.identity(), &ctx_wrapper, &spawned_blocks);
                                    if !weighted_combat(player_block_count, opp_block_count){
                                        continue; // Do not take block
                                    }
                                } else if let OwnerType::Bot(bot_id) = block_owner{ // If bot
                                    // Get block counts
                                    let bot_block_count = get_bot_block_count(bot_id, &ctx_wrapper, &spawned_blocks);
                                    let player_block_count = get_block_count(ctx_wrapper.ctx.identity(), &ctx_wrapper, &spawned_blocks);
                                    if !weighted_combat(player_block_count, bot_block_count){
                                        continue; // Do not take block
                                    }
                                }
                                hook_timer.0.reset();
                            }
                        }

                        if let Ok(mut target_grid) = grid_query.get_mut(attach_link.player_entity) {
                            target_grid.block_position.remove(&attach_link.grid_offset);
                            println!(
                                "Stole block at: ({}, {}) from bot or opp",
                                attach_link.grid_offset.0, attach_link.grid_offset.1
                            );
                        }

                        // Add grid_pos/block entity tuple to player grid
                        grid.block_position.insert(nextpos, block_entity);
                        println!("Add new block in gridpos: ({}, {})", nextpos.0, nextpos.1);

                        // Update block ownership locally
                        attach_link.player_entity = player_entity;
                        attach_link.grid_offset = nextpos;

                        // Update block ownership on server
                        let id_block_db = spawned_blocks
                            .entities
                            .get(&block_entity)
                            .expect("Failed lookup for block Entity->ServerID");
                        ctx_wrapper.ctx.reducers().update_block_owner(
                            id_block_db.clone(),
                            OwnerType::Player(ctx_wrapper.ctx.identity()),
                            nextpos.0,
                            nextpos.1,
                        ).unwrap();
                    } else {
                        commands.entity(block_entity).insert(AttachedBlock {
                            grid_offset: nextpos,
                            player_entity: player_entity,
                        });

                        // Update the server as well
                        let id_block_db = spawned_blocks
                            .entities
                            .get(&block_entity)
                            .expect("Failed lookup for block Entity->ServerID");

                        ctx_wrapper.ctx.reducers().update_block_owner(
                            id_block_db.clone(),
                            OwnerType::Player(ctx_wrapper.ctx.identity()),
                            nextpos.0,
                            nextpos.1,
                        ).unwrap();
                    }
                    grid.block_position.insert(nextpos, block_entity);
                    block_transform.translation.x += 200_000.;
                    println!(
                        "Attach at gridpos ({}, {})",
                        grid.next_free_pos.0, grid.next_free_pos.1
                    );
                    // increment grid pos
                    //increment_grid_pos(&mut grid);

                    player.block_count += 1;
                    ctx_wrapper.ctx.reducers().update_owner_grid(player.block_count, grid.next_free_pos.0, grid.next_free_pos.1).unwrap();
                }
            }
        }
    }
}

fn weighted_combat(
    player_block_count: i32,
    opps_block_count: i32,
) -> bool {
    
    let total_block_count = player_block_count + opps_block_count;
    if total_block_count == 0 {
        return true; // No blocks, player wins by default
    }

    let player_weight = (player_block_count + 1) as f32/ (total_block_count + 2) as f32;

    let mut rng = rand::rng();
    rng.random_range(0.0..1.0) // Generate a random number between 0 and 1
    < (player_weight + 1 as f32) // If the roll is less than the player's weight, they win
}

pub fn spawn_opponent_hook(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    existing_hooks_query: &Query<&OpponentHook>,
    opponent_id: &Identity,
    local_player_id: &Identity,
    x: f32,
    y: f32,
) {
    // Don't spawn hook for yourself
    if opponent_id == local_player_id {
        return;
    }

    // Don't spawn already existing hooks
    for hook in existing_hooks_query.iter() {
        if hook.id == *opponent_id {
            return; // Hook already exists
        }
    }

    commands.spawn((
        Sprite {
            custom_size: Some(HOOK_CONFIG.hook_size),
            image: asset_server.load(HOOK_CONFIG.hook_path),
            anchor: bevy::sprite::Anchor::BottomCenter,
            ..default()
        },
        Transform::from_xyz(x, y, 5.0),
        OpponentHook { id: *opponent_id },
    ));
}

pub fn update_opponent_hook(
    query: &mut Query<(&mut Sprite, &mut Transform, &OpponentHook), With<OpponentHook>>,
    id: &Identity,
    x: f32,
    y: f32,
    rotation: f32,
    width: f32,
    height: f32,
) {
    for (mut sprite, mut transform, hook) in query.iter_mut() {
        if hook.id == *id {
            transform.translation.x = x;
            transform.translation.y = y;
            transform.rotation = Quat::from_rotation_z(rotation).normalize();
            sprite.custom_size = Some(bevy::prelude::Vec2::new(width, height));
        }
    }
}

pub fn despawn_opponent_hooks(
    mut commands: Commands,
    ctx_wrapper: Res<CtxWrapper>,
    query: Query<(Entity, &OpponentHook)>,
) {
    // List all online players
    let online_players: Vec<Identity> = ctx_wrapper
        .ctx
        .db
        .player()
        .iter()
        .map(|player| player.identity)
        .collect();

    for (entity, hook) in query.iter() {
        if !online_players.contains(&hook.id) {
            commands.entity(entity).despawn();
        }
    }
}

pub fn handle_obstacle_hit(
    ctx_wrapper: Res<CtxWrapper>,
    hook_query: Query<(&Transform, &Sprite), With<Hook>>,
    obstacle_query: Query<(&Obstacle, &Transform)>,
) {
    let obstacle_radius = OBSTACLE_CONFIG.size.x.min(OBSTACLE_CONFIG.size.y) / 2.0;
    let hook_radius = 6.0;

    // Ensure hook_query and obstacle_query contain valid entities
    if hook_query.is_empty() || obstacle_query.is_empty() {
        return; // Skip if no hooks or obstacles exist
    }

    for (hook_transform, hook_sprite) in &hook_query {
        let hook_tip =
            hook_transform.translation + hook_transform.up() * (hook_sprite.custom_size.unwrap().y); // tip = base + height

        for (obstacle, obstacle_transform) in &obstacle_query {
            let obstacle_pos = obstacle_transform.translation.truncate();
            let obstacle_pos_3d = bevy::prelude::Vec3::new(obstacle_pos.x, obstacle_pos.y, 0.0);
            let distance = hook_tip.distance(obstacle_pos_3d);

            if distance < (hook_radius + obstacle_radius) {
                // Ask SpaceTimeDB to handle the damage
                let _ = ctx_wrapper.ctx.reducers.damage_obstacle(obstacle.id, 1);
            }
        }
    }
}

