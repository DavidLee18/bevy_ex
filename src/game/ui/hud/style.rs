use bevy::prelude::*;

pub const HUD_STYLE: Style = Style {
    align_items: AlignItems::Center,
    flex_direction: FlexDirection::Row,
    gap: Size::new(Val::Percent(80.0), Val::Px(0.0)),
    justify_content: JustifyContent::Center,
    margin: UiRect { bottom: Val::Percent(90.0), ..UiRect::DEFAULT },
    size: Size::new(Val::Percent(100.0), Val::Percent(10.0)),
    ..Style::DEFAULT
};

pub const STAR_BOARD_STYLE: Style = Style {
    align_items: AlignItems::Center,
    align_content: AlignContent::Center,
    justify_content: JustifyContent::Center,
    position: UiRect::all(Val::Px(0.0)),
    size: Size::all(Val::Percent(6.0)),
    ..Style::DEFAULT
};