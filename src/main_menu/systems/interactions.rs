use bevy::{prelude::*, app::AppExit};

use crate::{
    main_menu::{
        components::*,
        styles::{
            PRESSED_BUTTON_COLOR,
            HOVERED_BUTTON_COLOR,
            NORMAL_BUTTON_COLOR
        }
    }, AppState
};

pub fn interact_with_play_button(mut button_query: Query<
    (&Interaction, &mut BackgroundColor),
    (Changed<Interaction>, With<PlayButton>)
    >,
    mut next_state: ResMut<NextState<AppState>>
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Clicked => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                next_state.set(AppState::Game);
            }
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *background_color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}

pub fn interact_with_quit_button(mut button_query: Query<
    (&Interaction, &mut BackgroundColor),
    (Changed<Interaction>, With<QuitButton>)
    >,
    mut exit_event: EventWriter<AppExit>
) {
        if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
            match *interaction {
                Interaction::Clicked => {
                    *background_color = PRESSED_BUTTON_COLOR.into();
                    exit_event.send(AppExit);
                }
                Interaction::Hovered => {
                    *background_color = HOVERED_BUTTON_COLOR.into();
                }
                Interaction::None => {
                    *background_color = NORMAL_BUTTON_COLOR.into();
                }
            }
        }
    }