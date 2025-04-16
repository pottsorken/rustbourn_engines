use bevy::prelude::*;
use crate::common::*;
use crate::start_menu::*;

// pub fn running_state(state: Res<State<GameState>>) -> bool {
//     // matches!(*state, Game | Edit)
//     state == Game | state == Edit;
// }

pub fn in_game_or_edit(state: Res<State<GameState>>) -> bool {
    matches!(state.get(), GameState::Game | GameState::Edit)
}

pub fn edit_plugin(app: &mut App) {
    app
        .add_systems(Update, toggle_edit_menu)//.run_in(in_game_or_edit)
        .add_systems(OnEnter(GameState::Edit), edit_setup)
        .add_systems(OnExit(GameState::Edit), despawn_screen::<OnEditScreen>)
        .add_systems(Update, handle_camera);//.run_if(in_game_or_edit));
}

pub fn toggle_edit_menu(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    // mut commands: Commands,
    mut game_state: ResMut<NextState<GameState>>,
    state: Res<State<GameState>>,
    // edit_menu: Query<Entity, With<OnEditScreen>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyO) {
        let current_state = *state.get();
        
        match current_state {
            GameState::Game => {
                game_state.set(GameState::Edit);
            }
            GameState::Edit => {
                game_state.set(GameState::Game);
            }
            _ => {}
        }
    }
}

pub fn edit_setup(mut commands: Commands) {
    // Create a semi-transparent background
    let background_color = Color::rgba(0.0, 0.0, 0.0, 0.5); // Dark overlay
    
    // Create the root node
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                ..default()
            },
            OnEditScreen,
            BackgroundColor(background_color),
            ZIndex(200),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("EDIT MODE"),
                TextFont {
                    font_size: 67.0,
                    ..default()
                },
                TextColor(Color::rgba(1.0, 1.0, 1.0, 1.0)), // White text
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(50.0),
                    top: Val::Px(50.0),
                    ..default()
                },
                ZIndex(201),
            ));
        });
}

pub fn handle_camera(
    state: Res<State<GameState>>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    player_query: Query<&Transform, (With<Player>, Without<Camera>)>,
) {
    if let Ok(mut camera_transform) = camera_query.get_single_mut() {
        if let Ok(player_transform) = player_query.get_single() {
            // Always keep the camera centered on the player, regardless of state
            camera_transform.translation = Vec3::new(
                player_transform.translation.x,
                player_transform.translation.y,
                camera_transform.translation.z,
            );
        }
    }
}