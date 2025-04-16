use spacetimedb::{
    reducer, rand,
    spacetimedb_lib::{db, identity},
    table, DbContext, Identity, Local, ReducerContext, SpacetimeType, Table, Timestamp,
};

const N_BOTS: u64 = 30;
const N_OBSTACLES: u64 = 200;

use noise::{NoiseFn, Perlin};

/// Player component data
use glam::{Vec3, Quat};

const FIXED_DELTA: f32 = 1.0 / 30.0; // Fixed delta for 3cd ..0 FPS simulation
const BOT_SIZE: f32 = 80.0; // Size of bot
const BOT_MOVE: f32 = 110.0;

// Player data
#[spacetimedb::table(name = player, public)]
pub struct Player {
    #[primary_key]
    identity: Identity,
    name: String,
    // Position stored in custom bevy transform struct
    position: BevyTransform,
    online: bool,
    hook: Hook,
    track: Track,
    grid: Grid,
}

#[derive(Debug, SpacetimeType)]
pub struct Grid {
    load: i32,
    next_free_x: i32,
    next_free_y: i32,
}

/// Obstacle component data
#[spacetimedb::table(name = obstacle, public)]
pub struct Obstacle {
    #[primary_key]
    id: u64,
    position: Vec2,
    hp: u32,
}

/// Bot component data
#[spacetimedb::table(name = bots, public)]
pub struct Bot {
    #[primary_key]
    id: u64,
    position: BevyTransform,
    // Rotation direction
    alive: bool, // instead of online we have alive that checks if that specific bot is alive
    movement_dir: Vec3_space,
    rotation_dir: f32,
}

#[spacetimedb::table(name = track, public)]
pub struct Track {
    #[primary_key]
    owner_identity: Identity,
    position: BevyTransform,
    rotation: f32,
    width: f32,
    height: f32,
    id: u64,
}

/// Hook component data
#[derive(Debug, SpacetimeType)]
pub struct Hook {
    position: Vec2,
    rotation: f32,
    width: f32,
    // Dynamicallt adjusted when extended
    height: f32,
}
#[spacetimedb::table(name = block, public)]
pub struct Block {
    offset_x: i32,
    offset_y: i32,
    #[primary_key]
    id: u64,
    owner: OwnerType,
}

#[derive(SpacetimeType, Clone, Debug, PartialEq)]
pub enum OwnerType {
    Bot(u64),
    Player(Identity),
    None,
}

/// Custom struct containing bevy transform data
#[derive(Debug, SpacetimeType)]
pub struct BevyTransform {
    coordinates: Vec2,
    rotation: f32,
    scale: Vec2,
}

/// Custom f32 2D vector containing xy-coordinates
#[derive(Debug, SpacetimeType)]
pub struct Vec2 {
    x: f32,
    y: f32,
}

/// Custom f32 3D vector containing xyz-coordinates
#[derive(Debug, SpacetimeType)]
pub struct Vec3_space {
    x: f32,
    y: f32,
    z: f32,
}

#[spacetimedb::table(name = leaderboard, public)]
pub struct Leaderboard {
    #[primary_key]
    id: u64,
    top_players: Vec<Identity>,  // Array of the top three players
}


/// Reducer for decreasing a ("id") specific obstacle's HP by "damage" points.
/// Client invokes this reducer in "handle_obstacle_hit" function when dealing damage to an obstacle with their hook.

#[spacetimedb::reducer]
/// Clients invoke this reducer to set their user names.
pub fn set_name(ctx: &ReducerContext, name: String) -> Result<(), String> {
    let name = validate_name(name)?;
    if let Some(user) = ctx.db.player().identity().find(ctx.sender) {
        ctx.db.player().identity().update(Player { name, ..user });
        Ok(())
    } else {
        Err("Cannot set name for unknown user".to_string())
    }
}

/// Takes a name and checks if it's acceptable as a user's name.
fn validate_name(name: String) -> Result<String, String> {
    if name.is_empty() {
        Err("Names must not be empty".to_string())
    } else {
        Ok(name)
    }
}


