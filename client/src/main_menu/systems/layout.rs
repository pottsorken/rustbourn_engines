use bevy::prelude::*;
use bevy::ui::*;
use bevy::text::*;

use crate::main_menu::components::*;
use crate::main_menu::styles::*;

#[derive(Component)]
pub struct MainMenuRoot;

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let _main_menu_entity = build_main_menu(&mut commands, &asset_server);
}

pub fn build_main_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    // Setup button style
    let button_style = Node {
        width: Val::Px(200.0),
        height: Val::Px(80.0),
        margin: UiRect::all(Val::Px(10.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    // Setup button text style
    let button_text_style = TextFont {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 32.0,
        font_smoothing: FontSmoothing::None,
    };

    commands
        .spawn((
            Node::default(),
            MainMenuRoot,
        ))
        .insert(Node {
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        })
        .insert(BackgroundColor(Color::srgb(0.1, 0.1, 0.1)))
        .with_children(|parent| {
            parent
                .spawn((
                    Button::default(),
                    PlayButton,
                ))
                .insert(button_style.clone())
                .insert(button_text_style.clone())
                .insert(BackgroundColor(NORMAL_BUTTON.into()))
                .with_children(|parent| {
                    parent.spawn(Text::new(
                        "Block",
                    ));
                });
        })
        .id()
}

pub fn despawn_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenuRoot>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
