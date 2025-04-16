use bevy::prelude::*;
use crate::common::{Parry, PlayerAttach};


pub fn setup_parry(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn a player sprite at position (0, 0) at a higher z-index than map
    commands.spawn((
        Sprite {
            custom_size: Some(Vec2::new(25.0, 26.0)), // Square size 100x100 pixels
            image: asset_server.load("sprites/parry/circle1.png"),
            anchor: bevy::sprite::Anchor::Center,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 5.0),
       
        Parry {
            active : false,
            parry_duration: 0.5,
            timer: Timer::from_seconds(0.5, TimerMode::Once),
        },
        PlayerAttach {
            offset: Vec2::new(0.0, 20.0), // Offset from player's center
        },
    ));
}

pub fn parry_controls(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Sprite, &mut Transform), With<Parry>>,
    time: Res<Time>,
) 
{
    for (mut sprite, mut parry) in query.iter_mut() {

         if keyboard_input.just_pressed(KeyCode::KeyF) {
              parry.active = true;
              //add the code for timer
            }
        
        if parry.active{
        if let Some(size) = sprite.custom_size {
                sprite.custom_size = Some(Vec2::new(size.x, size.y * 1.2));
            if parry.timer.finished()
            {
                parry.active =false;
                sprite.custom_size = Some(Vec2::new(25.0, 26.0));


            }
        }



    }
}}