//Reducer for damaging obstacle
#[spacetimedb::reducer]
pub fn damage_obstacle(ctx: &ReducerContext, id: u64, damage: u32) -> Result<(), String> {
    if let Some(mut obstacle) = ctx.db.obstacle().id().find(id) {
        // Subtract (saturating) obstacle's HP with a specific amount of "damage".
        obstacle.hp = obstacle.hp.saturating_sub(damage);
        // Update column in "obstacle" table.
        ctx.db.obstacle().id().update(obstacle);
        Ok(())
    } else {
        // Reaches only when client tries to deal damage to an obstacle with an unknown ID.
        Err("Obstacle does not exist!".to_string())
    }
}

/// Reducer for updating a ("identity") specific hook "position" and "rotation" on the map.
/// Client invokes this reducer in "attach_items" function when reattaching hook to player.
#[spacetimedb::reducer]
pub fn update_hook_position(
    ctx: &ReducerContext,
    identity: Identity,
    position: Vec2,
    rotation: f32,
) -> Result<(), String> {
    // Find requested player by Identity.
    if let Some(mut player) = ctx.db.player().identity().find(identity) {
        // Update player hook position and rotation.
        player.hook.position = position;
        player.hook.rotation = rotation;
        // Update column in "player" table.
        ctx.db.player().identity().update(player);
        Ok(())
    } else {
        // Reaches only when requesting a player's hook with an unknown identity.
        Err("Player not found".to_string())
    }
}

#[spacetimedb::reducer]
pub fn update_tracks_system(
    ctx: &ReducerContext,
    owner_identity: Identity,
    position: BevyTransform,
    rotation: f32,
    width: f32,
    height: f32,
    id: u64,
) -> Result<(), String> {
    if let Some(_player) = ctx.db.player().identity().find(ctx.sender) {
        ctx.db.track().insert(Track {
            owner_identity,
            position,
            rotation,
            width,
            height,
            id,
        });
        Ok(())
    } else {
        Err("Player not found".to_string())
    }
}

/// Reducer for updating a ("identity") specific hook's movement in extension and retraction through it's "width" and "height".
/// Client invokes this reducer in "hook_controls" function when player extends and retracs hook.
#[spacetimedb::reducer]
pub fn update_hook_movement(
    ctx: &ReducerContext,
    identity: Identity,
    width: f32,
    height: f32,
) -> Result<(), String> {
    // Find requested player by Identity.
    if let Some(mut player) = ctx.db.player().identity().find(identity) {
        // Update player hook width and height.
        player.hook.width = width;
        player.hook.height = height;
        // Update column in "player" table.
        ctx.db.player().identity().update(player);
        Ok(())
    } else {
        // Reaches only when requesting a player's hook with an unknown identity.
        Err("Player not found".to_string())
    }
}

/// Reducer for updating player position by sending the entity data contained in a transform. All data is sent in a custom "BevyTransform" struct.
/// Client invokes this reducer in "player_movement" function when updating the position of the player sprite.
#[spacetimedb::reducer]
pub fn update_player_position(
    ctx: &ReducerContext,
    bevy_transform: BevyTransform,
) -> Result<(), String> {
    // Find requested player by Identity.
    if let Some(_player) = ctx.db.player().identity().find(ctx.sender) {
        // Update modified column in "player" table.
        ctx.db.player().identity().update(Player {
            // Update player position
            position: bevy_transform,
            .._player
        });
        Ok(())
    } else {
        // Reaches only when requesting a player with an unknown identity.
        Err("Player not found".to_string())
    }
}

#[spacetimedb::reducer]
pub fn update_owner_grid(
    ctx: &ReducerContext,
    load: i32,
    next_free_x: i32,
    next_free_y: i32,
) -> Result<(), String> {
    if let Some(_player) = ctx.db.player().identity().find(ctx.sender) {
        ctx.db.player().identity().update(Player {
            grid: Grid {
                load,
                next_free_x,
                next_free_y,
            },
            .._player
        });
        Ok(())
    } else {
        Err("Player not found".to_string())
    }
}

#[spacetimedb::reducer]
pub fn decrease_grid_load(
    ctx: &ReducerContext,
    identity: Identity,
    load: i32,
) -> Result<(), String> {
    if let Some(mut player) = ctx.db.player().identity().find(identity) {
        player.grid.load = load;
        ctx.db.player().identity().update(player);
        Ok(())
    } else {
        Err("Player not found".to_string())
    }
}

