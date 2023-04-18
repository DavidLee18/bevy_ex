use bevy::prelude::*;

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

pub const MAIN_MENU_STYLE: Style = Style {
    align_items: AlignItems::Center,
    flex_direction: FlexDirection::Column,
    gap: Size::new(Val::Px(8.0), Val::Px(8.0)),
    justify_content: JustifyContent::Center,
    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
    ..Style::DEFAULT
};

pub fn get_text_bundle(asset_server: &Res<AssetServer>, text: &str, font_size: f32) -> TextBundle {
    TextBundle {
        text: Text {
            sections: vec![ TextSection::new(text, TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size,
                color: Color::WHITE,
            }) ],
            alignment: TextAlignment::Center,
            ..Default::default()
        },
        ..Default::default()
    }
}
