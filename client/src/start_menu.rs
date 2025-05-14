// Information from:
// https://bevyengine.org/examples/games/game-menu/

use crate::common::*;
use crate::common::*;
use bevy::prelude::*;
use bevy::text::*;
use bevy::{app::AppExit, color::palettes::css::CRIMSON, prelude::*};

use rand::prelude::IndexedRandom;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
pub const BACKGROUND_COLOR: Color = Color::srgb(
    0.11764705882352941,
    0.11764705882352941,
    0.11764705882352941,
);
pub const SPLASH_COLOR: Color = Color::srgb(0.0, 0.0, 0.0);

pub const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
// pub const USERNAME_BUTTON: Color = Color::srgb(0.2823529411764706, 0.2823529411764706, 0.2823529411764706);
pub const USERNAME_BUTTON: Color = Color::srgb(1.0, 0.0, 0.0);
pub const HOVERED_BUTTON: Color =
    Color::srgb(0.2823529411764706, 0.2823529411764706, 0.2823529411764706);
pub const HOVERED_PRESSED_BUTTON: Color =
    Color::srgb(0.9254901960784314, 0.6313725490196078, 0.2196078431372549);
pub const PRESSED_BUTTON: Color =
    Color::srgb(0.7686274509803922, 0.3764705882352941, 0.054901960784313725);

// ###################################### SPLASH ##########################################

pub fn splash_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Splash), splash_setup)
        .add_systems(Update, countdown.run_if(in_state(GameState::Splash)))
        .add_systems(OnExit(GameState::Splash), despawn_screen::<OnSplashScreen>);
}

pub fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let icon = asset_server.load("screens/splashscreen.png");
    // Display the logo on splash screen
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                // width: Val::Px(100.0),
                // height: Val::Px(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            OnSplashScreen,
            BackgroundColor(SPLASH_COLOR),
        ))
        .with_children(|parent| {
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        ImageNode::new(icon),
                        Node {
                            width: Val::Px(200.0),                   // Size of splash logo
                            margin: UiRect::all(Val::Percent(20.0)), // How far from the top the splash logo is placed
                            ..default()
                        },
                    ));
                });
        });

    commands.insert_resource(SplashTimer(Timer::from_seconds(2.0, TimerMode::Once)));
}

pub fn countdown(
    mut game_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
) {
    if timer.tick(time.delta()).finished() {
        game_state.set(GameState::Menu);
    }
}

// ###################################### GAME ##########################################

pub fn game_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Game), game_setup)
        .add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>);
}

pub fn game_setup(mut commands: Commands) {
    //commands.spawn(Camera2dBundle::default());
    //commands.spawn((
    //    Node {
    //        width: Val::Percent(100.0),
    //        height: Val::Percent(100.0),
    //        align_items: AlignItems::Center,
    //        justify_content: JustifyContent::Center,
    //        ..default()
    //    },
    //    OnGameScreen,
    //));
}

// ###################################### MENU ##########################################

pub fn menu_plugin(app: &mut App) {
    app.init_state::<MenuState>()
        .add_systems(OnEnter(GameState::Menu), menu_setup)
        .add_systems(OnEnter(MenuState::Main), main_menu_setup)
        .add_systems(OnExit(MenuState::Main), despawn_screen::<OnMainMenuScreen>)
        .add_systems(OnEnter(MenuState::Settings), settings_menu_setup)
        .add_systems(
            OnExit(MenuState::Settings),
            despawn_screen::<OnSettingsMenuScreen>,
        )
        .add_systems(
            OnEnter(MenuState::SettingsDisplay),
            display_settings_menu_setup,
        )
        // .add_systems(
        //     Update,
        //     (setting_button::<DisplayQuality>.run_if(in_state(MenuState::SettingsDisplay)),),
        // )
        // .add_systems(
        //     OnExit(MenuState::SettingsDisplay),
        //     despawn_screen::<OnDisplaySettingsMenuScreen>,
        // )
        // .add_systems(OnEnter(MenuState::SettingsSound), sound_settings_menu_setup)
        // .add_systems(
        //     Update,
        //     setting_button::<Volume>.run_if(in_state(MenuState::SettingsSound)),
        // )
        // .add_systems(
        //     OnExit(MenuState::SettingsSound),
        //     despawn_screen::<OnSoundSettingsMenuScreen>,
        // )
        // Common systems to all screens that handle buttons
        .add_systems(
            Update,
            (menu_action, button_system).run_if(in_state(GameState::Menu)),
        );
}

