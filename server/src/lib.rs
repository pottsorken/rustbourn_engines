use spacetimedb::{
    reducer,
    spacetimedb_lib::{db, identity},
    table, DbContext, Identity, ReducerContext, SpacetimeType, Table, Timestamp,
};

const N_BOTS: u64 = 3;
const N_OBSTACLES: u64 = 200;

use noise::{NoiseFn, Perlin};

/// Player component data
#[spacetimedb::table(name = player, public)]
pub struct Player {
    #[primary_key]
    identity: Identity,
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
    // Instead of online we have alive that checks if that specific bot is alive
    alive: bool,
    // Movement direction
    movement_dir: Vec3,
    // Rotation direction
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
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

/// Reducer for decreasing a ("id") specific obstacle's HP by "damage" points.
/// Client invokes this reducer in "handle_obstacle_hit" function when dealing damage to an obstacle with their hook.
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

/// Reducer for updating a ("bot_id") specific bot position by sending the entity data contained in a transform. All data is sent in a custom "BevyTransform" struct, except "new_rotate_dir".
/// Client invokes this reducer in "render_bots_from_db" function when updating the position of the bot sprite.
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
}

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
    let bot_spawn_positions = vec![(200.0, 20.0), (-200.0, -20.0), (200.0, -250.0)];

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
    } else { // If first time connecting to server.
        // Insert new column in "player" table.
        ctx.db.player().insert(Player {
            // Set player Identity to connecting client.
            identity: ctx.sender,
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
    let bot_spawn_positions = vec![(200.0, 20.0), (-200.0, -20.0), (200.0, -250.0)];

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
            // Bots are alive when generated.
            alive: true,
            movement_dir: Vec3 {
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
    let blocks_per_bot = 5;
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
