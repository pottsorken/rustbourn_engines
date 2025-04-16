use spacetimedb::{
    reducer, rand,
    spacetimedb_lib::{db, identity},
    table, DbContext, Identity, ReducerContext, SpacetimeType, Table, Timestamp,
};

use glam::{Vec3, Quat};
use noise::{NoiseFn, Perlin};

const FIXED_DELTA: f32 = 1.0 / 120.0; // Fixed delta for 120 FPS simulation
const BOT_SIZE: f32 = 80.0; // Size of bot
const BOT_MOVE: f32 = 100.0;
use std::f32::consts::PI;

// Player data
#[spacetimedb::table(name = player, public)]
pub struct Player {
    #[primary_key]
    identity: Identity,
    position: BevyTransform,
    online: bool,
    hook: Hook,
}

#[spacetimedb::table(name = obstacle, public)]
pub struct Obstacle {
    position: Vec2,
    #[primary_key]
    id: u64,
    hp: u32,
}

#[spacetimedb::table(name = bots, public)]
pub struct Bot {
    #[primary_key]
    id: u64,
    position: BevyTransform,
    alive: bool, // instead of online we have alive that checks if that specific bot is alive
    movement_dir: Vec3_space,
    rotation_dir: f32,
}

// Hook component data
#[derive(Debug, SpacetimeType)]
pub struct Hook {
    position: Vec2,
    rotation: f32,
    width: f32,
    height: f32,
}

// Bevy transform data
#[derive(Debug, SpacetimeType)]
pub struct BevyTransform {
    coordinates: Vec2,
    rotation: f32,
    scale: Vec2,
}

// Vector with x, y coordinates
#[derive(Debug, SpacetimeType)]
pub struct Vec2 {
    x: f32,
    y: f32,
}

// New
// Vector with x, y coordinates

#[derive(Debug, SpacetimeType)]
pub struct Vec3_space {
    x: f32,
    y: f32,
    z: f32,
}


//Reducer for damaging obstacle
#[spacetimedb::reducer]
pub fn damage_obstacle(ctx: &ReducerContext, id: u64, damage: u32) -> Result<(), String> {
    if let Some(mut obstacle) = ctx.db.obstacle().id().find(id) {
        obstacle.hp = obstacle.hp.saturating_sub(damage);
        ctx.db.obstacle().id().update(obstacle);
        Ok(())
    } else {
        Err("Obstacle does not exist!".to_string())
    }
}

// Reducer for updating hook position
#[spacetimedb::reducer]
pub fn update_hook_position(
    ctx: &ReducerContext,
    identity: Identity,
    position: Vec2,
    rotation: f32,
) -> Result<(), String> {
    // Find player by id
    if let Some(mut player) = ctx.db.player().identity().find(identity) {
        // Update player hook position
        player.hook.position = position;
        player.hook.rotation = rotation;
        ctx.db.player().identity().update(player);
        Ok(())
    } else {
        Err("Player not found".to_string())
    }
}

#[spacetimedb::reducer]
pub fn update_hook_movement(
    ctx: &ReducerContext,
    identity: Identity,
    width: f32,
    height: f32,
) -> Result<(), String> {
    if let Some(mut player) = ctx.db.player().identity().find(identity) {
        player.hook.width = width;
        player.hook.height = height;
        ctx.db.player().identity().update(player);
        Ok(())
    } else {
        Err("Player not found".to_string())
    }
}

// Reducer for updating player position
#[spacetimedb::reducer]
pub fn update_player_position(
    ctx: &ReducerContext,
    bevy_transform: BevyTransform,
) -> Result<(), String> {
    log::info!(
        "Code reaches this point! --------{:?}------",
        bevy_transform
    );
    if let Some(_player) = ctx.db.player().identity().find(ctx.sender) {
        ctx.db.player().identity().update(Player {
            position: bevy_transform,
            .._player
        });
        Ok(())
    } else {
        Err("Player not found".to_string())
    }
}

