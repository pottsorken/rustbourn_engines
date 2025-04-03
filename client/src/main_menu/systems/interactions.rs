use bevy::prelude::*;
use bevy::ui::Interaction;
use crate::main_menu::components::*;
use crate::main_menu::styles::*;


pub fn handle_button_interactions(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<PlayButton>)>,
) {
    for (interaction, mut color) in &mut interaction_query {
        *color = match *interaction {
            Interaction::Pressed => PRESSED_BUTTON.into(),
            Interaction::Hovered => HOVERED_BUTTON.into(),
            Interaction::None => NORMAL_BUTTON.into(),
        };
    }
}