/// Reducer for updating a ("bot_id") specific bot position by sending the entity data contained in a transform. All data is sent in a custom "BevyTransform" struct, except "new_rotate_dir".
/// Client invokes this reducer in "render_bots_from_db" function when updating the position of the bot sprite.
// Reducer for updating bot position
#[spacetimedb::reducer]
pub fn update_bot_position(
    ctx: &ReducerContext,
    //bevy_transform: BevyTransform,
    bot_id: u64,
    //new_rotate_dir: f32,
) -> Result<(), String> {
    log::info!(
        "Code reaches this point! --------------",
    );

    //let obstacles = ctx.db.obstacle().iter().collect::<Vec<_>>();
    //let players = ctx.db.player().iter().collect::<Vec<_>>();

    //let max_check_distance = BOT_SIZE * 2.5;


    if let Some(mut _bot) = ctx.db.bots().iter().find(|b| b.id == bot_id) {
        //_bot.position = bevy_transform;
       // _bot.rotation_dir = new_rotate_dir;

        let bevy_dir = Vec3::new(_bot.movement_dir.x, _bot.movement_dir.y, _bot.movement_dir.z);
        

        let mut transform_translation = Vec3::new(_bot.position.coordinates.x, _bot.position.coordinates.y, 0.0);

        let rotation_quat = Quat::from_rotation_z(_bot.position.rotation);

        let mut movement_dir = rotation_quat * bevy_dir;
        let front_dir = rotation_quat * Vec3::new(1.8, 0.0, 0.0);
        //let left_dir = rotation_quat * Quat::from_rotation_z(f32::to_radians(45.0)) * Vec3::X;
        //let right_dir = rotation_quat * Quat::from_rotation_z(f32::to_radians(-45.0)) * Vec3::X;
        
        let left_dir = rotation_quat * Vec3::new(2.0, 1.0, 0.0);
        let left_dir2: Vec3 = rotation_quat * Vec3::new(1.0, 1.5, 0.0);



        let right_dir = rotation_quat * Vec3::new(2.0, -1.0, 0.0);
        let right_dir2: Vec3 = rotation_quat * Vec3::new(1.0, -1.5, 0.0);

        let mut rotation_dir = _bot.rotation_dir;

        let front_pos = transform_translation + front_dir * BOT_SIZE; // Adjust distance
        let left_pos = transform_translation + left_dir * BOT_SIZE; // Adjust distance
        let left_pos2 = transform_translation + left_dir2 * BOT_SIZE; // Adjust distance
        let right_pos = transform_translation + right_dir * BOT_SIZE; // Adjust distance
        let right_pos2 = transform_translation + right_dir2 * BOT_SIZE; // Adjust distance

        let check_points = vec![front_pos, left_pos, right_pos ,left_pos2, right_pos2];

        // Single pass through all entities
        let collision_results = check_collisions_at_points(
            &check_points,
            ctx.db.obstacle().iter(),
            ctx.db.player().iter(),
            BOT_SIZE * 1.5
        );

        let front_clear = collision_results[0];
        let left_clear = collision_results[1];
        let right_clear = collision_results[2];
        let left_clear2 = collision_results[3];
        let right_clear2 = collision_results[4];

        let mut new_pos = transform_translation + movement_dir * BOT_MOVE * FIXED_DELTA;

        if !left_clear.0 && !right_clear.0 && !front_clear.0 && 
           !left_clear.1 && !right_clear.1 && !front_clear.1 {
            _bot.position.coordinates.x = new_pos.x;
            _bot.position.coordinates.y = new_pos.y;
        }
        else {

            let rotation_speed = f32::to_radians(90.0);
            // Player is on the left and NOT on the right → rotate right to chas
            // Check left and right vision

            if left_clear.0 && !right_clear.0 || left_clear2.0 && !right_clear2.0{
                // If left is clear but right is blocked, rotate right
                //_bot.position.rotation += rotation_speed * FIXED_DELTA; // Rotate left (positive direction)
                _bot.position.rotation += rotation_speed * FIXED_DELTA; // Tracking to right

            } 
            // Player is on the right and NOT on the left → rotate left to chase
            else if right_clear.0 && !left_clear.0 || right_clear2.0 && !left_clear2.0{
                // If right is clear but left is blocked, rotate left
                //_bot.position.rotation -= rotation_speed * FIXED_DELTA; // Rotate right (negative direction)
                _bot.position.rotation -= rotation_speed * FIXED_DELTA; // tracking to left

            } 
            
            else if (right_clear.1 && left_clear.1 && !front_clear.0) || 
                    (front_clear.1 && !front_clear.0) ||
                    (right_clear2.1 && left_clear2.1 && !front_clear.0) ||
                    (right_clear2.1 && left_clear.1 && !front_clear.0) ||
                    (right_clear.1 && left_clear2.1 && !front_clear.0) ||
                    (right_clear.1 && right_clear2.1 && left_clear.1) ||
                    (left_clear.1 && left_clear2.1 && right_clear.1) {
                // Move backward slightly
                _bot.position.coordinates.x -= movement_dir.x * (BOT_MOVE * 0.5) * FIXED_DELTA; 

                // Both left and right are blocked, so rotate 180° (turn around)
                _bot.position.rotation += std::f32::consts::PI ; // Turn around (180 degrees) // turns around but for now we have this commentd so that it looks at player.
                
            }
            
            else if left_clear.1 && !right_clear.1 || left_clear2.1 && !right_clear2.1{
                _bot.position.rotation -= rotation_speed * FIXED_DELTA; // Rotate left (positive direction)
            } 
            
            else if right_clear.1 && !left_clear.1 || right_clear2.1 && !left_clear2.1{
                _bot.position.rotation += rotation_speed * FIXED_DELTA; // Rotate right to avoid collision (positive direction)
            }
            

        }

        ctx.db.bots().id().update(_bot);

        Ok(())

    } else {
        Err("Bot not found".to_string())
    }
}

