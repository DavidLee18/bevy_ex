use bevy::{prelude::*, app::AppExit};

use crate::{game::{ui::{PRESSED_BUTTON_COLOR, HOVERED_BUTTON_COLOR, NORMAL_BUTTON_COLOR, pause_menu::components::*}, SimulationState}, AppState};

pub fn interact_with_resume_button(mut button_query: Query<
    (&Interaction, &mut BackgroundColor),
    (Changed<Interaction>, With<ResumeButton>)
    >,
    mut next_state: ResMut<NextState<SimulationState>>
) {
    if let Ok((interaction, mut bg_color)) = button_query.get_single_mut() {
        on_click(interaction, &mut bg_color, &mut next_state, SimulationState::Running);
    }
}

pub fn interact_with_main_menu_button(mut button_query: Query<
    (&Interaction, &mut BackgroundColor),
    (Changed<Interaction>, With<MainMenuButton>)
    >,
    mut next_state: ResMut<NextState<AppState>>
) {
    if let Ok((interaction, mut bg_color)) = button_query.get_single_mut() {
        on_click(interaction, &mut bg_color, &mut next_state, AppState::MainMenu);
    }
}

pub fn interact_with_quit_button(mut button_query: Query<
    (&Interaction, &mut BackgroundColor),
    (Changed<Interaction>, With<QuitButton>)
    >,
    mut quit_event: EventWriter<AppExit>
) {
    if let Ok((interaction, mut bg_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Clicked => {
                *bg_color = PRESSED_BUTTON_COLOR.into();
                quit_event.send(AppExit);
            }
            Interaction::Hovered => {
                *bg_color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *bg_color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}

fn on_click<T: States>(interaction: &Interaction, background_color: &mut BackgroundColor, next_state: &mut ResMut<NextState<T>>, state: T) {
        match *interaction {
            Interaction::Clicked => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                next_state.set(state);
            }
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *background_color = NORMAL_BUTTON_COLOR.into();
            }
        }
}