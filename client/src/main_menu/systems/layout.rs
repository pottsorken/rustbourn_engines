use bevy::{
    prelude::*,
    text::*,
    color::palettes::basic::*,
    winit::WinitSettings,
};

use crate::Val::*;
use crate::main_menu::components::*;
use crate::main_menu::styles::*;

#[derive(Component)]
pub struct MainMenuRoot;

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let _main_menu_entity = build_main_menu(&mut commands, &asset_server);
}

pub fn build_main_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {

    // Root UI node with horizontal layout
    commands
        .spawn((
            Node {
                right: Val::Percent(-90.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Row,
                ..default()
            },
            MainMenuRoot,
        ))
        .with_children(|parent| {
            // Left-side vertical button panel
            parent
                .spawn(Node {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(20.0)),
                        ..default()
                })
                .with_children(|panel| {
                    for label in &["Block 1", "Block 2", "Block 3", "Block 4"] {
                        panel
                            .spawn((
                                Button,
                                Node {
                                    width: Val::Px(50.0),
                                    height: Val::Px(50.0),
                                    margin: UiRect::all(Val::Px(10.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                BorderColor(Color::BLACK),
                                BackgroundColor(NORMAL_BUTTON),
                                PlayButton, // You can change this per label if needed
                            ))
                            .with_children(|button| {
                                button.spawn((
                                    Text::new(*label),
                                    TextFont {
                                        font_size: 10.0,
                                        ..Default::default()
                                    },
                                    TextColor {
                                        0: Color::WHITE,
                                    },
                                ));
                            });
                    }
                });
        })
        .id()
}

pub fn despawn_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenuRoot>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