pub fn check_collisions_at_points(
    points: &[Vec3],
    obstacles: impl Iterator<Item = Obstacle>,
    players: impl Iterator<Item = Player>,
    check_radius: f32,
) -> Vec<(bool, bool)> {
    let radius_sq = check_radius * check_radius;
    let mut results = vec![(false, false); points.len()];
    
    // Check obstacles
    for obstacle in obstacles {
        let o_pos = Vec3::new(obstacle.position.x, obstacle.position.y, 0.0);
        
        for (i, point) in points.iter().enumerate() {
            if point.distance_squared(o_pos) < radius_sq {
                results[i].1 = true; // Obstacle hit
            }
        }
    }
    
    // Check players
    for player in players {
        let p_pos = Vec3::new(player.position.coordinates.x, player.position.coordinates.y, 0.0);
        
        for (i, point) in points.iter().enumerate() {
            if point.distance_squared(p_pos) < radius_sq {
                results[i].0 = true; // Player hit
            }
        }
    }
    
    results
}

pub fn will_collide(
    new_pos: Vec3,
    obstacles: &[Obstacle],
    players: &[Player],
) -> (bool, bool) {
    let player_radius = BOT_SIZE.min(BOT_SIZE) / 2.0;
    let obstacle_radius = BOT_SIZE.min(BOT_SIZE) / 2.0;
    let collision_distance = player_radius + obstacle_radius;

    let obstacle_hit = obstacles.iter().any(|obstacle| {
        let obstacle_pos = Vec3::new(obstacle.position.x, obstacle.position.y, 0.0);
        new_pos.distance(obstacle_pos) < collision_distance
    });

    let player_hit = players.iter().any(|player| {
        let obstacle_pos = Vec3::new(player.position.coordinates.x, player.position.coordinates.y, 0.0);
        new_pos.distance(obstacle_pos) < collision_distance
    });

    //obstacle_hit || player_hit
    (player_hit, obstacle_hit)
}



/*
// Reducer for updating bot position
#[spacetimedb::reducer]
pub fn update_bot_position(
    ctx: &ReducerContext,
    bevy_transform: BevyTransform,
    bot_id: u64,
    new_rotate_dir: f32,
) -> Result<(), String> {
    // Find requested bot by bot id.
    if let Some(mut _bot) = ctx.db.bots().iter().find(|b| b.id == bot_id) {
        // Update bot position and rotation.
        _bot.position = bevy_transform;
        _bot.rotation_dir = new_rotate_dir;

        // Update column in "bots" table.
        ctx.db.bots().id().update(_bot);
        Ok(())
    } else {
        // Reaches only when requesting a bot with a unknown identity.
        Err("Bot not found".to_string())
    }
}*/