// Reducer for updating bot position
#[spacetimedb::reducer]
pub fn update_bot_position(
    ctx: &ReducerContext,
    bevy_transform: BevyTransform,
    bot_id: u64,
    new_rotate_dir: f32,
) -> Result<(), String> {
    log::info!(
        "Code reaches this point! --------{:?}------",
        bevy_transform
    );

    let obstacles = ctx.db.obstacle().iter().collect::<Vec<_>>();
    let players = ctx.db.player().iter().collect::<Vec<_>>();
    let rotation_speed = f32::to_radians(1.0);

    if let Some(mut _bot) = ctx.db.bots().iter().find(|b| b.id == bot_id) {
        _bot.position = bevy_transform;
        _bot.rotation_dir = new_rotate_dir;

        let bevy_dir = Vec3::new(_bot.movement_dir.x, _bot.movement_dir.y, _bot.movement_dir.z);
        

        let mut transform_translation = Vec3::new(_bot.position.coordinates.x, _bot.position.coordinates.y, 0.0);

        let rotation_quat = Quat::from_rotation_z(_bot.position.rotation);

        let mut movement_dir = rotation_quat * bevy_dir;
        let front_dir = rotation_quat * Vec3::new(1.5, 0.0, 0.0);
        //let left_dir = rotation_quat * Quat::from_rotation_z(f32::to_radians(45.0)) * Vec3::X;
        //let right_dir = rotation_quat * Quat::from_rotation_z(f32::to_radians(-45.0)) * Vec3::X;
        
        let left_dir = rotation_quat * Vec3::new(2.0, 1.5, 0.0);
        let right_dir = rotation_quat * Vec3::new(2.0, -1.5, 0.0);

        let mut rotation_dir = _bot.rotation_dir;

        let front_pos = transform_translation + front_dir * BOT_SIZE; // Adjust distance
        let left_pos = transform_translation + left_dir * BOT_SIZE; // Adjust distance
        let right_pos = transform_translation + right_dir * BOT_SIZE; // Adjust distance

        let mut new_pos = transform_translation + movement_dir * BOT_MOVE * FIXED_DELTA;

        let left_clear = will_collide(left_pos, &obstacles, &players);
        let right_clear = will_collide(right_pos, &obstacles, &players);
        let front_clear = will_collide(front_pos, &obstacles, &players);


        /* 
        if !will_collide(front_pos, &obstacles, &players) { // Front vector
            _bot.position.coordinates.x = new_pos.x;
            _bot.position.coordinates.y = new_pos.y;
        } 
        */


        if !left_clear.0 && !right_clear.0 && !front_clear.0 && !left_clear.1 && !right_clear.1 && !front_clear.1 {
            _bot.position.coordinates.x = new_pos.x;
            _bot.position.coordinates.y = new_pos.y;
        }
        else {

            let rotation_speed = f32::to_radians(90.0);
            // Player is on the left and NOT on the right → rotate right to chas
            // Check left and right vision
            if left_clear.0 && !right_clear.0 {
                // If left is clear but right is blocked, rotate right
                //_bot.position.rotation += rotation_speed * FIXED_DELTA; // Rotate left (positive direction)
                _bot.position.rotation += rotation_speed * FIXED_DELTA; // Tracking to right

            } 
            // Player is on the right and NOT on the left → rotate left to chase
            else if right_clear.0 && !left_clear.0 {
                // If right is clear but left is blocked, rotate left
                //_bot.position.rotation -= rotation_speed * FIXED_DELTA; // Rotate right (negative direction)
                _bot.position.rotation -= rotation_speed * FIXED_DELTA; // tracking to left

            } 
            
            else if (right_clear.1 && left_clear.1 && !front_clear.0) || (front_clear.1 && !front_clear.0) {
                // Move backward slightly
                _bot.position.coordinates.x -= movement_dir.x * (BOT_MOVE * 0.5) * FIXED_DELTA; 

                // Both left and right are blocked, so rotate 180° (turn around)
                _bot.position.rotation += std::f32::consts::PI ; // Turn around (180 degrees) // turns around but for now we have this commentd so that it looks at player.
                
            }
            
            else if left_clear.1 && !right_clear.1 {
                _bot.position.rotation -= rotation_speed * FIXED_DELTA; // Rotate left (positive direction)
            } 
            
            else if right_clear.1 && !left_clear.1 {
                _bot.position.rotation += rotation_speed * FIXED_DELTA; // Rotate right to avoid collision (positive direction)
            }
            
        

            _bot.position.rotation = _bot.position.rotation % std::f32::consts::TAU;
        }

        ctx.db.bots().id().update(_bot);

        Ok(())

    } else {
        Err("Bot not found".to_string())
    }
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
    log::info!(
        "Code reaches this point! --------{:?}------",
        bevy_transform
    );
    if let Some(mut _bot) = ctx.db.bots().iter().find(|b| b.id == bot_id) {
        _bot.position = bevy_transform;
        _bot.rotation_dir = new_rotate_dir;

        ctx.db.bots().id().update(_bot);

        Ok(())
    } else {
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

#[spacetimedb::reducer]
pub fn reset_bots_if_no_players_online(ctx: &ReducerContext) -> Result<(), String> {
    // Check if any players are online
    let online_players_exist = ctx.db.player().iter().any(|p| p.online);

    if online_players_exist {
        return Ok(()); // Do nothing if any player is online
    }

    // Your original bot spawn points
    let bot_spawn_positions = vec![(200.0, 20.0), (-200.0, -20.0), (200.0, -250.0)];

    // Reset each bot (e.g., set them to some default positions)
    for (i, mut bot) in ctx.db.bots().iter().enumerate() {
        let (x, y) = bot_spawn_positions.get(i).cloned().unwrap_or((0.0, 0.0));

        bot.position = BevyTransform {
            coordinates: Vec2 { x, y },
            rotation: 0.0,
            scale: Vec2 { x: 0.0, y: 0.0 },
        };
        ctx.db.bots().id().update(bot);
    }

    Ok(())
}

// Reducer for creating and/or login existing player to server
// Called when client connects
#[spacetimedb::reducer(client_connected)]
pub fn player_connected(ctx: &ReducerContext) {
    log::info!(" HEY -----");
    if let Some(_player) = ctx.db.player().identity().find(ctx.sender) {
        ctx.db.player().identity().update(Player {
            online: true,
            .._player
        });
    } else {
        ctx.db.player().insert(Player {
            identity: ctx.sender,
            position: BevyTransform {
                coordinates: Vec2 { x: 0.0, y: 0.0 },
                rotation: 0.0,
                scale: Vec2 { x: 50.0, y: 100.0 },
            },
            online: true,
            hook: Hook {
                position: Vec2 { x: 0.0, y: 0.0 },
                rotation: 0.0,
                width: 0.0,
                height: 0.0,
            },
        });
    }
}

// Reducer for logging out player
// Called when client disconnects from server
#[spacetimedb::reducer(client_disconnected)]
pub fn player_disconnected(ctx: &ReducerContext) {
    if let Some(_player) = ctx.db.player().identity().find(ctx.sender) {
        ctx.db.player().identity().update(Player {
            online: false,
            .._player
        });
    } else {
        // Should never reach!!
        log::warn!(
            "Disconnect event for unknown user with identity {:?}",
            ctx.sender
        )
    }

    reset_bots_if_no_players_online(ctx);
}

#[spacetimedb::reducer(init)]
pub fn server_startup(ctx: &ReducerContext) {
    generate_obstacles(ctx);
    generate_bots(ctx);
}


fn generate_bots(ctx: &ReducerContext) {
    // Example bot generation logic
    let bot_spawn_positions = vec![(200.0, 20.0), (-200.0, -20.0), (200.0, -250.0)];

    // Generate and insert bots into the database
    for (i, (x, y)) in bot_spawn_positions.into_iter().enumerate() {
        let bot_id = i as u64; // Unique ID for each bot
        let bot_transform = BevyTransform {
            coordinates: Vec2 { x, y },
            rotation: 0.0, // Initial rotation
            scale: Vec2 { x: 1.0, y: 1.0 },
        };

        // Insert bot into the database
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

fn generate_obstacles(ctx: &ReducerContext) {
    let mut rng = ctx.rng();
    let perlin_x = Perlin::new(21);
    let perlin_y = Perlin::new(1345);
    // Generate 200 obstacles
    for i in 0..200 {
        let x = (i as f32) / 10.0; // Control frequency
        let y = ((i + 1) as f32) / 10.0;

        let random_x = perlin_x.get([x as f64, y as f64]) as f32 * 4056.0;
        let random_y = perlin_y.get([y as f64, x as f64]) as f32 * 4056.0;
        let invalid_x = random_x < 300.0 && random_x > -300.0;
        let invalid_y = random_y < 300.0 && random_y > -300.0;

        if invalid_x && invalid_y {
            continue;
        }

        ctx.db.obstacle().insert(Obstacle {
            position: Vec2 {
                x: random_x,
                y: random_y,
            },
            id: i,
            hp: 100,
        });
    }
}