// This system handles changing all buttons color based on mouse interaction
pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut background_color, selected) in &mut interaction_query {
        *background_color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}

// This system updates the settings when a new value for a setting is selected, and marks
// the button as the one currently selected
pub fn setting_button<T: Resource + Component + PartialEq + Copy>(
    interaction_query: Query<(&Interaction, &T, Entity), (Changed<Interaction>, With<Button>)>,
    selected_query: Single<(Entity, &mut BackgroundColor), With<SelectedOption>>,
    mut commands: Commands,
    mut setting: ResMut<T>,
) {
    let (previous_button, mut previous_button_color) = selected_query.into_inner();
    for (interaction, button_setting, entity) in &interaction_query {
        if *interaction == Interaction::Pressed && *setting != *button_setting {
            *previous_button_color = NORMAL_BUTTON.into();
            commands.entity(previous_button).remove::<SelectedOption>();
            commands.entity(entity).insert(SelectedOption);
            *setting = *button_setting;
        }
    }
}

pub fn menu_setup(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::Main);
}

pub fn main_menu_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    //username: Res<Username>,
) {
    // Common style for all buttons on the screen
    let button_node = Node {
        width: Val::Px(300.0),
        height: Val::Px(75.0),
        margin: UiRect::all(Val::Px(20.0)),
        padding: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    // let username_button_node = Node {
    //     width: Val::Px(300.0),
    //     height: Val::Px(75.0),
    //     margin: UiRect::all(Val::Px(20.0)),
    //     padding: UiRect::all(Val::Px(20.0)),
    //     justify_content: JustifyContent::Center,
    //     align_items: AlignItems::Center,
    //     ..default()
    // };
    let button_icon_node = Node {
        width: Val::Px(30.0),
        position_type: PositionType::Absolute,
        left: Val::Px(10.0),
        ..default()
    };
    let button_text_font = TextFont {
        font_size: 30.0,
        ..default()
    };
    let username_button_text_font = TextFont {
        font_size: 20.0,
        ..default()
    };

    pub fn print_random_string(commands: &mut Commands) -> String {
        let usernames_first = [
            "cool", "lazy", "sneaky", "noisy", "rusty", "clever", "sleepy", "simple", "nuclear",
            "rowdy", "grumpy", "shady", "quick", "twitchy", "silent", "spunky", "clumsy", "ugly",
            "mighty", "smart", "trusty", "wild", "scrummy", "loud", "howling", "boring", "slow",
        ];

        let usernames_second = [
            "gear", "cog", "motor", "circuit", "valve", "bot", "servo", "engine", "module", "chain",
            "piston", "drone", "terminal", "chip", "sensor", "wheel", "pump", "turbine", "rope", "link",
            "switch", "coal", "hook", "fire", "city", "town", "gate", "platoon", "monster", "castle",
            "pipe", "screw", "bolt",
        ];

        let mut rng_first = thread_rng();
        let mut rng_second = thread_rng();

        let Some(random_username_first) = usernames_first.choose(&mut rng_first) else {
            todo!()
        };
        let Some(random_username_second) = usernames_second.choose(&mut rng_second) else {
            todo!()
        };
        let random_username = format!("{}-{}", random_username_first, random_username_second);

        //commands.insert_resource(Username {
        //    name: random_username.clone().to_string(),
        //});

        // println!("{}", random_username);
        return random_username.to_string();
    }

    let name = print_random_string(&mut commands);
    let mut username = name.clone();
    commands.insert_resource(Username { name: name.clone() });

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            OnMainMenuScreen,
            BackgroundColor(BACKGROUND_COLOR),
        ))
        .with_children(|parent| {
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                })
                .with_children(|parent| {
                    // Display title screen
                    parent.spawn((
                        ImageNode::new(asset_server.load("screens/titlescreen.png")),
                        Node {
                            width: Val::Px(600.0),
                            margin: UiRect::all(Val::Px(50.0)),
                            ..default()
                        },
                    ));

                    // Display username
                    parent
                        .spawn((
                            Button,
                            button_node.clone(),
                            BackgroundColor(PRESSED_BUTTON),
                            username = name.clone(),
                            // MenuButtonAction::Settings,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new(name.clone()),
                                username_button_text_font.clone(),
                                TextColor(PRESSED_BUTTON),
                            ));
                        });

                    // parent
                    //     .spawn((
                    //         Sprite::from_color(Color::srgb(0.25, 0.25, 0.55), box_size),
                    //         Transform::from_translation(box_position.extend(0.0)),
                    //     ))
                    //     .with_children(|builder| {
                    //         builder.spawn((
                    //             Text2d::new("this text wraps in the box\n(Unicode linebreaks)"),
                    //             button_text_font.clone(),
                    //             TextLayout::new(JustifyText::Left, LineBreak::WordBoundary),
                    //             // Wrap text in the rectangle
                    //             TextBounds::from(box_size),
                    //             // Ensure the text is drawn on top of the box
                    //             Transform::from_translation(Vec3::Z),
                    //         ));
                    //     });
                    // Play button

                    parent
                        .spawn((
                            Button,
                            button_node.clone(),
                            BackgroundColor(NORMAL_BUTTON),
                            MenuButtonAction::Play,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("PLAY"),
                                button_text_font.clone(),
                                TextColor(TEXT_COLOR),
                            ));
                        });
                    // parent
                    //     .spawn((
                    //         Button,
                    //         button_node.clone(),
                    //         BackgroundColor(NORMAL_BUTTON),
                    //         MenuButtonAction::Settings,
                    //     ))
                    //     .with_children(|parent| {
                    //         let icon = asset_server.load("textures/Game Icons/wrench.png");
                    //         parent.spawn((ImageNode::new(icon), button_icon_node.clone()));
                    //         parent.spawn((
                    //             Text::new("Settings"),
                    //             button_text_font.clone(),
                    //             TextColor(TEXT_COLOR),
                    //         ));
                    //     });

                    parent
                        .spawn((
                            Button,
                            button_node,
                            BackgroundColor(NORMAL_BUTTON),
                            MenuButtonAction::Quit,
                        ))
                        .with_children(|parent| {
                            let icon = asset_server.load("textures/Game Icons/wrench.png");
                            parent.spawn((ImageNode::new(icon), button_icon_node));
                            parent.spawn((
                                Text::new("Quit"),
                                button_text_font,
                                TextColor(TEXT_COLOR),
                            ));
                        });
                });

            // .with_children(|parent| {
            //     parent.spawn(TextBundle::from_section(
            //         "Enter Username: ",
            //         TextStyle {
            //             font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            //             font_size: 40.0,
            //             color: Color::WHITE,
            //         },
            //     ).with_text_alignment(TextAlignment::Left))
            //     .insert(UsernameText);
            // });
        });
}