/* 
// Reducer for updating bot position
#[spacetimedb::reducer]
pub fn update_bot_position(
    ctx: &ReducerContext,
    bot_id: u64,
) -> Result<(), String> {
    log::info!(
        "Code reaches this point! --------------",

    );
    let obstacles = ctx.db.obstacle().iter().collect::<Vec<_>>();
    let players = ctx.db.player().iter().collect::<Vec<_>>();


    if let Some(mut _bot) = ctx.db.bots().iter().find(|b| b.id == bot_id) {
        let server_dir= &_bot.movement_dir;
        let bevy_dir = Vec3::new(server_dir.x, server_dir.y, server_dir.z);

        let server_rotation = _bot.position.rotation;

        let mut transform_rotation = Quat::from_rotation_z(server_rotation);
        let mut transform_translation = Vec3::new(
            _bot.position.coordinates.x,
            _bot.position.coordinates.y,
            0.0,
        );

        let mut movement_dir = transform_rotation * bevy_dir;
        
        let mut new_pos = transform_translation + movement_dir * BOT_MOVE * FIXED_DELTA;

        let mut rotation_dir = _bot.rotation_dir;

        let front_direction = transform_rotation * Vec3::new(1.0, 0.0, 0.0);
        let front_pos = transform_translation + front_direction * BOT_SIZE; // Adjust distance


        //if !will_collide(front_pos, &obstacles, &players){
            transform_translation = new_pos;
            _bot.position.coordinates.x = transform_translation.x;
            _bot.position.coordinates.y = transform_translation.y;
            _bot.position.rotation = transform_rotation.to_euler(glam::EulerRot::XYZ).2;
            _bot.rotation_dir = rotation_dir;
            ctx.db.bots().id().update(_bot);
        

        //ctx.db.bots().id().update(_bot);

        Ok(())
    } else {
        Err("Bot not found".to_string())
    }
}
*/

/// Function for respawning all bots in server when no player is online.
/// Server invokes this function in "player_disconnected" reducer when all players in the server is marked "offline" (online=false).
pub fn reset_bots_if_no_players_online(ctx: &ReducerContext) -> Result<(), String> {
    // Check if any players are online.
    let online_players_exist = ctx.db.player().iter().any(|p| p.online);

    // Do nothing if any player is online.
    if online_players_exist {
        return Ok(());
    }

    // Original bot spawn points. Three bots are spawned.
    let bot_spawn_positions = vec![  
        (-15432.0, 12340.0),  
        (12984.0, -11324.0),  
        (-8764.0, -13240.0),  
        (5432.0, 10876.0),  
        (12098.0, -2345.0),  
        (-4321.0, 5432.0),  
        (8234.0, -9321.0),  
        (-14230.0, 8391.0),  
        (987.0, 1234.0),  
        (16000.0, -16000.0),  
        (-16384.0, 16384.0),  
        (1000.0, -500.0),  
        (10234.0, 9423.0),  
        (-9382.0, -12083.0),  
        (8745.0, 9843.0),  
        (-4321.0, -6789.0),  
        (14230.0, -5432.0),  
        (-10987.0, 10342.0),  
        (11234.0, -7890.0),  
        (-12345.0, -12345.0),  
        (3198.0, 8754.0),  
        (-6543.0, 14982.0),  
        (4321.0, -9321.0),  
        (-15432.0, -1324.0),  
        (13842.0, 15843.0),  
        (-12983.0, 2342.0),  
        (7654.0, -7654.0),  
        (-8432.0, 7654.0),  
        (15678.0, -13245.0),  
        (-12874.0, 8234.0),  
    ];



    // Reset each bot (e.g., set them to some default positions).
    for (i, mut bot) in ctx.db.bots().iter().enumerate() {
        let (x, y) = bot_spawn_positions.get(i).cloned().unwrap_or((0.0, 0.0));

        bot.position = BevyTransform {
            coordinates: Vec2 { x, y },
            rotation: 0.0,
            scale: Vec2 { x: 0.0, y: 0.0 },
        };
        // Update column in "bots" table.
        ctx.db.bots().id().update(bot);
    }

    Ok(())
}

