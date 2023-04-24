mod game_over_menu;
mod hud;
mod pause_menu;

use bevy::prelude::*;

use game_over_menu::GameOverMenuPlugin;
use hud::HudPlugin;
use pause_menu::PauseMenuPlugin;

pub const NORMAL_BUTTON_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON_COLOR: Color = Color::rgb(0.35, 0.75, 0.35);

pub const BUTTON_STYLE: Style = Style {
    align_items: AlignItems::Center,
    justify_content: JustifyContent::Center,
    size: Size::new(Val::Px(200.0), Val::Px(80.0)),
    ..Style::DEFAULT
};

pub const IMAGE_STYLE: Style = Style {
    size: Size::new(Val::Px(64.0), Val::Px(64.0)),
    margin: UiRect::all(Val::Px(8.0)),
    ..Style::DEFAULT
};

pub const TITLE_STYLE: Style = Style {
    align_items: AlignItems::Center,
    flex_direction:FlexDirection::Row,
    justify_content: JustifyContent::Center,
    size: Size::new(Val::Px(300.0), Val::Px(120.0)),
    ..Style::DEFAULT
};

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugin(HudPlugin)
        .add_plugin(PauseMenuPlugin)
        .add_plugin(GameOverMenuPlugin);
    }
}