pub fn settings_menu_setup(mut commands: Commands) {
    let button_node = Node {
        width: Val::Px(200.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text_style = (
        TextFont {
            font_size: 33.0,
            ..default()
        },
        TextColor(TEXT_COLOR),
    );

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            OnSettingsMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(CRIMSON.into()),
                ))
                .with_children(|parent| {
                    for (action, text) in [
                        (MenuButtonAction::SettingsDisplay, "Display"),
                        (MenuButtonAction::SettingsSound, "Sound"),
                        (MenuButtonAction::BackToMainMenu, "Back"),
                    ] {
                        parent
                            .spawn((
                                Button,
                                button_node.clone(),
                                BackgroundColor(NORMAL_BUTTON),
                                action,
                            ))
                            .with_children(|parent| {
                                parent.spawn((Text::new(text), button_text_style.clone()));
                            });
                    }
                });
        });
}

pub fn display_settings_menu_setup(mut commands: Commands, display_quality: Res<DisplayQuality>) {
    let button_node = Node {
        width: Val::Px(200.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_text_style = (
        TextFont {
            font_size: 33.0,
            ..default()
        },
        TextColor(TEXT_COLOR),
    );

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            OnDisplaySettingsMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(CRIMSON.into()),
                ))
                .with_children(|parent| {
                    parent
                        .spawn((
                            Node {
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(CRIMSON.into()),
                        ))
                        .with_children(|parent| {
                            // Display a label for the current setting
                            parent.spawn((Text::new("Display Quality"), button_text_style.clone()));
                            // Display a button for each possible value
                            for quality_setting in [
                                DisplayQuality::Low,
                                DisplayQuality::Medium,
                                DisplayQuality::High,
                            ] {
                                let mut entity = parent.spawn((
                                    Button,
                                    Node {
                                        width: Val::Px(150.0),
                                        height: Val::Px(65.0),
                                        ..button_node.clone()
                                    },
                                    BackgroundColor(NORMAL_BUTTON),
                                    quality_setting,
                                ));
                                entity.with_children(|parent| {
                                    parent.spawn((
                                        Text::new(format!("{quality_setting:?}")),
                                        button_text_style.clone(),
                                    ));
                                });
                                if *display_quality == quality_setting {
                                    entity.insert(SelectedOption);
                                }
                            }
                        });
                    // Display the back button to return to the settings screen
                    parent
                        .spawn((
                            Button,
                            button_node,
                            BackgroundColor(NORMAL_BUTTON),
                            MenuButtonAction::BackToSettings,
                        ))
                        .with_children(|parent| {
                            parent.spawn((Text::new("Back"), button_text_style));
                        });
                });
        });
}