/// Reducer for creating and/or login existing player to server.
/// Server invokes this reducer when client establishes connection to server.
#[spacetimedb::reducer(client_connected)]
pub fn player_connected(ctx: &ReducerContext) {
    // Check if returning player.
    if let Some(_player) = ctx.db.player().identity().find(ctx.sender) {
        // Update modified column in "player" table.
        ctx.db.player().identity().update(Player {
            // Change from offline to online.
            online: true,
            .._player
        });
    } else {
        // If first time connecting to server.
        // Insert new column in "player" table.
        ctx.db.player().insert(Player {
            // Set player Identity to connecting client.
            identity: ctx.sender,
            name: "Lorem Ipsum".to_string(),
            //name,
            // Set default position data.
            position: BevyTransform {
                coordinates: Vec2 { x: 0.0, y: 0.0 },
                rotation: 0.0,
                scale: Vec2 { x: 50.0, y: 100.0 },
            },
            // Set to online.
            online: true,
            // Set default hook component data.
            hook: Hook {
                position: Vec2 { x: 0.0, y: 0.0 },
                rotation: 0.0,
                width: 0.0,
                height: 0.0,
            },
            track: Track {
                owner_identity: ctx.sender,
                position: BevyTransform {
                    coordinates: Vec2 { x: 0.0, y: 0.0 },
                    rotation: 0.0,
                    scale: Vec2 { x: 50.0, y: 100.0 },
                },
                width: 0.0,
                height: 0.0,
                rotation: 0.0,
                id: 0,
            },
            grid: Grid {
                load: 0,
                next_free_x: -1,
                next_free_y: 0,
            },
        });
    }
}

/// Reducer for logging out player from servers.
/// Server invokes this reducer when client disconnects from server
#[spacetimedb::reducer(client_disconnected)]
pub fn player_disconnected(ctx: &ReducerContext) {
    // Find requested player by Identity.
    if let Some(_player) = ctx.db.player().identity().find(ctx.sender) {
        // Update modified column in "player" table.
        ctx.db.player().identity().update(Player {
            // Set to offline.
            online: false,
            .._player
        });
    } else {
        // Reaches only when requesting player with unknown identity.
        // Should never reach!!
        log::warn!(
            "Disconnect event for unknown user with identity {:?}",
            ctx.sender
        )
    }

    // Check if all players are offfline, in which case reset the bots.
    reset_bots_if_no_players_online(ctx).unwrap();
}

/// Reducer for generating obstacles and bots in server.
/// Server invokes this reducer during intialization of the server.
#[spacetimedb::reducer(init)]
pub fn server_startup(ctx: &ReducerContext) {
    // Generate obstacles in server.
    generate_obstacles(ctx);
    // Generate bots in server.
    generate_bots(ctx);
    generate_blocks(ctx);
    generate_leaderboard(ctx);
}

#[spacetimedb::reducer]
pub fn update_block_owner(
    ctx: &ReducerContext,
    block_id: u64,
    new_owner: OwnerType,
    offset_x: i32,
    offset_y: i32,
) -> Result<(), String> {
    if let Some(mut block) = ctx.db.block().id().find(block_id) {
        block.owner = new_owner;
        block.offset_x = offset_x;
        block.offset_y = offset_y;
        ctx.db.block().id().update(block);
        Ok(())
    } else {
        Err("Block does not exist".to_string())
    }
}

