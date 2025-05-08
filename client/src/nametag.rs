use bevy::{
    color::palettes::css::*,
    math::ops,
    prelude::*,
    sprite::Anchor,
    text::{FontSmoothing, LineBreak, TextBounds},
};

use crate::common::*;

#[derive(Component)]
struct AnimateScale;

pub fn spawn_tags(mut commands: Commands) {
    //commands.spawn((
    //    Text2d::new("Lorem Ipsum Dolor"),
    //    TextFont {
    //        font_size: 50.0,
    //        ..default()
    //    },
    //    TextLayout::new_with_justify(JustifyText::Center),
    //    Transform::from_xyz(1.0, 2.0, 3.0),
    //    //Anim
    //    //PlayerAttach {
    //    //    offset: Vec2::new(10., 10.),
    //    //},
    //    //TEXT_JUSTIFICATION.clone(),
    //));

    //commands.spawn((
    //    Text2d::new("Lorem Ipsum Dolor"),
    //    TextFont {
    //        font_size: 10.0,
    //        font_color: TextColor::BLACK,
    //        ..default()
    //    },
    //    TextLayout::new_with_justify(JustifyText::Center),
    //    Transform::from_xyz(50.0, 50.0, 3.0),
    //    PlayerAttach {
    //        offset: Vec2::new(0., -40.),
    //    },
    //));

    // Demonstrate text wrapping
    let slightly_smaller_text_font = TextFont {
        font_size: 35.0,
        ..default()
    };
    let box_size = Vec2::new(200.0, 100.0);
    let box_position = Vec2::new(0.0, -250.0);
    commands
        .spawn((
            Sprite::from_color(Color::srgb(0.25, 0.25, 0.55), box_size),
            Transform::from_translation(box_position.extend(0.0)),
            PlayerAttach {
                offset: Vec2::new(0., -40.),
            },
        ))
        .with_children(|builder| {
            builder.spawn((
                Text2d::new("this text wraps in the box\n(Unicode linebreaks)"),
                TextFont {
                    font_size: 10.0,
                    //font_color: TextColor::BLACK,
                    ..default()
                },
                TextLayout::new(JustifyText::Left, LineBreak::WordBoundary),
                // Wrap text in the rectangle
                TextBounds::from(box_size),
                // Ensure the text is drawn on top of the box
                Transform::from_translation(Vec3::Z),
            ));
        });
}