pub fn sound_settings_menu_setup(mut commands: Commands, volume: Res<Volume>) {
    let button_node = Node {
        width: Val::Px(200.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_text_style = (
        TextFont {
            font_size: 33.0,
            ..default()
        },
        TextColor(TEXT_COLOR),
    );

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            OnSoundSettingsMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(CRIMSON.into()),
                ))
                .with_children(|parent| {
                    parent
                        .spawn((
                            Node {
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(CRIMSON.into()),
                        ))
                        .with_children(|parent| {
                            parent.spawn((Text::new("Volume"), button_text_style.clone()));
                            for volume_setting in [0, 1, 2, 3, 4, 5, 6, 7, 8, 9] {
                                let mut entity = parent.spawn((
                                    Button,
                                    Node {
                                        width: Val::Px(30.0),
                                        height: Val::Px(65.0),
                                        ..button_node.clone()
                                    },
                                    BackgroundColor(NORMAL_BUTTON),
                                    Volume(volume_setting),
                                ));
                                if *volume == Volume(volume_setting) {
                                    entity.insert(SelectedOption);
                                }
                            }
                        });
                    parent
                        .spawn((
                            Button,
                            button_node,
                            BackgroundColor(NORMAL_BUTTON),
                            MenuButtonAction::BackToSettings,
                        ))
                        .with_child((Text::new("Back"), button_text_style));
                });
        });
}

pub fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Quit => {
                    app_exit_events.send(AppExit::Success);
                }
                MenuButtonAction::Play => {
                    game_state.set(GameState::Game);
                    menu_state.set(MenuState::Disabled);
                }
                MenuButtonAction::Settings => menu_state.set(MenuState::Settings),
                MenuButtonAction::SettingsDisplay => {
                    menu_state.set(MenuState::SettingsDisplay);
                }
                MenuButtonAction::SettingsSound => {
                    menu_state.set(MenuState::SettingsSound);
                }
                MenuButtonAction::BackToMainMenu => menu_state.set(MenuState::Main),
                MenuButtonAction::BackToSettings => {
                    menu_state.set(MenuState::Settings);
                }
            }
        }
    }
}

pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

// KEYBOARD????

// #[derive(Resource, Default)]
// struct Username {
//     current: String,
// }

// // Marker component for the UI text we want to update
// #[derive(Component)]
// struct UsernameText;

// fn handle_keyboard_input(
//     mut char_evr: EventReader<ReceivedCharacter>,
//     keys: Res<Input<KeyCode>>,
//     mut username: ResMut<Username>,
// ) {
//     for ev in char_evr.iter() {
//         if !ev.char.is_control() {
//             username.current.push(ev.char);
//         }
//     }

//     // Handle backspace
//     if keys.just_pressed(KeyCode::Back) {
//         username.current.pop();
//     }
// }

// fn update_text_display(
//     username: Res<Username>,
//     mut query: Query<&mut Text, With<UsernameText>>,
// ) {
//     if username.is_changed() {
//         let mut text = query.single_mut();
//         text.sections[0].value = format!("Enter Username: {}", username.current);
//     }
// }