// Function for generating bots in server.
// Server invokes this function in "server_startup" reducer during server intialization.
fn generate_bots(ctx: &ReducerContext) {
    // Example bot generation logic, spawn three bots.
    let bot_spawn_positions = vec![  
        (-15432.0, 12340.0),  
        (12984.0, -11324.0),  
        (-8764.0, -13240.0),  
        (5432.0, 10876.0),  
        (12098.0, -2345.0),  
        (-4321.0, 5432.0),  
        (8234.0, -9321.0),  
        (-14230.0, 8391.0),  
        (987.0, 1234.0),  
        (16000.0, -16000.0),  
        (-16384.0, 16384.0),  
        (1000.0, -500.0),  
        (10234.0, 9423.0),  
        (-9382.0, -12083.0),  
        (8745.0, 9843.0),  
        (-4321.0, -6789.0),  
        (14230.0, -5432.0),  
        (-10987.0, 10342.0),  
        (11234.0, -7890.0),  
        (-12345.0, -12345.0),  
        (3198.0, 8754.0),  
        (-6543.0, 14982.0),  
        (4321.0, -9321.0),  
        (-15432.0, -1324.0),  
        (13842.0, 15843.0),  
        (-12983.0, 2342.0),  
        (7654.0, -7654.0),  
        (-8432.0, 7654.0),  
        (15678.0, -13245.0),  
        (-12874.0, 8234.0),  
    ];


    // Generate and insert bots into the database.
    for (i, (x, y)) in bot_spawn_positions.into_iter().enumerate() {
        let bot_id = i as u64; // Unique ID for each bot
        let bot_transform = BevyTransform {
            coordinates: Vec2 { x, y },
            rotation: 0.0, // Initial rotation
            scale: Vec2 { x: 1.0, y: 1.0 },
        };

        // Insert bot into the database.
        ctx.db.bots().insert(Bot {
            id: bot_id,
            position: bot_transform,
            alive: true, // Bots are alive when generated
            movement_dir: Vec3_space {
                x: 0.5,
                y: 0.0,
                z: 0.0,
            },
            rotation_dir: 0.0,
        });
    }
}

/// Function for generating obstacles in server.
/// Server invokes this function in "server_startup" reducer during server initialization.
fn generate_blocks(ctx: &ReducerContext) {
    let blocks_per_bot = 10;
    let grid_size = (1, 2); // num 2 does not matter
    let mut block_id = 0;

    for bot in 0..N_BOTS {
        let mut pos = (-1, 0);

        for block_num in 0..blocks_per_bot {
            ctx.db.block().insert(Block {
                id: block_id,
                offset_x: pos.0,
                offset_y: pos.1,
                owner: OwnerType::Bot(bot),
            });
            block_id += 1;
            increment_grid_pos(&mut pos, grid_size);
        }
    }
}

fn increment_grid_pos(grid_pos: &mut (i32, i32), grid_max: (i32, i32)) {
    // increment grid pos
    grid_pos.0 += 1;
    if *grid_pos == (0, 0) {
        grid_pos.0 += 1;
    }
    if grid_pos.0 > grid_max.0 {
        grid_pos.0 -= 1;
        grid_pos.0 = -grid_pos.0;
        grid_pos.1 -= 1;
    }

    //grid.load += 1;
    //player.block_count += 1;
}

fn generate_obstacles(ctx: &ReducerContext) {
    // Initialize 2 noise generators with different seeds.
    let perlin_x = Perlin::new(21);
    let perlin_y = Perlin::new(1345);
    // Generate 200 obstacles.
    for i in 0..200 {
        // Control frequency.
        let x = (i as f32) / 10.0;
        let y = ((i + 1) as f32) / 10.0;

        // Noise generate x & y values within the map (8192*8192).
        let random_x = perlin_x.get([x as f64, y as f64]) as f32 * 4056.0;
        let random_y = perlin_y.get([y as f64, x as f64]) as f32 * 4056.0;

        // Define invalid x & y values within the safe zone (spawn point).
        let invalid_x = random_x < 300.0 && random_x > -300.0;
        let invalid_y = random_y < 300.0 && random_y > -300.0;

        // If noise generated spawnpoint is within safe zone, do not spawn obstacle.
        if invalid_x && invalid_y {
            continue;
        }

        // Insert column in "obstacle" table.
        ctx.db.obstacle().insert(Obstacle {
            position: Vec2 {
                // Insert noise generated x & y values.
                x: random_x,
                y: random_y,
            },
            // Set ID to iteration integer "i".
            id: i,
            // Set default HP value.
            hp: 100,
        });
    }
}


#[spacetimedb::reducer]
pub fn generate_leaderboard(ctx: &ReducerContext){
	// Create a new leaderboard entry with an empty top players list
    let leaderboard = Leaderboard {
        id: 1, // id är 1
        top_players: Vec::new(),
    };

    // Insert the leaderboard into the database
    ctx.db.leaderboard().insert(leaderboard);
}