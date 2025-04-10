use crate::common::{
    Bot, 
    Obstacle, 
    BOT_CONFIG, 
    OBSTACLE_CONFIG,
};
use crate::db_connection::{
    load_bots, 
    update_bot_position, 
    CtxWrapper
};
use crate::player_attach::check_collision;
use bevy::prelude::*;

pub fn spawn_bots(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ctx_wrapper: Res<CtxWrapper>,
    query: Query<&Bot>,
) {
    if query.is_empty() {
        // Check if the query is empty, meaning no bots are in the world yet

        let bots = load_bots(&ctx_wrapper);
        println!("[BOTS] Loaded {} bots", bots.len());

        for (x, y, bot_id) in bots {
            println!("[BOTS] Spawning bot {} at ({}, {})", bot_id, x, y);

            commands.spawn((
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
            ));
        }
    }
}

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
        let mut rotation_dir = 0.0 ;

        let front_direction = transform.rotation * Vec3::new(1.0, 0.0, 0.0);
        let front_pos = transform.translation + front_direction * BOT_CONFIG.size.x; // Adjust distance

        if !check_collision(
            front_pos.truncate(), 
            &obstacle_query, 
            BOT_CONFIG.size, 
            OBSTACLE_CONFIG.size
        ) {
            // If no collision, update the bot's position
            transform.translation = new_pos;
        } else {
            //println!("Hello");
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
                OBSTACLE_CONFIG.size
            );
            let right_clear = !check_collision(
                right_pos.truncate(),
                 &obstacle_query, 
                 BOT_CONFIG.size,
                OBSTACLE_CONFIG.size
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
        update_bot_position(&ctx_wrapper, &transform, _bot.id);
        //println!("[BOT] {} collided at ({}, {})", _bot.id, transform.translation.x, transform.translation.y);
    }
}
